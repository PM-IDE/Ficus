use std::{cell::RefCell, rc::Rc};

use test_core::simple_events_logs_provider::{create_simple_event_log, create_simple_event_log2};

use ficus_backend::{
    event_log::{
        core::trace::Trace,
        simple::simple_event_log::SimpleTrace,
    },
    features::mutations::split::split_by_traces,
};

mod test_core;

#[test]
fn test_split_log() {
    let log = create_simple_event_log();
    let splitted = to_strings_vec(split_by_traces(&log));

    assert_eq!(splitted, vec![vec![vec!["A", "B", "C"], vec!["A", "B", "C"]]]);
}

#[test]
fn test_split_log2() {
    let log = create_simple_event_log2();
    let splitted = to_strings_vec(split_by_traces(&log));

    assert_eq!(
        splitted,
        vec![
            vec![vec!["A", "B", "C", "D", "E"]],
            vec![vec!["B", "C", "E", "A", "A", "A"], vec!["B", "C", "E", "A", "A", "A"]],
            vec![vec!["A", "E", "C", "B", "B", "B", "E", "A"]],
            vec![vec!["A", "B", "C", "C", "A"]]
        ]
    );
}

fn to_strings_vec(groups: Vec<Vec<Rc<RefCell<SimpleTrace>>>>) -> Vec<Vec<Vec<String>>> {
    let mut result = Vec::new();

    for group in groups {
        let mut group_vec = Vec::new();
        for trace in group {
            group_vec.push(trace.borrow().to_names_vec());
        }

        result.push(group_vec);
    }

    result
}
