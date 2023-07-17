use ficus_backend::event_log::simple::simple_event_log::SimpleEventLog;

pub fn create_simple_event_log() -> SimpleEventLog {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    SimpleEventLog::new(&raw_log)
}
