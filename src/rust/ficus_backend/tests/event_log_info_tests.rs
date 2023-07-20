use ficus_backend::features::analysis::{dfg_entropy::calculate_default_dfg_entropy, event_log_info::EventLogInfo};
use test_core::simple_events_logs_provider::create_simple_event_log;

use crate::test_core::simple_events_logs_provider::create_log_from_filter_out_chaotic_events;

mod test_core;

#[test]
fn test_event_log_info() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(&log, false);
    assert_eq!(log_info.get_events_count(), 6);

    assert_eq!(log_info.get_event_count(&"A".to_string()), 2usize);
    assert_eq!(log_info.get_event_count(&"B".to_string()), 2usize);
    assert_eq!(log_info.get_event_count(&"C".to_string()), 2usize);
}

#[test]
fn test_dfg_entropy() {
    let log = create_simple_event_log();
    println!("{:?}", calculate_default_dfg_entropy(&log));
}

#[test]
fn test_dfg_entropy_log_from_paper() {
    let log = create_log_from_filter_out_chaotic_events();
    println!("{:?}", calculate_default_dfg_entropy(&log));
}
