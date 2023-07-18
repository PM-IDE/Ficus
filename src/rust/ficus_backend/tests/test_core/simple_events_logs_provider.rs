use ficus_backend::event_log::simple::simple_event_log::SimpleEventLog;

pub fn create_simple_event_log() -> SimpleEventLog {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    SimpleEventLog::new(&raw_log)
}

pub fn create_simple_event_log2() -> SimpleEventLog {
    let raw_log = vec![
        vec!["A", "B", "C", "D", "E"],
        vec!["B", "C", "E", "A", "A", "A"],
        vec!["A", "E", "C", "B", "B", "B", "E", "A"],
        vec!["A", "B", "C", "C", "A"],
        vec!["B", "C", "E", "A", "A", "A"],
    ];

    SimpleEventLog::new(&raw_log)
}
