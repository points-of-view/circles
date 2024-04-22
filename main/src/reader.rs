pub mod command;
use crate::tags::{Tag, TagsMap};
use llrp::{choices::*, enumerations::*, messages::*, parameters::*, BinaryMessage, LLRPMessage};
use std::{
    fmt::{Display, Formatter},
    net::{self, TcpStream},
    time::{Duration, Instant},
};
use tauri::{
    api::process::CommandEvent,
    async_runtime::{spawn, JoinHandle, Receiver},
    AppHandle, Manager,
};

const REFRESH_INTERVAL: u64 = 500;
const DEFAULT_PORT: u16 = 5084;
const DEFAULT_ROSPEC_ID: u32 = 1234;

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
                hostname, self.message
            ),
            ReaderErrorKind::CouldNotConnect(hostname) => write!(
                f,
                "Could not connect to hostname {}. Original error: {}",
                hostname, self.message
            ),
            ReaderErrorKind::Unknown => write!(
                f,
                "Encountered an unexpected error in the reader. Message: {}",
                self.message
            ),
        }
    }
}

#[derive(Debug)]
pub struct Reader {
    hostname: String,
    stream: Option<TcpStream>,
}

impl Reader {
    pub fn new(hostname: String) -> Result<Self, ReaderError> {
        if hostname.len() != 12 {
            return Err(ReaderError {
                kind: ReaderErrorKind::IncorrectHostname(hostname.clone()),
                message: format!(
                    "This should be exactly 12 characters, but was {}",
                    hostname.len()
                ),
            });
        }

        let mut reader = Reader {
            hostname,
            stream: None,
        };
        reader.connect()?;
        reader.prepare()?;
        Ok(reader)
    }

    /// Connect to the reader.
    ///
    /// We first try the hostname and check if we can connect that way.
    /// If the hostname is unavailable, we fall back on the LinkLocal ipv4.
    fn connect(&mut self) -> Result<(), ReaderError> {
        self.stream = match net::TcpStream::connect(format!("{}:{}", self.hostname, DEFAULT_PORT)) {
            Ok(stream) => Some(stream),
            Err(_) => None,
        };

        if self.stream.is_none() {
            self.stream = match net::TcpStream::connect_timeout(
                &net::SocketAddr::new(self.hostname_as_ip()?, DEFAULT_PORT),
                Duration::from_secs(5),
            ) {
                Ok(stream) => Some(stream),
                Err(err) => {
                    return Err(ReaderError {
                        kind: ReaderErrorKind::CouldNotConnect(self.hostname.clone()),
                        message: err.to_string(),
                    })
                }
            };
        }

        // Wait for the first ReaderEventNotification and confirm that we are connected
        self.await_message_and::<ReaderEventNotification>(|m: &ReaderEventNotification| {
            m.reader_event_notification_data.connection_attempt_event
                == Some(ConnectionAttemptEvent {
                    status: ConnectionAttemptStatusType::Success,
                })
        })?;
        Ok(())
    }

    /// Convert the hostname to a LinkLocal IPv4 address.
    ///
    /// If the hostname is not available, the reader will try to assign itself an IP address.
    /// This IP can be calculated by converting the last two chunks of the mac address from hexadecimal to decimal
    fn hostname_as_ip(&self) -> Result<net::IpAddr, ReaderError> {
        let Ok(third_element) = u8::from_str_radix(&self.hostname[8..10], 16) else {
            return Err(ReaderError {
                kind: ReaderErrorKind::IncorrectHostname(self.hostname.clone()),
                message: String::from("Could not convert mac address to IP"),
            });
        };
        let Ok(fourth_element) = u8::from_str_radix(&self.hostname[10..12], 16) else {
            return Err(ReaderError {
                kind: ReaderErrorKind::IncorrectHostname(self.hostname.clone()),
                message: String::from("Could not convert mac address to IP"),
            });
        };
        Ok(net::IpAddr::from([
            169u8,
            254u8,
            third_element,
            fourth_element,
        ]))
    }

    fn write_message(&self, message: Message) -> Result<(), ReaderError> {
        // NOTE: The message id can be a random u32 - it is returned in the appropriate response
        // But for now, we don't care about these responses
        // We can unwrap here, since `from_dynamic_message` doesn't ever fail
        println!("writing: {:?}", &message);
        let message = BinaryMessage::from_dynamic_message(20, &message).unwrap();
        match llrp::write_message(self.stream.as_ref().unwrap(), message) {
            Ok(_) => Ok(()),
            Err(err) => Err(ReaderError {
                kind: ReaderErrorKind::Unknown,
                message: format!("Error writing: {:?}", err.to_string()),
            }),
        }
    }

    fn parse_message_and<T: LLRPMessage>(
        &self,
        closure: fn(message: &T) -> bool,
    ) -> Result<T, ReaderError> {
        let binary_message = match llrp::read_message(self.stream.as_ref().unwrap()) {
            Ok(message) => message,
            Err(err) => {
                return Err(ReaderError {
                    kind: ReaderErrorKind::Unknown,
                    message: err.to_string(),
                })
            }
        };

        let message = match binary_message.to_message::<T>() {
            Ok(message) => message,
            Err(err) => {
                return Err(ReaderError {
                    kind: ReaderErrorKind::Unknown,
                    message: format!(
                        "Message was not of the expected kind, but was {:?}. Original error: {}",
                        binary_message.to_dynamic_message(),
                        err.to_string()
                    ),
                })
            }
        };

        match closure(&message) {
            true => Ok(message),
            false => Err(ReaderError {
                kind: ReaderErrorKind::Unknown,
                message: format!(
                    "Message did not pass closure check. Full message: {:?}",
                    binary_message.to_dynamic_message(),
                ),
            }),
        }
    }

