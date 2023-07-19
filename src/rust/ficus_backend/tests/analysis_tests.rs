use ficus_backend::features::analysis::event_log_info::EventLogInfo;

use crate::test_core::simple_events_logs_provider::create_simple_event_log;

mod test_core;

#[test]
fn test_dfg_info() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(&log);
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
