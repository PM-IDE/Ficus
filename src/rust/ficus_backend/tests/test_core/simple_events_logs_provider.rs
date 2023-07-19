use ficus_backend::event_log::simple::simple_event_log::SimpleEventLog;

pub fn create_raw_event_log() -> Vec<Vec<&'static str>> {
    vec![vec!["A", "B", "C"], vec!["A", "B", "C"]]
}

pub fn create_simple_event_log() -> SimpleEventLog {
    SimpleEventLog::new(&create_raw_event_log())
}

pub fn create_raw_event_log2() -> Vec<Vec<&'static str>> {
    vec![
        vec!["A", "B", "C", "D", "E"],
        vec!["B", "C", "E", "A", "A", "A"],
        vec!["A", "E", "C", "B", "B", "B", "E", "A"],
        vec!["A", "B", "C", "C", "A"],
        vec!["B", "C", "E", "A", "A", "A"],
    ]
}

pub fn create_simple_event_log2() -> SimpleEventLog {
    SimpleEventLog::new(&create_raw_event_log2())
}

pub fn create_raw_event_log3() -> Vec<Vec<&'static str>> {
    vec![
        vec!["A", "B", "C", "D", "E"],
        vec!["B", "C", "E", "A", "A", "A"],
        vec!["A", "E", "C", "B", "B", "B", "E", "A"],
        vec!["A", "B", "C", "C", "A"],
        vec!["B", "C", "E", "A", "A", "A"],
        vec!["A", "B", "C", "D", "E"],
        vec!["A", "B", "C", "C", "A"],
        vec!["A", "B", "C", "C", "A"],
        vec!["A", "E", "C", "B", "B", "B", "E", "A"],
    ]
}

pub fn create_simple_event_log3() -> SimpleEventLog {
    SimpleEventLog::new(&create_raw_event_log3())
}
