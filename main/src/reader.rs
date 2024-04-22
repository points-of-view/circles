pub mod error;
mod llrp_reader;
mod rospec;

pub use error::{ReaderError, ReaderErrorKind};
use llrp::messages::Message;
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

// GENERAL NOTE: For a good overview of the LLRP spec, see [this document](https://gs1go2.azureedge.net/cdn/ff/7aZwEHsz5I8MM-lbIcvFNwjL9OFcGGZgBU_hAjJttEE/1416474591/public/docs/epc/llrp_1_1-standard-20101013.pdf).

const DEFAULT_ROSPEC_ID: u32 = 1234;
const REFRESH_INTERVAL: u64 = 500;

pub fn handle_reader_input<R: tauri::Runtime>(
    stream: TcpStream,
    app_handle: AppHandle<R>,
) -> JoinHandle<()> {
    spawn(async move {
        let mut tags: Vec<Tag> = vec![];
        let interval = Duration::from_millis(REFRESH_INTERVAL);
        let mut last_update = Instant::now();
        while let Ok(message) = llrp::read_message(&stream) {
            let message = message.to_dynamic_message().unwrap();
            handle_new_message(message, &mut tags);
            if last_update.elapsed() > interval {
                let new_map = TagsMap::from(tags.drain(..));
                app_handle.emit_all("updated-tags", new_map).unwrap();
                last_update = Instant::now();
            }
        }
    })
}

fn handle_new_message(message: Message, tags: &mut Vec<Tag>) {
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
        // We can just ignore other messages for now, but print them in development
        other_message => {
            #[cfg(debug_assertions)]
            println!("Got unexpected message {:?}", other_message)
        }
    }
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
    use llrp::{enumerations::StatusCode, messages::StopRospecResponse, parameters::LLRPStatus};

    #[test]
    fn should_add_correct_tag_to_vector() {
        let mut vec: Vec<Tag> = vec![];

        let message = construct_report_message(1, -40);
        handle_new_message(message, &mut vec);

        assert_eq!(1, vec.len());
    }

    #[test]
    fn should_ignore_incorrect_tags() {
        let mut vec: Vec<Tag> = vec![];

        let message = construct_report_message(10, -40);
        handle_new_message(message, &mut vec);

        assert_eq!(0, vec.len());
    }

    #[test]
    fn should_ignore_other_messages() {
        let mut vec: Vec<Tag> = vec![];

        let message = Message::StopRospecResponse(StopRospecResponse {
            status: LLRPStatus {
                status_code: StatusCode::M_Success,
                error_description: String::from(""),
                field_error: None,
                parameter_error: None,
            },
        });
        handle_new_message(message, &mut vec);

        assert_eq!(0, vec.len());
    }
}
