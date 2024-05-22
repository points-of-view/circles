pub mod error;
mod llrp_reader;
pub mod messages;
mod mock_reader;
mod rospec;

pub use error::{ReaderError, ReaderErrorKind};
use llrp::messages::Message;
pub use llrp_reader::LLRPReader;
pub use mock_reader::MockReader;
use std::{
    net::TcpStream,
    sync::mpsc::{channel, Sender},
    time::{Duration, Instant},
};
use tauri::{AppHandle, Manager};

use crate::{
    error::CirclesError,
    tags::{Tag, TagsMap},
};

use self::messages::handle_new_message;

const DEFAULT_ROSPEC_ID: u32 = 1234;
const REFRESH_INTERVAL: u32 = 200;
const RECV_TIMEOUT: Duration = Duration::from_millis(100);
const KEEP_OLD_MAPS: usize = 15; // At a refresh interval of 200ms, this is 3 secs of data

#[derive(Debug)]
pub enum Reader {
    LLRPReader(LLRPReader),
    MockReader(MockReader),
}

impl Reader {
    pub fn start_reading<R: tauri::Runtime>(
        &mut self,
        app_handle: AppHandle<R>,
    ) -> Result<(), ReaderError> {
        match self {
            Reader::LLRPReader(reader) => reader.start_reading(app_handle),
            Reader::MockReader(reader) => reader.start_reading(app_handle),
        }
    }

    pub fn stop_reading(&mut self, await_confirmation: bool) -> Result<(), ReaderError> {
        match self {
            Reader::LLRPReader(reader) => reader.stop_reading(await_confirmation),
            Reader::MockReader(reader) => reader.stop_reading(await_confirmation),
        }
    }
}

pub trait ReaderProtocol {
    fn new(hostname: String) -> Result<Self, ReaderError>
    where
        Self: Sized;
    fn start_reading<R: tauri::Runtime>(
        &mut self,
        app_handle: AppHandle<R>,
    ) -> Result<(), ReaderError>;
    fn stop_reading(&mut self, await_confirmation: bool) -> Result<(), ReaderError>;
}

pub fn handle_reader_input<R: tauri::Runtime>(
    stream: TcpStream,
    app_handle: AppHandle<R>,
) -> tauri::async_runtime::JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let (tx, rx) = channel::<Message>();
        let mut tags: Vec<Tag> = vec![];
        let mut previous_maps: Vec<TagsMap> = Vec::with_capacity(10);
        let mut last_update = Instant::now();
        let mut last_alive = Instant::now();
        let update_interval = Duration::from_millis(REFRESH_INTERVAL.into());
        let alive_interval = Duration::from_millis((REFRESH_INTERVAL * 10).into());

        // Since reading from a TcpStream is blocking, we do this in a subthread.
        // The messages get send to this thread, so we loop regardless of new messages.
        receive_messages(stream.try_clone().unwrap(), tx);
        loop {
            if let Ok(message) = rx.recv_timeout(RECV_TIMEOUT) {
                handle_new_message(message, &mut tags, &stream);
                last_alive = Instant::now();
            }

            if last_update.elapsed() > update_interval {
                // Create a map from the tags we just saw
                let new_map = TagsMap::from(tags.drain(..));

                // Remove first item (if needed) and push new map
                if previous_maps.len() >= KEEP_OLD_MAPS {
                    previous_maps.drain(0..1);
                }
                previous_maps.push(new_map);

                // Create a map from the collection of maps
                let total_map = TagsMap::from(&previous_maps);
                app_handle.emit_all("updated-tags", total_map).unwrap();
                last_update = Instant::now();
            }

            if last_alive.elapsed() > alive_interval {
                // If we are not alive for our interval, we assume the connection has failed
                // In that case we break from our loop, so an error is sent.
                send_error_to_frontend(
                    app_handle,
                    ReaderError {
                        kind: ReaderErrorKind::LostConnection,
                        message: String::from(""),
                    }
                    .into(),
                );
                break;
            }
        }
    })
}

fn send_error_to_frontend<R: tauri::Runtime>(app_handle: AppHandle<R>, error: CirclesError) {
    app_handle
        .emit_all("error", error)
        .expect("Should be able to emit to app_handle");
}

fn receive_messages(stream: TcpStream, sender: Sender<Message>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        while let Ok(message) = llrp::read_message(&stream) {
            let res = match message.to_dynamic_message() {
                Ok(m) => sender.send(m),
                Err(err) => Ok({
                    #[cfg(debug_assertions)]
                    println!("Could not decode message. {}", err)
                }),
            };
            // If sending fails, this probably means that our parent-thread has quit as well.
            // In that case we just stop here and let this thread finish.
            if res.is_err() {
                break;
            }
        }
    })
}
