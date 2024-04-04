pub mod command;
use crate::tags::Tag;
use std::fmt::{Display, Formatter};
use tauri::api::process::CommandEvent;

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

pub fn handle_reader_event(event: CommandEvent) {
    match event {
        CommandEvent::Stdout(line) => handle_reader_stdout(line),
        CommandEvent::Stderr(error) => handle_reader_error(error),
        CommandEvent::Error(error) => handle_reader_error(error),
        CommandEvent::Terminated(payload) => {
            todo!("Reader was terminated with payload {:?}", payload)
        }
        // NOTE: We don't expect any other CommandEvents to occur. For now, we'll just panic and print them
        event => todo!("An unexpected CommandEvent occured: {:#?}", event),
    }
}

fn handle_reader_stdout(line: String) {
    match Tag::from_reader(line) {
        Ok(tag) => println!("{:?}", tag),
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
