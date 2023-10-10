use crate::test_core::simple_events_logs_provider::{create_raw_event_log3, create_simple_event_log3};
use ficus_backend::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use ficus_backend::features::discovery::alpha::alpha::discover_petri_net_alpha;

#[test]
pub fn alpha_simple_test_1() {
    let log = create_simple_event_log3();
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));

    let petri_net = discover_petri_net_alpha(&info);
    println!("{:?}", petri_net);
}
