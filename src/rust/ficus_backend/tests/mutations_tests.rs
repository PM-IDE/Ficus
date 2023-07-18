use std::{collections::HashSet, vec};
use test_core::simple_events_logs_provider::create_simple_event_log;

use ficus_backend::{
    event_log::core::{event::Event, event_log::EventLog},
    features::mutations::filtering::{filter_log_by_name, filter_log_by_names},
};

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

#[test]
fn test_removing_events3() {
    let mut log = create_simple_event_log();
    filter_log_by_name(&mut log, "A");

    assert_eq!(log.to_raw_vector(), vec![vec!["B", "C"], vec!["B", "C"]]);
}

#[test]
fn test_removing_events4() {
    let mut log = create_simple_event_log();
    let set = HashSet::from_iter(vec!["A".to_string(), "B".to_string()]);
    filter_log_by_names(&mut log, &set);

    assert_eq!(log.to_raw_vector(), vec![vec!["C"], vec!["C"]]);
}

#[test]
fn test_removing_events5() {
    let mut log = create_simple_event_log();
    let set = HashSet::from_iter(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    filter_log_by_names(&mut log, &set);

    assert!(log.to_raw_vector().is_empty());
}
