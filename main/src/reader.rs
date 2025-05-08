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

use crate::{error::CirclesError, GlobalState};

use self::messages::handle_new_message;

const DEFAULT_ROSPEC_ID: u32 = 1234;
const REFRESH_INTERVAL: u32 = 125;
const RECV_TIMEOUT: Duration = Duration::from_millis(100);

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
    fn new<R: tauri::Runtime>(
        hostname: String,
        app_handle: AppHandle<R>,
    ) -> Result<Self, ReaderError>
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
        let tags_map = app_handle.state::<GlobalState>().tags_map.clone();
        let mut last_update = Instant::now();
        let mut last_alive = Instant::now();
        let update_interval = Duration::from_millis(REFRESH_INTERVAL.into());
        let alive_interval = Duration::from_millis((REFRESH_INTERVAL * 10).into());

        // Since reading from a TcpStream is blocking, we do this in a subthread.
        // The messages get send to this thread, so we loop regardless of new messages.
        receive_messages(stream.try_clone().unwrap(), tx);
        loop {
            if let Ok(message) = rx.recv_timeout(RECV_TIMEOUT) {
                let tags = handle_new_message(message, &stream);
                tags_map.lock().unwrap().add_tags(tags);
                last_alive = Instant::now();
            }

            if last_update.elapsed() > update_interval {
                // Update the frontend with a new map.
                // We clone the map inside our mutex, since we don't care about any changes while we are sending this event
                app_handle
                    .emit_all("updated-tags", tags_map.lock().unwrap().clone())
                    .unwrap();
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
