use llrp::{
    enumerations,
    messages::{self, Message},
    parameters, LLRPMessage,
};
use std::{
    io::Write,
    net::{self, TcpStream},
    time::Duration,
};
use tauri::{async_runtime::JoinHandle, AppHandle, Manager};

use super::{
    handle_reader_input,
    messages::{parse_message_and, write_message},
    rospec::construct_default_rospec,
    ReaderError, ReaderErrorKind, ReaderProtocol, DEFAULT_ROSPEC_ID, REFRESH_INTERVAL,
};

const DEFAULT_PORT: u16 = 5084;

/// Interact with an LLRP-compatible RFID-reader
///
/// ## LLRP standard
/// For more info and the full documentation, see [this site for a PDF of the standard](https://www.gs1.org/standards/epc-rfid/llrp/1-1-0).  
/// Note that the device we use (a Zebra FX9600) only support version 1.0.0 or 1.0.1 of the standard and we
/// cannot use feature from version 1.1.0
///
/// We are also able to use the custom extensions made by Zebra. See [Zebra's docs for the available custom extensions](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/interface-control-guide-en.pdf)
#[derive(Debug)]
pub struct LLRPReader {
    hostname: String,
    stream: Option<TcpStream>,
    handle: Option<JoinHandle<()>>,
}

impl ReaderProtocol for LLRPReader {
    fn new<R: tauri::Runtime>(
        hostname: String,
        app_handle: AppHandle<R>,
    ) -> Result<Self, ReaderError> {
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
        // NOTE: We log every step of the connection process to make it easier to understand when something goes wrong
        // We always ignore the output these emit events, since we don't care if this fails.
        let _ = app_handle.emit_all("connection-status", "Start connecting to reader");
        reader.connect(app_handle.clone())?;
        let _ = app_handle.emit_all("connection-status", "Start preparing reader for usage");
        reader.prepare(app_handle.clone())?;
        let _ = app_handle.emit_all("connection-status", "Connected and ready to go");
        Ok(reader)
    }

    fn start_reading<R: tauri::Runtime>(
        &mut self,
        app_handle: AppHandle<R>,
    ) -> Result<(), ReaderError> {
        // Just in case we are already reading, we should try to stop
        let _ = app_handle.emit_all("connection-status", "Stopping previous session");
        self.stop_reading(true)?;
        let _ = app_handle.emit_all("connection-status", "Previous session stopped");

        // Actually start
        self.write_message(Message::StartRospec(messages::StartRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        let _ = app_handle.emit_all("connection-status", "Started new session");

        let stream = self.stream.as_ref().unwrap().try_clone().unwrap();
        let handle = handle_reader_input::<R>(stream, app_handle.clone());
        self.handle = Some(handle);
        let _ = app_handle.emit_all("connection-status", "Ready to receive messages");
        Ok(())
    }

    fn stop_reading(&mut self, await_confirmation: bool) -> Result<(), ReaderError> {
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
    fn connect<R: tauri::Runtime>(&mut self, app_handle: AppHandle<R>) -> Result<(), ReaderError> {
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
                    #[cfg(debug_assertions)]
                    println!("Connection error {:#?}", err);
                    return Err(ReaderError {
                        kind: ReaderErrorKind::CouldNotConnect(self.hostname.clone()),
                        message: err.to_string(),
                    });
                }
            };
        }
        let _ = app_handle.emit_all("connection-status", "Opened connection to reader");

        // Wait for the first ReaderEventNotification and confirm that we are connected
        self.await_message_and::<messages::ReaderEventNotification>(
            |m: &messages::ReaderEventNotification| {
                m.reader_event_notification_data.connection_attempt_event
                    == Some(parameters::ConnectionAttemptEvent {
                        status: enumerations::ConnectionAttemptStatusType::Success,
                    })
            },
        )?;
        let _ = app_handle.emit_all("connection-status", "Receiving connection ");

        // Just in case the reader is holding on to some old reports, we try to flush the stream
        let _ = self.stream.as_ref().unwrap().flush();
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
        write_message(self.stream.as_ref().unwrap(), message, None)
    }

    fn parse_message_and<T: LLRPMessage>(
        &mut self,
        closure: fn(message: &T) -> bool,
    ) -> Result<T, ReaderError> {
        let stream = self.stream.as_mut().unwrap();
        parse_message_and(stream, closure)
    }

    fn parse_message<T: LLRPMessage>(&mut self) -> Result<T, ReaderError> {
        self.parse_message_and(|_| true)
    }

    fn await_message_and<T: LLRPMessage + std::fmt::Debug>(
        &mut self,
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
            false => {
                #[cfg(debug_assertions)]
                println!(
                    "Message did not pass closure check. Original message {:#?}",
                    message
                );
                Err(ReaderError {
                    kind: ReaderErrorKind::Unknown,
                    message: String::from("Message did not pass closure check."),
                })
            }
        }
    }

    fn await_message<T: LLRPMessage + std::fmt::Debug>(&mut self) -> Result<T, ReaderError> {
        self.await_message_and(|_| true)
    }

    fn prepare<R: tauri::Runtime>(&mut self, app_handle: AppHandle<R>) -> Result<(), ReaderError> {
        // Set reader config to emit keepalive messages
        self.write_message(Message::SetReaderConfig(messages::SetReaderConfig {
            reset_to_factory_default: true,
            reserved: 0, // Unclear what this field is for
            reader_event_notification_spec: None,
            antenna_properties: Vec::new(),
            antenna_configuration: Vec::new(),
            ro_report_spec: None,
            access_report_spec: None,
            keepalive_spec: Some(parameters::KeepaliveSpec {
                keepalive_trigger_type: enumerations::KeepaliveTriggerType::Periodic,
                periodic_trigger_value: REFRESH_INTERVAL / 2,
            }),
            gpo_write_data: Vec::new(),
            gpi_port_current_state: Vec::new(),
            events_and_reports: None,
            custom: Vec::new(),
        }))?;
        self.await_message_and::<messages::SetReaderConfigResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        let _ = app_handle.emit_all("connection-status", "Reset reader settings");

        // Remove all existing ro_specs in the reader. ro_spec_id `0` means all ro_spec's should be deleted
        self.write_message(Message::DeleteRospec(messages::DeleteRospec {
            ro_spec_id: 0,
        }))?;
        self.await_message_and::<messages::DeleteRospecResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        let _ = app_handle.emit_all("connection-status", "Removed old reader config");

        // Add our new ro_spec
        self.write_message(Message::AddRospec(messages::AddRospec {
            ro_spec: construct_default_rospec(),
        }))?;
        self.await_message_and::<messages::AddRospecResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        let _ = app_handle.emit_all("connection-status", "Added new reader config");

        // Enable our new ro_spec
        self.write_message(Message::EnableRospec(messages::EnableRospec {
            ro_spec_id: DEFAULT_ROSPEC_ID,
        }))?;
        self.await_message_and::<messages::EnableRospecResponse>(|m| {
            m.status.status_code == enumerations::StatusCode::M_Success
        })?;
        let _ = app_handle.emit_all("connection-status", "Enabled reader config");

        Ok(())
    }
}

impl Drop for LLRPReader {
    fn drop(&mut self) {
        // When dropping, we don't actually care for the responses
        if self.handle.is_some() {
            let _ = self.stop_reading(false);
        }
        if self.stream.is_some() {
            let _ = self.write_message(Message::CloseConnection(messages::CloseConnection {}));
        }
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
