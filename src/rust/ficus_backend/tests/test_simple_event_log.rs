use ficus_backend::event_log::simple::simple_event_log::SimpleEventLog;

#[test]
fn test_simple_event_log_creation() {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    let simple_event_log = SimpleEventLog::new(&raw_log);
    assert_eq!(raw_log, simple_event_log.to_raw_vector())
}
