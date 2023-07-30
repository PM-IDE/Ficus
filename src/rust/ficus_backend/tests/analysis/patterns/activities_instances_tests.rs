use std::{cell::RefCell, rc::Rc};

use ficus_backend::{
    event_log::{core::event::event_hasher::default_class_extractor, simple::simple_event_log::SimpleEventLog},
    features::analysis::patterns::{
        activity_instances::{ActivityInTraceInfo, UndefActivityHandlingStrategy, UNDEF_ACTIVITY_NAME},
        contexts::{ActivitiesDiscoveryContext, ActivitiesInstancesDiscoveryContext, PatternsDiscoveryContext},
        entry_points::{discover_activities_and_create_new_log, discover_activities_instances, PatternsKind},
    },
};

use crate::{
    analysis::patterns::utils::create_activity_name,
    test_core::simple_events_logs_provider::{create_log_from_taxonomy_of_patterns, create_maximal_repeats_log},
};

#[test]
fn test_activity_instances() {
    let log = Rc::new(RefCell::new(create_log_from_taxonomy_of_patterns()));

    let patterns_context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::PrimitiveTandemArrays(20),
        default_class_extractor,
    );

    let context = ActivitiesDiscoveryContext::new(patterns_context, 0, |sub_array| {
        create_activity_name(&log.borrow(), sub_array)
    });

    let activities = discover_activities_instances(&context);
    let activities = dump_activities(&activities.borrow());

    assert_eq!(activities, [[(2, 15), (17, 19)]]);
}

fn dump_activities(instances: &Vec<Vec<ActivityInTraceInfo>>) -> Vec<Vec<(usize, usize)>> {
    instances
        .into_iter()
        .map(|trace_instances| trace_instances.into_iter().map(|instance| instance.dump()).collect())
        .collect()
}

#[test]
fn test_activity_instances1() {
    let log = Rc::new(RefCell::new(create_maximal_repeats_log()));

    let patterns_context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::PrimitiveTandemArrays(20),
        default_class_extractor,
    );

    let context = ActivitiesDiscoveryContext::new(patterns_context, 0, |sub_array| {
        create_activity_name(&log.borrow(), sub_array)
    });

    let activities = discover_activities_instances(&context);

    let activities = dump_activities(&activities.borrow());
    assert_eq!(
        activities,
        vec![
            vec![(0, 10)],
            vec![(0, 10)],
            vec![(0, 12)],
            vec![(0, 10)],
            vec![(0, 9), (10, 19), (20, 23)]
        ]
    );
}

#[test]
fn test_creating_new_log_from_activity_instances_insert_all_events() {
    execute_activities_discovery_test(
        create_log_from_taxonomy_of_patterns(),
        UndefActivityHandlingStrategy::InsertAllEvents,
        &vec![vec!["g", "d", "abc", "f", "i", "abc"]],
    );
}

fn execute_activities_discovery_test(
    log: SimpleEventLog,
    strategy: UndefActivityHandlingStrategy,
    expected: &Vec<Vec<&str>>,
) {
    let log = Rc::new(RefCell::new(log));

    let patterns_context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::PrimitiveTandemArrays(20),
        default_class_extractor,
    );

    let context = ActivitiesDiscoveryContext::new(patterns_context, 0, |sub_array| {
        create_activity_name(&log.borrow(), sub_array)
    });
    let context = ActivitiesInstancesDiscoveryContext::new(context, strategy);

    let new_log = discover_activities_and_create_new_log(&context);

    assert_eq!(new_log.borrow().to_raw_vector(), *expected);
}

#[test]
fn test_creating_new_log_from_activity_instances_insert_as_single_event() {
    execute_activities_discovery_test(
        create_log_from_taxonomy_of_patterns(),
        UndefActivityHandlingStrategy::InsertAsSingleEvent,
        &vec![vec![UNDEF_ACTIVITY_NAME, "abc", UNDEF_ACTIVITY_NAME, "abc"]],
    );
}

#[test]
fn test_creating_new_log_from_activity_instances_dont_insert() {
    execute_activities_discovery_test(
        create_log_from_taxonomy_of_patterns(),
        UndefActivityHandlingStrategy::DontInsert,
        &vec![vec!["abc", "abc"]],
    );
}
