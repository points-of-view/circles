use llrp::{
    choices, enumerations,
    messages::{self, Message},
    parameters, BinaryMessage, LLRPMessage,
};
use std::{
    net::{self, TcpStream},
    time::Duration,
};

use super::{Reader, ReaderError, ReaderErrorKind};

const DEFAULT_PORT: u16 = 5084;
const DEFAULT_ROSPEC_ID: u32 = 1234;

#[derive(Debug)]
pub struct LLRPReader {
    hostname: String,
    stream: Option<TcpStream>,
}

impl Reader for LLRPReader {
    fn new(hostname: String) -> Result<Box<Self>, ReaderError> {
        if hostname.len() != 12 {
            return Err(ReaderError {
                kind: ReaderErrorKind::IncorrectHostname(hostname.clone()),
                message: format!(
                    "This should be exactly 12 characters, but was {}",
                    hostname.len()
                ),
            });
        }

        let mut reader = LLRPReader {
            hostname,
            stream: None,
        };
        reader.connect()?;
        reader.prepare()?;
        Ok(Box::new(reader))
    }

    fn start_reading(&self) -> Result<(), ReaderError> {
        // Just in case we are already reading, we should try to stop
        self.stop_reading()?;

        // Actually start
        self.write_message(Message::StartRospec(messages::StartRospec {
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

    fn stop_reading(&self) -> Result<(), ReaderError> {
        self.write_message(Message::StopRospec(messages::StopRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        let message = self.await_message::<messages::StopRospecResponse>();
        Ok(())
    }
}

impl LLRPReader {
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
        self.await_message_and::<messages::ReaderEventNotification>(
            |m: &messages::ReaderEventNotification| {
                m.reader_event_notification_data.connection_attempt_event
                    == Some(parameters::ConnectionAttemptEvent {
                        status: enumerations::ConnectionAttemptStatusType::Success,
                    })
            },
        )?;
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
        self.write_message(Message::DeleteRospec(messages::DeleteRospec {
            ro_spec_id: 0,
        }))?;
        self.await_message_and::<messages::DeleteRospecResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        // Add our new ro_spec
        self.write_message(Message::AddRospec(messages::AddRospec {
            ro_spec: construct_default_rospec(),
        }))?;
        self.await_message_and::<messages::AddRospecResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        // Enable our new ro_spec
        self.write_message(Message::EnableRospec(messages::EnableRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        self.await_message_and::<messages::EnableRospecResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        Ok(())
    }
}

fn construct_default_rospec() -> parameters::ROSpec {
    parameters::ROSpec {
        ro_spec_id: DEFAULT_ROSPEC_ID,
        priority: 0,
        current_state: enumerations::ROSpecState::Disabled, // Setting this to `Inactive` or `Active` results in an error from our reader
        ro_boundary_spec: parameters::ROBoundarySpec {
            ro_spec_start_trigger: parameters::ROSpecStartTrigger {
                ro_spec_start_trigger_type: enumerations::ROSpecStartTriggerType::Null,
                periodic_trigger_value: None,
                gpi_trigger_value: None,
            },
            ro_spec_stop_trigger: parameters::ROSpecStopTrigger {
                ro_spec_stop_trigger_type: enumerations::ROSpecStopTriggerType::Null,
                // We have to pass a duration, but this value is ignored since out trigger type isn't `Duration`
                duration_trigger_value: 0,
                gpi_trigger_value: None,
            },
        },
        spec_parameter: vec![choices::SpecParameter::AISpec(parameters::AISpec {
            antenna_ids: vec![1, 2, 3],
            ai_spec_stop_trigger: parameters::AISpecStopTrigger {
                ai_spec_stop_trigger_type: enumerations::AISpecStopTriggerType::Null,
                // We have to pass a duration, but this value is ignored since out trigger type isn't `Duration`
                duration_trigger: 0,
                gpi_trigger_value: None,
                tag_observation_trigger: None,
            },
            inventory_parameter_spec: vec![parameters::InventoryParameterSpec {
                inventory_parameter_spec_id: 1,
                protocol_id: enumerations::AirProtocols::EPCGlobalClass1Gen2,
                antenna_configuration: Vec::new(),
                custom: Vec::new(),
            }],
            custom: Vec::new(),
        })],
        ro_report_spec: Some(parameters::ROReportSpec {
            // NOTE: The spec defines trigger based on N amount of milliseconds, but our readers doesn't accept these
            ro_report_trigger:
                enumerations::ROReportTriggerType::Upon_N_Tags_Or_End_Of_AISpec_Or_End_Of_RFSurveySpec,
            n: 1,
            tag_report_content_selector: parameters::TagReportContentSelector {
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
