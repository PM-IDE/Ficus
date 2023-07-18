use ficus_backend::event_log::core::{event::Event, event_log::EventLog};
use test_core::simple_events_logs_provider::create_simple_event_log;

mod test_core;

#[test]
fn test_removing_events() {
    let mut log = create_simple_event_log();
    log.filter_events_by(|event| event.get_name() == "A");

    assert_eq!(log.to_raw_vector(), vec![vec!["B", "C"], vec!["B", "C"]]);
}

#[test]
fn test_removing_events2() {
    let mut log = create_simple_event_log();
    log.filter_events_by(|event| event.get_name() == "B" || event.get_name() == "C");

    assert_eq!(log.to_raw_vector(), vec![vec!["A"], vec!["A"]]);
}
