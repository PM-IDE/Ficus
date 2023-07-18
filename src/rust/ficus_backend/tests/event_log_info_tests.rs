use std::collections::HashMap;

use ficus_backend::features::analysis::event_log_info::EventLogInfo;
use test_core::simple_events_logs_provider::create_simple_event_log;

mod test_core;

#[test]
fn test_event_log_info() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(&log);
    assert_eq!(log_info.get_events_count(), 6);

    let expected = HashMap::from([("A".to_string(), 2usize), ("B".to_string(), 2), ("C".to_string(), 2)]);

    assert_eq!(log_info.get_event_classes_names(), &expected);
}
