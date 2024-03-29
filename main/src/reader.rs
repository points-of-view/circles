pub mod command;
use tauri::api::process::CommandEvent;

use crate::tags::Tag;

pub fn handle_reader_event(event: CommandEvent) {
    match event {
        CommandEvent::Stderr(_) => todo!(),
        CommandEvent::Stdout(line) => handle_reader_stdout(line),
        CommandEvent::Error(_) => todo!(),
        CommandEvent::Terminated(_) => todo!(),
        _ => todo!(),
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
