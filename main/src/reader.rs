pub mod command;
use crate::tags::{Tag, TagsMap};
use std::time::Instant;
use std::{
    fmt::{Display, Formatter},
    time::Duration,
};
use tauri::{
    api::process::CommandEvent,
    async_runtime::{spawn, JoinHandle, Receiver},
    AppHandle, Manager,
};

const REFRESH_INTERVAL: u64 = 500;

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

pub fn handle_reader_events<R: tauri::Runtime>(
    mut rx: Receiver<CommandEvent>,
    handle: AppHandle<R>,
) -> JoinHandle<()> {
    spawn(async move {
        let mut tags: Vec<Tag> = vec![];
        let interval = Duration::from_millis(REFRESH_INTERVAL);
        let mut last_update = Instant::now();
        while let Some(event) = rx.recv().await {
            handle_reader_event(event, &mut tags);
            if last_update.elapsed() > interval {
                let new_map = TagsMap::from(tags.drain(..));
                handle.emit_all("updated-tags", new_map).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags::create_mock_tag;

    #[test]
    fn should_add_correct_tag_to_vector() {
        let mut vec: Vec<Tag> = vec![];

        let event = CommandEvent::Stdout(create_mock_tag());
        handle_reader_event(event, &mut vec);

        assert_eq!(1, vec.len());
    }

    #[test]
    fn should_ignore_incorrect_tags() {
        let mut vec: Vec<Tag> = vec![];

        let event = CommandEvent::Stdout(String::from("incorrect tag"));
        handle_reader_event(event, &mut vec);

        assert_eq!(0, vec.len());
    }
}
