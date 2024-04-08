pub mod command;
use crate::tags::Tag;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use std::vec::Drain;
use std::{
    fmt::{Display, Formatter},
    time::Duration,
};
use tauri::{
    api::process::CommandEvent,
    async_runtime::{spawn, JoinHandle, Receiver, Sender},
};

const REFRESH_INTERVAL: u64 = 500;

pub type TagsMap = HashMap<String, Tag>;

#[derive(Debug, PartialEq)]
pub enum ReaderErrorKind {
    Unknown,
}

#[derive(Debug)]
pub struct ReaderError {
    pub kind: ReaderErrorKind,
    message: String,
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.kind {
            ReaderErrorKind::Unknown => write!(
                f,
                "Encountered an unexpected error in the reader. Message: {}",
                self.message
            ),
        }
    }
}

pub fn handle_reader_events(
    mut rx: Receiver<CommandEvent>,
    sender: Arc<tauri::async_runtime::Mutex<Sender<TagsMap>>>,
) -> JoinHandle<()> {
    spawn(async move {
        let mut tags: Vec<Tag> = vec![];
        let interval = Duration::from_millis(REFRESH_INTERVAL);
        let mut last_update = Instant::now();
        while let Some(event) = rx.recv().await {
            handle_reader_event(event, &mut tags);
            if last_update.elapsed() > interval {
                let new_map = reduce_tags_to_map(tags.drain(..));
                let lock = sender.lock().await;
                lock.send(new_map).await.unwrap();
                last_update = Instant::now();
            }
        }
    })
}

fn handle_reader_event(event: CommandEvent, tags: &mut Vec<Tag>) {
    match event {
        CommandEvent::Stdout(line) => handle_reader_stdout(line, tags),
        CommandEvent::Stderr(error) => handle_reader_error(error),
        CommandEvent::Error(error) => handle_reader_error(error),
        CommandEvent::Terminated(payload) => {
            todo!("Reader was terminated with payload {:?}", payload)
        }
        // NOTE: We don't expect any other CommandEvents to occur. For now, we'll just panic and print them
        event => todo!("An unexpected CommandEvent occured: {:#?}", event),
    }
}

fn handle_reader_stdout(line: String, tags: &mut Vec<Tag>) {
    match Tag::from_reader(line) {
        Ok(tag) => tags.push(tag),
        Err(err) => {
            // We print faulty tags in development (so we can learn from them)
            // In production these get ignored
            #[cfg(debug_assertions)]
            println!("{:?}", err)
        }
    }
}

fn handle_reader_error(error: String) {
    let err = ReaderError {
        kind: ReaderErrorKind::Unknown,
        message: error,
    };
    eprintln!("{:?}", err)
}

fn reduce_tags_to_map(tags: Drain<'_, Tag>) -> TagsMap {
    tags.fold(TagsMap::new(), |mut acc, new_tag| {
        acc.entry(new_tag.id.clone())
            .and_modify(|old_tag| {
                // If there is a current tag, we update this if the new one has a stronger signal
                if old_tag.strength < new_tag.strength.clone() {
                    *old_tag = new_tag.clone();
                }
            })
            // If there isn't a new tag, we insert it
            .or_insert(new_tag.clone());

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_resolve_a_simple_vec() {
        let tag = Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -30,
        };
        let mut tags = vec![tag];

        let map = reduce_tags_to_map(tags.drain(..));

        assert_eq!(1, map.keys().len());
        assert_eq!(map.contains_key("abc123"), true);
        assert_eq!(map["abc123"].antenna, 1);
    }

    #[test]
    fn should_keep_strongest_of_two_tokens() {
        let tag1 = Tag {
            id: String::from("abc123"),
            antenna: 2,
            strength: -35,
        };
        let tag2 = Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -30,
        };
        let tag3 = Tag {
            id: String::from("abc123"),
            antenna: 1,
            strength: -65,
        };
        let mut tags = vec![tag1, tag2, tag3];

        let map = reduce_tags_to_map(tags.drain(..));

        assert_eq!(1, map.keys().len());
        assert_eq!(map.contains_key("abc123"), true);
        assert_eq!(map["abc123"].antenna, 1);
        assert_eq!(map["abc123"].strength, -30);
    }
}
