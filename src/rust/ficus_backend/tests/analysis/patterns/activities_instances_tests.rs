use ficus_backend::{
    event_log::{
        simple::simple_event_log::SimpleEventLog, core::{event_log::EventLog, event::event_hasher::NameEventHasher},
    },
    features::analysis::patterns::{
        entry_points::{discover_activities_instances, PatternsKind},
        repeat_sets::{ActivitiesDiscoveryContext, ActivityInTraceInfo, SubArrayWithTraceIndex},
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
