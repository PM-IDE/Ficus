use std::{cell::RefCell, rc::Rc};

use test_core::simple_events_logs_provider::{create_simple_event_log, create_simple_event_log2, create_simple_event_log3};

use ficus_backend::{
    event_log::{
        core::{event::Event, trace::Trace, event_log::EventLog},
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

#[test]
pub fn test_split_log3() {
    let log = create_simple_event_log3();
    let splitted = to_strings_vec(split_by_traces(&log));

    assert_eq!(
        splitted,
        vec![
            vec![vec!["A", "B", "C", "D", "E"], vec!["A", "B", "C", "D", "E"]],
            vec![vec!["B", "C", "E", "A", "A", "A"], vec!["B", "C", "E", "A", "A", "A"]],
            vec![vec!["A", "E", "C", "B", "B", "B", "E", "A"], vec!["A", "E", "C", "B", "B", "B", "E", "A"]],
            vec![vec!["A", "B", "C", "C", "A"], vec!["A", "B", "C", "C", "A"], vec!["A", "B", "C", "C", "A"]]
        ]
    )
}

#[test]
pub fn test_mutations_log1() {
    let log = create_simple_event_log();
    let splitted: Vec<Vec<Rc<RefCell<SimpleTrace>>>> = split_by_traces(&log);

    let new_name = "ASDASD".to_string();
    splitted[0][0].borrow_mut().get_events()[0].borrow_mut().set_name(&new_name);
    assert_eq!(log.get_traces()[0].borrow().get_events()[0].borrow().get_name(), &new_name);
}

#[test]
pub fn test_mutations_log2() {
    let log = create_simple_event_log2();
    let splitted = split_by_traces(&log);

    let new_name = "ASD".to_string();
    splitted[3][0].borrow_mut().get_events().last().unwrap().borrow_mut().set_name(&new_name);
    assert_eq!(log.get_traces()[3].borrow().get_events().last().unwrap().borrow().get_name(), &new_name);
}

#[test]
pub fn test_mutations_log3() {
    let log = create_simple_event_log3();
    let splitted = split_by_traces(&log);

    let new_name = "ASD123".to_string();
    splitted[2][1].borrow_mut().get_events().last().unwrap().borrow_mut().set_name(&new_name);
    assert_eq!(log.get_traces().last().unwrap().borrow().get_events().last().unwrap().borrow().get_name(), &new_name);
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
