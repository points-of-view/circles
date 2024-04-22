pub mod error;
mod llrp_reader;
pub mod messages;
mod rospec;

pub use error::{ReaderError, ReaderErrorKind};
use llrp::messages::Message;
pub use llrp_reader::LLRPReader;
use std::{
    net::TcpStream,
    sync::mpsc::{channel, Sender},
    time::{Duration, Instant},
};
use tauri::{AppHandle, Manager};

use crate::tags::{Tag, TagsMap};

use self::messages::handle_new_message;

const DEFAULT_ROSPEC_ID: u32 = 1234;
const REFRESH_INTERVAL: u32 = 500;
const RECV_TIMEOUT: Duration = Duration::from_millis(100);

pub fn handle_reader_input<R: tauri::Runtime>(
    stream: TcpStream,
    app_handle: AppHandle<R>,
) -> tauri::async_runtime::JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let (tx, rx) = channel::<Message>();
        let mut tags: Vec<Tag> = vec![];
        let mut last_update = Instant::now();
        let mut last_alive = Instant::now();
        let update_interval = Duration::from_millis(REFRESH_INTERVAL.into());
        let alive_interval = Duration::from_millis((REFRESH_INTERVAL * 4).into());

        // Since reading from a TcpStream is blocking, we do this in a subthread.
        // The messages get send to this thread, so we loop regardless of new messages.
        receive_messages(stream.try_clone().unwrap(), tx);
        loop {
            if let Ok(message) = rx.recv_timeout(RECV_TIMEOUT) {
                handle_new_message(message, &mut tags, &stream);
                last_alive = Instant::now();
            }

            if last_update.elapsed() > update_interval {
                let new_map = TagsMap::from(tags.drain(..));
                app_handle.emit_all("updated-tags", new_map).unwrap();
                last_update = Instant::now();
            }

            if last_alive.elapsed() > alive_interval {
                // If we are not alive for our interval, we assume the connection has failed
                // In that case we break from our loop, so an error is sent.
                app_handle
                    .emit_all(
                        "reader-error",
                        ReaderError {
                            kind: ReaderErrorKind::LostConnection,
                            message: String::from(""),
                        },
                    )
                    .unwrap();
                break;
            }
        }
    })
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
