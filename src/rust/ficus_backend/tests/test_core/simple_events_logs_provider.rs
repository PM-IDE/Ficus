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

pub fn create_log_from_filter_out_chaotic_events() -> SimpleEventLog {
    let mut raw_log = vec![];

    for _ in 0..10 {
        raw_log.push(vec!["a", "b", "c", "x"]);
        raw_log.push(vec!["a", "b", "x", "c"]);
        raw_log.push(vec!["a", "x", "b", "c"]);
    }

    SimpleEventLog::new(&raw_log)
}

pub fn create_log_from_filter_out_chaotic_events_with_noise() -> SimpleEventLog {
    let mut raw_log = vec![];

    for _ in 0..10 {
        raw_log.push(vec![
            "d", "v", "d", "d", "a", "d", "b", "c", "x", "d", "d", "d", "d", "d",
        ]);
        raw_log.push(vec!["a", "d", "d", "d", "d", "b", "d", "x", "c", "d"]);
        raw_log.push(vec!["d", "d", "d", "v", "d", "a", "x", "b", "c", "d"]);
    }

    SimpleEventLog::new(&raw_log)
}

pub fn create_log_from_taxonomy_of_patterns() -> SimpleEventLog {
    let raw_log = vec![vec![
        "g", "d", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "f", "i", "c", "a",
    ]];
    SimpleEventLog::new(&raw_log)
}

pub fn create_no_tandem_array_log() -> SimpleEventLog {
    let raw_log = vec![vec!["a", "b", "c", "d"]];
    SimpleEventLog::new(&raw_log)
}

pub fn create_one_tandem_array_log() -> SimpleEventLog {
    let raw_log = vec![vec!["a", "b", "a", "b", "c", "d"]];
    SimpleEventLog::new(&raw_log)
}