    fn parse_message<T: LLRPMessage>(&self) -> Result<T, ReaderError> {
        self.parse_message_and(|_| true)
    }

    fn await_message_and<T: LLRPMessage>(
        &self,
        closure: fn(message: &T) -> bool,
    ) -> Result<T, ReaderError> {
        let message = loop {
            match self.parse_message::<T>() {
                Ok(message) => break message,
                Err(_) => (),
            }
        };

        match closure(&message) {
            true => Ok(message),
            false => Err(ReaderError {
                kind: ReaderErrorKind::Unknown,
                message: String::from("Message did not pass closure check."),
            }),
        }
    }

    fn await_message<T: LLRPMessage>(&self) -> Result<T, ReaderError> {
        self.await_message_and(|_| true)
    }

    fn prepare(&self) -> Result<(), ReaderError> {
        // Remove all existing ro_specs in the reader. ro_spec_id `0` means all ro_spec's should be deleted
        self.write_message(Message::DeleteRospec(DeleteRospec { ro_spec_id: 0 }))?;
        self.await_message_and::<DeleteRospecResponse>(|m: &DeleteRospecResponse| {
            m.status.status_code == StatusCode::M_Success
        })?;
        // Add our new ro_spec
        self.write_message(Message::AddRospec(AddRospec {
            ro_spec: construct_default_rospec(),
        }))?;
        self.await_message_and::<AddRospecResponse>(|m: &AddRospecResponse| {
            m.status.status_code == StatusCode::M_Success
        })?;
        // Enable our new ro_spec
        self.write_message(Message::EnableRospec(EnableRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        self.await_message_and::<EnableRospecResponse>(|m: &EnableRospecResponse| {
            m.status.status_code == StatusCode::M_Success
        })?;
        Ok(())
    }

    pub fn start_reading(&self) -> Result<(), ReaderError> {
        // Just in case we are already reading, we should try to stop
        self.stop_reading()?;

        // Actually start
        self.write_message(Message::StartRospec(StartRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        for _ in 0..10 {
            match llrp::read_message(self.stream.as_ref().unwrap()) {
                Ok(message) => {
                    println!("{:?}", message.to_dynamic_message().unwrap());
                }
                Err(err) => {
                    return Err(ReaderError {
                        kind: ReaderErrorKind::Unknown,
                        message: err.to_string(),
                    })
                }
            };
        }
        Ok(())
    }

    pub fn stop_reading(&self) -> Result<(), ReaderError> {
        self.write_message(Message::StopRospec(StopRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        let message = self.await_message::<StopRospecResponse>();
        println!("{:#?}", message);
        Ok(())
    }
}

fn construct_default_rospec() -> ROSpec {
    ROSpec {
        ro_spec_id: DEFAULT_ROSPEC_ID,
        priority: 0,
        current_state: ROSpecState::Disabled, // Setting this to `Inactive` or `Active` results in an error from our reader
        ro_boundary_spec: ROBoundarySpec {
            ro_spec_start_trigger: ROSpecStartTrigger {
                ro_spec_start_trigger_type: ROSpecStartTriggerType::Null,
                periodic_trigger_value: None,
                gpi_trigger_value: None,
            },
            ro_spec_stop_trigger: ROSpecStopTrigger {
                ro_spec_stop_trigger_type: ROSpecStopTriggerType::Null,
                // We have to pass a duration, but this value is ignored since out trigger type isn't `Duration`
                duration_trigger_value: 0,
                gpi_trigger_value: None,
            },
        },
        spec_parameter: vec![SpecParameter::AISpec(AISpec {
            antenna_ids: vec![1, 2, 3],
            ai_spec_stop_trigger: AISpecStopTrigger {
                ai_spec_stop_trigger_type: AISpecStopTriggerType::Null,
                // We have to pass a duration, but this value is ignored since out trigger type isn't `Duration`
                duration_trigger: 0,
                gpi_trigger_value: None,
                tag_observation_trigger: None,
            },
            inventory_parameter_spec: vec![InventoryParameterSpec {
                inventory_parameter_spec_id: 1,
                protocol_id: AirProtocols::EPCGlobalClass1Gen2,
                antenna_configuration: Vec::new(),
                custom: Vec::new(),
            }],
            custom: Vec::new(),
        })],
        ro_report_spec: Some(ROReportSpec {
            // NOTE: The spec defines trigger based on N amount of milliseconds, but our readers doesn't accept these
            ro_report_trigger:
                ROReportTriggerType::Upon_N_Tags_Or_End_Of_AISpec_Or_End_Of_RFSurveySpec,
            n: 1,
            tag_report_content_selector: TagReportContentSelector {
                enable_ro_spec_id: false,
                enable_spec_index: false,
                enable_inventory_parameter_spec_id: false,
                enable_antenna_id: true,
                enable_channel_index: false,
                enable_peak_rssi: true,
                enable_first_seen_timestamp: true,
                enable_last_seen_timestamp: true,
                enable_tag_seen_count: true,
                enable_access_spec_id: false,
                reserved: 0, // Unclear what this param is for
                air_protocol_epc_memory_selector: Vec::new(),
            },
            custom: Vec::new(),
        }),
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
