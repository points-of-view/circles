use llrp::{choices, enumerations, parameters, Encoder, LLRPValue};

use super::DEFAULT_ROSPEC_ID;

pub fn construct_default_rospec() -> parameters::ROSpec {
    let moto_antenna_stop_condition_param = parameters::Custom {
        // This is the MotoAntennaStopCondition from Zebra's custom extensions
        // See their manual page 183
        vendor_identifier: 161,
        parameter_subtype: 704,
        // This sets the following values:
        // * AntennaStopTrigger: Set the trigger to `DwellTime` (0) or `NumberInventoryCycles` (1)
        // * AntennaStopConditionValue: Set the associated value in milliseconds or number of cycles
        // This is represented in an u8 vec, where the first byte represent the trigger and the next two bytes the value
        data: vec![0, 0, 250],
    };
    let mut encoded_moto_antenna_stop_condition_param: Vec<u8> = vec![];
    let mut encoder = Encoder::new(&mut encoded_moto_antenna_stop_condition_param);
    moto_antenna_stop_condition_param.encode(&mut encoder);

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
              // This duration sets the dwell time for each antenna (in MS), so we can control how long each antenna reads before going to the next one
              duration_trigger: 0,
              gpi_trigger_value: None,
              tag_observation_trigger: None,
          },
          inventory_parameter_spec: vec![parameters::InventoryParameterSpec {
              inventory_parameter_spec_id: 1,
              protocol_id: enumerations::AirProtocols::EPCGlobalClass1Gen2,
              antenna_configuration: vec![parameters::AntennaConfiguration {
                antenna_id: 0, // Antenna ID 0 means this applies to all antennas
                rf_receiver: None,
                rf_transmitter: None,
                air_protocol_inventory_command_settings: vec![
                    choices::AirProtocolInventoryCommandSettings::C1G2InventoryCommand(
                    parameters::C1G2InventoryCommand {
                        tag_inventory_state_aware: false,
                        reserved: 0,
                        c1g2_filter: Vec::new(),
                        c1g2_rf_control: None,
                        c1g2_singulation_control: None,
                        custom: vec![
                            // We use custom params to control the dwell time of the antennas. This needs to be set as part of a C1G2InventoryCommand
                            // according to the Zebra manual.
                            // Note that setting the wrong custom parameter at the wrong location (while still providing a valid configuration)
                            // often results in the reader freezing up and not returning any messages until a force-reboot.
                            parameters::Custom {
                                // This is the `MotoAntennaConfig` from Zebra's custom extensions
                                // See page 183 of their manual
                                // This wraps our `MotoAntennaStopCondition`
                                // It also supports other optional parameters (which we don't use)
                                vendor_identifier: 161,
                                parameter_subtype: 703,
                                data: encoded_moto_antenna_stop_condition_param
                            }
                        ],
                    })
                ]
              }],
              custom: Vec::new(),
          }],
          custom: Vec::new(),
      })],
      ro_report_spec: Some(parameters::ROReportSpec {
          // NOTE: The llrp crate includes more options, but these options were added in version 1.1.0 of the LLRP spec
          // The Zebra FX9600 that we use only supports version 1.0.1 of the spec.
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
              reserved: 0, // Unclear what this field is for
              air_protocol_epc_memory_selector: Vec::new(),
          },
          custom: Vec::new(),
      }),
  }
}
