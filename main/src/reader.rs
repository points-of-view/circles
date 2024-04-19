pub mod command;
use crate::tags::{Tag, TagsMap};
use std::{
    fmt::{Display, Formatter},
    net,
    time::{Duration, Instant},
};
use tauri::{
    api::process::CommandEvent,
    async_runtime::{spawn, JoinHandle, Receiver},
    AppHandle, Manager,
};

const REFRESH_INTERVAL: u64 = 500;

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ReaderErrorKind {
    IncorrectHostname(String),
    CouldNotConnect(String),
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ReaderError {
    pub kind: ReaderErrorKind,
    message: String,
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.kind {
            ReaderErrorKind::IncorrectHostname(hostname) => write!(
                f,
                "Hostname {} is incorrect. Original error: {}",
                hostname,
                self.message
            ),
            ReaderErrorKind::CouldNotConnect(hostname) => write!(
                f,
                "Could not connect to hostname {}. Original error: {}",
                hostname,
                self.message
            ),
            ReaderErrorKind::Unknown => write!(
                f,
                "Encountered an unexpected error in the reader. Message: {}",
                self.message
            ),
        }
    }
}

enum ReaderConnectionType {
    Hostname,
    LinkLocal,
}

pub struct Reader {
    hostname: String,
    connection_type: ReaderConnectionType,
}

impl Reader {
    pub fn new(hostname: String) -> Result<Self, ReaderError> {
        if hostname.len() != 12 {
            return Err(ReaderError { kind: ReaderErrorKind::IncorrectHostname(hostname.clone()), message: format!("This should be exactly 12 characters, but was {}", hostname.len()) })
        }
 
        let mut reader = Reader { 
            hostname,
            connection_type: ReaderConnectionType::Hostname,
        };
        reader.try_connect()?;
        Ok(reader)
    }

    /// Try to connect to the reader.
    /// 
    /// We first try the hostname and check if we can connect that way.
    /// If the hostname is unavailable, we fall back on the LinkLocal ipv4.
    fn try_connect(&mut self) -> Result<(), ReaderError> {
        if let Ok(_) = net::TcpStream::connect(&self.hostname) {
            return Ok(());
        }

        self.connection_type = ReaderConnectionType::LinkLocal;
        let socket = net::SocketAddr::new(self.hostname_as_ip()?, 5084);

        match net::TcpStream::connect_timeout(&socket, Duration::from_secs(2)) {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(ReaderError {
                    kind: ReaderErrorKind::CouldNotConnect(self.hostname.clone()),
                    message: err.to_string(),
                })
            }
        }
    }
    

    /// Convert the hostname to a LinkLocal IPv4 address.
    ///
    /// If the hostname is not available, the reader will try to assign itself an IP address.
    /// This IP can be calculated by converting the last two chunks of the mac address from hexadecimal to decimal
    fn hostname_as_ip(&self) -> Result<net::IpAddr, ReaderError> {
        let Ok(third_element) = u8::from_str_radix(&self.hostname[8..10], 16) else {
            return Err(ReaderError {
                kind: ReaderErrorKind::IncorrectHostname(self.hostname.clone()),
                message: String::from("Could not convert mac address to IP")
            })
        };
        let Ok(fourth_element) = u8::from_str_radix(&self.hostname[10..12], 16) else {
            return Err(ReaderError {
                kind: ReaderErrorKind::IncorrectHostname(self.hostname.clone()),
                message: String::from("Could not convert mac address to IP")
            })
        };
        Ok(net::IpAddr::from([169u8, 254u8, third_element, fourth_element]))
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
            match handle_reader_event(event, &mut tags) {
                Ok(()) => (),
                Err(err) => handle.emit_all("reader-error", err).unwrap(),
            }
            if last_update.elapsed() > interval {
                let new_map = TagsMap::from(tags.drain(..));
                handle.emit_all("updated-tags", new_map).unwrap();
                last_update = Instant::now();
            }
        }
    })
}

fn handle_reader_event(event: CommandEvent, tags: &mut Vec<Tag>) -> Result<(), ReaderError> {
    match event {
        CommandEvent::Stdout(line) => handle_reader_stdout(line, tags),
        CommandEvent::Stderr(error) => handle_reader_error(error)?,
        CommandEvent::Error(error) => handle_reader_error(error)?,
        CommandEvent::Terminated(payload) => {
            todo!("Reader was terminated with payload {:?}", payload)
        }
        // NOTE: We don't expect any other CommandEvents to occur. For now, we'll just panic and print them
        event => todo!("An unexpected CommandEvent occured: {:#?}", event),
    }
    Ok(())
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

fn handle_reader_error(error: String) -> Result<(), ReaderError> {
    let err = ReaderError {
        kind: ReaderErrorKind::Unknown,
        message: error,
    };

    // While developing, we print the error to stderr so it is easier to identify
    #[cfg(debug_assertions)]
    eprintln!("{:?}", &err);

    Err(err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags::create_mock_tag;

    #[test]
    fn should_add_correct_tag_to_vector() {
        let mut vec: Vec<Tag> = vec![];

        let event = CommandEvent::Stdout(create_mock_tag());
        let result = handle_reader_event(event, &mut vec);

        assert!(result.is_ok());
        assert_eq!(1, vec.len());
    }

    #[test]
    fn should_ignore_incorrect_tags() {
        let mut vec: Vec<Tag> = vec![];

        let event = CommandEvent::Stdout(String::from("incorrect tag"));
        let result = handle_reader_event(event, &mut vec);

        assert!(result.is_ok());
        assert_eq!(0, vec.len());
    }

    #[test]
    fn should_turn_unexpected_error_into_unknown() {
        let message = "Some weird unhandled error! Scary stuff";
        let mut vec: Vec<Tag> = vec![];

        let event = CommandEvent::Stderr(String::from(message));
        let result = handle_reader_event(event, &mut vec);

        assert!(result.is_err_and(|x| x.kind == ReaderErrorKind::Unknown));

        let event = CommandEvent::Error(String::from(message));
        let result = handle_reader_event(event, &mut vec);

        assert!(result.is_err_and(|x| x.kind == ReaderErrorKind::Unknown));
    }
}
