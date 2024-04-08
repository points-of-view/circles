pub mod command;
use crate::tags::{Tag, TagsMap};
use std::sync::Arc;
use std::time::Instant;
use std::{
    fmt::{Display, Formatter},
    time::Duration,
};
use tauri::{
    api::process::CommandEvent,
    async_runtime::{spawn, JoinHandle, Receiver, Sender},
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
                let new_map = TagsMap::from(tags.drain(..));
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
