use crate::test_core::simple_events_logs_provider::create_simple_event_log3;
use ficus_backend::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use ficus_backend::features::discovery::alpha::discover_petri_net_alpha;
use ficus_backend::features::discovery::petri_net_serialization::serialize_to_pnml;

#[test]
pub fn test_serialization_1() {
    let log = create_simple_event_log3();
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));

    let petri_net = discover_petri_net_alpha(info);
    println!("{:?}", serialize_to_pnml(&petri_net));
}
