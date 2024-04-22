pub mod error;
mod llrp_reader;
pub mod messages;
mod rospec;

pub use error::{ReaderError, ReaderErrorKind};
pub use llrp_reader::LLRPReader;
use std::{
    net::TcpStream,
    time::{Duration, Instant},
};
use tauri::{
    async_runtime::{spawn, JoinHandle},
    AppHandle, Manager,
};

use crate::tags::{Tag, TagsMap};

use self::messages::handle_new_message;

const DEFAULT_ROSPEC_ID: u32 = 1234;
const REFRESH_INTERVAL: u32 = 500;

pub fn handle_reader_input<R: tauri::Runtime>(
    stream: TcpStream,
    app_handle: AppHandle<R>,
) -> JoinHandle<()> {
    spawn(async move {
        let mut tags: Vec<Tag> = vec![];
        let interval = Duration::from_millis(REFRESH_INTERVAL.into());
        let mut last_update = Instant::now();
        while let Ok(message) = llrp::read_message(&stream) {
            match message.to_dynamic_message() {
                Ok(m) => handle_new_message(m, &mut tags, &stream),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    println!("Could not decode message. {}", err)
                }
            }

            if last_update.elapsed() > interval {
                let new_map = TagsMap::from(tags.drain(..));
                app_handle.emit_all("updated-tags", new_map).unwrap();
                last_update = Instant::now();
            }
        }
    })
}
