use llrp::{choices, enumerations, parameters};

use super::DEFAULT_ROSPEC_ID;

pub fn construct_default_rospec() -> parameters::ROSpec {
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
          // NOTE: The llrp crate includes mre options, but these options were added in version 1.1.0 of the LLRP spec
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
