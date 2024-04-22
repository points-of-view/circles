use llrp::{
    enumerations,
    messages::{self, Message},
    parameters, BinaryMessage, LLRPMessage,
};
use std::{
    net::{self, TcpStream},
    time::Duration,
};
use tauri::{async_runtime::JoinHandle, AppHandle};

use super::{
    handle_reader_input, rospec::construct_default_rospec, ReaderError, ReaderErrorKind,
    DEFAULT_ROSPEC_ID,
};

const DEFAULT_PORT: u16 = 5084;

/// Interact with an LLRP-compatible RFID-reader
///
/// ## LLRP standard
/// For more info and the full documention, see [this site](https://www.gs1.org/standards/epc-rfid/llrp/1-1-0).
/// Note that the device we use (a Zebra FX9600) only support version 1.0.0 or 1.0.1 of the standard and we
/// cannot use feature from version 1.0.0
#[derive(Debug)]
pub struct LLRPReader {
    hostname: String,
    stream: Option<TcpStream>,
    handle: Option<JoinHandle<()>>,
}

impl LLRPReader {
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

        let mut reader = LLRPReader {
            hostname,
            stream: None,
            handle: None,
        };
        reader.connect()?;
        reader.prepare()?;
        Ok(reader)
    }

    pub fn start_reading<R: tauri::Runtime>(
        &mut self,
        app_handle: AppHandle<R>,
    ) -> Result<(), ReaderError> {
        // Just in case we are already reading, we should try to stop
        self.stop_reading(true)?;

        // Actually start
        self.write_message(Message::StartRospec(messages::StartRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;

        let stream = self.stream.as_ref().unwrap().try_clone().unwrap();
        let handle = handle_reader_input::<R>(stream, app_handle);
        self.handle = Some(handle);
        Ok(())
    }

    pub fn stop_reading(&mut self, await_confirmation: bool) -> Result<(), ReaderError> {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        };

        self.write_message(Message::StopRospec(messages::StopRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        if await_confirmation {
            self.await_message::<messages::StopRospecResponse>()?;
        }
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
    /// See [this manual p.50 for details](https://www.zebra.com/content/dam/zebra_new_ia/en-us/manuals/rfid/fxseries-ig-en.pdf)
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
        // NOTE: The message id can be a random u32 - it is returned in the matching response
        // We can unwrap here, since `from_dynamic_message` doesn't ever fail
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

impl Drop for LLRPReader {
    fn drop(&mut self) {
        // When dropping, we don't actually care for the responses
        let _ = self.stop_reading(false);
        let _ = self.write_message(Message::CloseConnection(messages::CloseConnection {}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn should_convert_hostname_to_ip() {
        let reader = LLRPReader {
            hostname: "fx9600749620".to_string(),
            stream: None,
            handle: None,
        };
        let ipv4 = reader.hostname_as_ip();

        assert!(ipv4.is_ok());
        assert_eq!(Ipv4Addr::new(169, 254, 150, 32), ipv4.unwrap())
    }

    #[test]
    fn should_return_err_if_hostname_cannot_convert() {
        let reader = LLRPReader {
            hostname: "fx960074XX20".to_string(),
            stream: None,
            handle: None,
        };
        let ipv4 = reader.hostname_as_ip();

        assert!(ipv4.is_err_and(
            |err| err.kind == ReaderErrorKind::IncorrectHostname("fx960074XX20".to_string())
        ));
    }
}
