use ficus_backend::{
    event_log::{
        core::{event::event_hasher::NameEventHasher, event_log::EventLog},
        simple::simple_event_log::SimpleEventLog,
    },
    features::analysis::patterns::{
        entry_points::{
            discover_activities_and_create_new_log, discover_activities_instances, ActivitiesInstancesDiscovery,
            ActivityDiscoveryContext, PatternsKind,
        },
        repeat_sets::{
            ActivityInTraceInfo, RepeatsSetsDiscoveryContext, UndefActivityHandlingStrategy, UNDEF_ACTIVITY_NAME,
        },
    },
};

use crate::{
    analysis::patterns::utils::create_activity_name,
    test_core::simple_events_logs_provider::{create_log_from_taxonomy_of_patterns, create_maximal_repeats_log},
};

#[test]
fn test_activity_instances() {
    let log = create_log_from_taxonomy_of_patterns();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let context = RepeatsSetsDiscoveryContext::new(0, |sub_array| create_activity_name(&log, sub_array));
    let context = ActivityDiscoveryContext::new(context, PatternsKind::PrimitiveTandemArrays(20));

    let activities = discover_activities_instances(&hashes, &context);
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
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let context = RepeatsSetsDiscoveryContext::new(0, |sub_array| create_activity_name(&log, sub_array));
    let context = ActivityDiscoveryContext::new(context, PatternsKind::PrimitiveTandemArrays(20));
    let activities = discover_activities_instances(&hashes, &context);

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
    let log = create_log_from_taxonomy_of_patterns();
    execute_activities_discovery_test(
        &log,
        UndefActivityHandlingStrategy::InsertAllEvents,
        &vec![vec!["g", "d", "abc", "f", "i", "abc"]],
    );
}

fn execute_activities_discovery_test(
    log: &SimpleEventLog,
    strategy: UndefActivityHandlingStrategy,
    expected: &Vec<Vec<&str>>,
) {
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let context = RepeatsSetsDiscoveryContext::new(0, |sub_array| create_activity_name(&log, sub_array));
    let context = ActivityDiscoveryContext::new(context, PatternsKind::PrimitiveTandemArrays(20));
    let context = ActivitiesInstancesDiscovery::new(strategy, context);

    let new_log = discover_activities_and_create_new_log(log, &hashes, &context);

    assert_eq!(new_log.borrow().to_raw_vector(), *expected);
}

#[test]
fn test_creating_new_log_from_activity_instances_insert_as_single_event() {
    let log = create_log_from_taxonomy_of_patterns();
    execute_activities_discovery_test(
        &log,
        UndefActivityHandlingStrategy::InsertAsSingleEvent,
        &vec![vec![UNDEF_ACTIVITY_NAME, "abc", UNDEF_ACTIVITY_NAME, "abc"]],
    );
}

#[test]
fn test_creating_new_log_from_activity_instances_dont_insert() {
    let log = create_log_from_taxonomy_of_patterns();
    execute_activities_discovery_test(
        &log,
        UndefActivityHandlingStrategy::DontInsert,
        &vec![vec!["abc", "abc"]],
    );
}
