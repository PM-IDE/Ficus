use ficus_backend::{
    event_log::{
        core::{event::event_hasher::NameEventHasher, event_log::EventLog},
        simple::simple_event_log::SimpleEventLog,
    },
    features::analysis::patterns::{
        entry_points::{discover_activities_instances, PatternsKind, discover_activities_and_create_new_log},
        repeat_sets::{ActivitiesDiscoveryContext, ActivityInTraceInfo, SubArrayWithTraceIndex, create_new_log_from_activities_instances},
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
    let context = ActivitiesDiscoveryContext::new(0, |sub_array| create_activity_name(&log, sub_array));

    let activities = discover_activities_instances(&hashes, PatternsKind::PrimitiveTandemArrays(20), context);
    let activities = dump_activities(&activities.borrow());
    assert_eq!(activities, [[(2, 17), (17, 19)]]);
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
    let context = ActivitiesDiscoveryContext::new(0, |sub_array| create_activity_name(&log, sub_array));

    let activities = discover_activities_instances(&hashes, PatternsKind::PrimitiveTandemArrays(20), context);
    let activities = dump_activities(&activities.borrow());
    assert_eq!(
        activities,
        vec![
            vec![(0, 10)],
            vec![(0, 10)],
            vec![(0, 12)],
            vec![(0, 10)],
            vec![(0, 10), (10, 20), (20, 23)]
        ]
    );
}

#[test]
fn test_creating_new_log_from_activity_instances() {
    let log = create_log_from_taxonomy_of_patterns();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let context = ActivitiesDiscoveryContext::new(0, |sub_array| create_activity_name(&log, sub_array));
    let new_log = discover_activities_and_create_new_log(&log, &hashes, PatternsKind::PrimitiveTandemArrays(20), context);

    println!("{:?}", new_log.borrow().to_raw_vector());
}
