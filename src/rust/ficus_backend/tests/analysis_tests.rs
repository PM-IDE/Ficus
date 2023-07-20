use std::collections::HashMap;

use ficus_backend::features::analysis::{
    dfg_entropy::{calculate_default_dfg_entropy, calculate_laplace_dfg_entropy},
    event_log_info::EventLogInfo,
};

use crate::test_core::simple_events_logs_provider::{
    create_log_from_filter_out_chaotic_events, create_simple_event_log,
};

mod test_core;

#[test]
fn test_dfg_info() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(&log, false);
    let dfg = log_info.get_dfg_info();

    assert_eq!(dfg.get_directly_follows_count(&("A".to_string(), "B".to_string())), 2);
    assert_eq!(dfg.get_directly_follows_count(&("B".to_string(), "C".to_string())), 2);
    assert_eq!(dfg.get_directly_follows_count(&("A".to_string(), "C".to_string())), 0);
    assert_eq!(dfg.get_directly_follows_count(&("C".to_string(), "B".to_string())), 0);
    assert_eq!(dfg.get_directly_follows_count(&("B".to_string(), "A".to_string())), 0);

    assert!(dfg.is_event_with_single_follower(&"A".to_string()));
    assert!(dfg.is_event_with_single_follower(&"B".to_string()));
    assert!(!dfg.is_event_with_single_follower(&"C".to_string()));

    let followers = dfg.get_followed_events(&"A".to_string()).unwrap();
    assert_eq!(followers.get(&"B".to_string()).unwrap(), &2usize);

    let followers = dfg.get_followed_events(&"B".to_string()).unwrap();
    assert_eq!(followers.get(&"C".to_string()).unwrap(), &2usize);

    assert_eq!(dfg.get_followed_events(&"C".to_string()), None);
}

#[test]
fn test_dfg_entropy() {
    let log = create_log_from_filter_out_chaotic_events();
    let entropies = calculate_default_dfg_entropy(&log);
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 1.8365916681089791),
        ("b".to_string(), 1.8365916681089791),
        ("x".to_string(), 3.169925001442312),
        ("a".to_string(), 0.9182958340544896),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_dfg_laplace_entropy() {
    let log = create_log_from_filter_out_chaotic_events();
    let entropies = calculate_laplace_dfg_entropy(&log);
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 1.905904975406124),
        ("b".to_string(), 1.905904975406124),
        ("x".to_string(), 3.2127002996007796),
        ("a".to_string(), 1.002726083454847),
    ]);

    assert_eq!(entropies, expected);
}
