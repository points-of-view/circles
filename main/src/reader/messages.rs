use crate::tags::Tag;

use super::{ReaderError, ReaderErrorKind};
use llrp::{
    messages::{Keepalive, Message},
    BinaryMessage, LLRPMessage,
};
use std::io;

pub fn write_message<W: io::Write>(
    writer: W,
    message: Message,
    id: Option<u32>,
) -> Result<(), ReaderError> {
    // NOTE: The message id can be a random u32 - it is returned in the matching response
    // We can unwrap here, since `from_dynamic_message` doesn't ever fail
    let message = BinaryMessage::from_dynamic_message(id.unwrap_or(20), &message).unwrap();
    match llrp::write_message(writer, message) {
        Ok(_) => Ok(()),
        Err(err) => {
            #[cfg(debug_assertions)]
            println!("Unknown error {:#?}", err);
            Err(ReaderError {
                kind: ReaderErrorKind::Unknown,
                message: format!("Error writing: {:?}", err.to_string()),
            })
        }
    }
}

pub fn handle_new_message<S: io::Write>(message: Message, tags: &mut Vec<Tag>, stream: S) {
    match message {
        Message::RoAccessReport(message) => {
            for report_data in message.tag_report_data {
                match Tag::from_report_data(report_data) {
                    Ok(tag) => tags.push(tag),
                    Err(err) => {
                        // We print faulty tags in development (so we can learn from them)
                        // In production these get ignored
                        #[cfg(debug_assertions)]
                        println!("{:?}", err)
                    }
                }
            }
        }
        Message::Keepalive(_) => respond_to_keepalive(stream),
        // We can just ignore other messages for now, but print them in development
        other_message => {
            #[cfg(debug_assertions)]
            println!("Got unexpected message {:?}", other_message)
        }
    }
}

pub fn parse_message_and<S, T: LLRPMessage>(
    mut stream: S,
    closure: fn(message: &T) -> bool,
) -> Result<T, ReaderError>
where
    S: io::Read + io::Write,
    for<'a> &'a mut S: io::Read,
{
    let binary_message = read_message(&mut stream)?;

    match &binary_message.to_message::<Keepalive>() {
        Ok(_) => respond_to_keepalive(stream),
        Err(_) => (),
    }

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

fn read_message<S: io::Read>(stream: S) -> Result<BinaryMessage, ReaderError> {
    match llrp::read_message(stream) {
        Ok(message) => Ok(message),
        Err(err) => {
            return Err(ReaderError {
                kind: ReaderErrorKind::Unknown,
                message: err.to_string(),
            })
        }
    }
}

fn respond_to_keepalive<S: io::Write>(stream: S) {
    let _ = write_message(
        stream,
        llrp::messages::Message::KeepaliveAck(llrp::messages::KeepaliveAck {}),
        None,
    );
}

#[cfg(test)]
pub fn construct_tag_report(antenna_id: u16, peak_rssi: i8) -> llrp::parameters::TagReportData {
    llrp::parameters::TagReportData {
        epc_parameter: llrp::choices::EPCParameter::EPC_96([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ro_spec_id: None,
        spec_index: None,
        inventory_parameter_spec_id: None,
        antenna_id: Some(antenna_id),
        peak_rssi: Some(peak_rssi),
        channel_index: None,
        first_seen_timestamp_utc: None,
        first_seen_timestamp_uptime: None,
        last_seen_timestamp_utc: None,
        last_seen_timestamp_uptime: None,
        tag_seen_count: None,
        air_protocol_tag_data: Vec::new(),
        access_spec_id: None,
        access_command_op_spec_result: Vec::new(),
        custom: Vec::new(),
    }
}

#[cfg(test)]
pub fn construct_report_message(antenna_id: u16, peak_rssi: i8) -> Message {
    let report = construct_tag_report(antenna_id, peak_rssi);

    Message::RoAccessReport(llrp::messages::RoAccessReport {
        tag_report_data: vec![report],
        rf_survey_report_data: Vec::new(),
        custom: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use llrp::{
        enumerations::StatusCode,
        messages::{self, StopRospecResponse},
        parameters::LLRPStatus,
        LLRPMessage,
    };
    use std::io::Cursor;

    #[test]
    fn should_add_correct_tag_to_vector() {
        let mut vec: Vec<Tag> = vec![];
        let stream = Cursor::<Vec<u8>>::new(vec![]);

        let message = construct_report_message(1, -40);
        handle_new_message(message, &mut vec, stream);

        assert_eq!(1, vec.len());
    }

    #[test]
    fn should_ignore_incorrect_tags() {
        let mut vec: Vec<Tag> = vec![];
        let stream = Cursor::<Vec<u8>>::new(vec![]);

        let message = construct_report_message(10, -40);
        handle_new_message(message, &mut vec, stream);

        assert_eq!(0, vec.len());
    }

    #[test]
    fn should_ignore_other_messages() {
        let mut vec: Vec<Tag> = vec![];
        let stream = Cursor::<Vec<u8>>::new(vec![]);

        let message = Message::StopRospecResponse(StopRospecResponse {
            status: LLRPStatus {
                status_code: StatusCode::M_Success,
                error_description: String::from(""),
                field_error: None,
                parameter_error: None,
            },
        });
        handle_new_message(message, &mut vec, stream);

        assert_eq!(0, vec.len());
    }

    #[test]
    fn should_respond_to_keepalive_with_keepalive_ack() {
        let mut tags: Vec<Tag> = vec![];
        let mut stream = Cursor::<Vec<u8>>::new(vec![]);
        let message = Message::Keepalive(messages::Keepalive {});

        handle_new_message(message, &mut tags, &mut stream);
        stream.set_position(0);

        let raw = llrp::read_message(stream).unwrap();

        assert_eq!(raw.message_type, llrp::messages::KeepaliveAck::ID);
    }
}
