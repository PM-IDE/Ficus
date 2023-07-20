use ficus_backend::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use test_core::simple_events_logs_provider::create_simple_event_log;

mod test_core;

#[test]
fn test_event_log_info() {
    let log = create_simple_event_log();
    let creation_dto = EventLogInfoCreationDto::default(&log);
    let log_info = EventLogInfo::create_from(creation_dto);
    assert_eq!(log_info.get_events_count(), 6);

    assert_eq!(log_info.get_event_count(&"A".to_string()), 2usize);
    assert_eq!(log_info.get_event_count(&"B".to_string()), 2usize);
    assert_eq!(log_info.get_event_count(&"C".to_string()), 2usize);
}
