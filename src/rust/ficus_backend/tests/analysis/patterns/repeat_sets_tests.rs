use std::{cell::RefCell, rc::Rc};

use ficus_backend::{
    event_log::core::event::event_hasher::default_class_extractor,
    features::analysis::patterns::{
        contexts::{ActivitiesDiscoveryContext, PatternsDiscoveryContext, PatternsDiscoveryStrategy},
        entry_points::{build_repeat_set_tree, find_repeats, PatternsKind},
        repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
    },
};

use crate::{
    analysis::patterns::utils::create_activity_name,
    test_core::simple_events_logs_provider::{create_log_from_taxonomy_of_patterns, create_maximal_repeats_log},
};

#[test]
fn test_repeat_sets_primitive_tandem_arrays() {
    let log = create_maximal_repeats_log();
    let context = PatternsDiscoveryContext::new(
        Rc::new(RefCell::new(log)),
        PatternsKind::PrimitiveTandemArrays(20),
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let repeats = find_repeats(&context);
    assert_eq!(get_first_trace_repeat(&repeats.borrow()), [(0, 4, 1), (3, 2, 4)]);
}

fn get_first_trace_repeat(repeats: &Vec<SubArrayWithTraceIndex>) -> Vec<(usize, usize, usize)> {
    repeats.into_iter().map(|array| array.dump()).collect()
}

#[test]
fn test_repeat_sets_super_maximal_repeats() {
    let log = create_maximal_repeats_log();
    let context = PatternsDiscoveryContext::new(
        Rc::new(RefCell::new(log)),
        PatternsKind::SuperMaximalRepeats,
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let repeats = find_repeats(&context);

    assert_eq!(
        get_first_trace_repeat(&repeats.borrow()),
        [
            (0, 1, 0),
            (2, 3, 0),
            (0, 4, 1),
            (0, 4, 2),
            (5, 1, 3),
            (7, 2, 3),
            (3, 3, 4),
            (9, 1, 4),
            (10, 2, 4)
        ]
    );
}

#[test]
fn test_repeat_sets_near_super_maximal_repeats() {
    let log = create_maximal_repeats_log();

    let repeats_context = PatternsDiscoveryContext::new(
        Rc::new(RefCell::new(log)),
        PatternsKind::NearSuperMaximalRepeats,
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let repeats = find_repeats(&repeats_context);

    assert_eq!(
        get_first_trace_repeat(&repeats.borrow()),
        [
            (0, 1, 0),
            (2, 1, 0),
            (2, 3, 0),
            (0, 4, 1),
            (0, 4, 2),
            (3, 1, 2),
            (3, 3, 4),
            (4, 1, 4),
            (9, 1, 4),
            (10, 2, 4)
        ]
    );
}

#[test]
fn test_repeat_set_tree() {
    let log = Rc::new(RefCell::new(create_log_from_taxonomy_of_patterns()));
    let context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::PrimitiveTandemArrays(20),
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let context =
        ActivitiesDiscoveryContext::new(context, 0, |sub_array| create_activity_name(&log.borrow(), sub_array));
    let repeats = build_repeat_set_tree(&context);

    assert_eq!(
        get_top_level_activities_event_classes(&repeats.borrow()),
        [[3102445089172487244, 8186225505942432243, 16993177596579750922]]
    );
}

fn get_top_level_activities_event_classes(activities: &Vec<Rc<RefCell<ActivityNode>>>) -> Vec<Vec<u64>> {
    activities
        .iter()
        .map(|node| {
            let mut vec: Vec<u64> = Vec::from_iter(node.borrow().event_classes.iter().map(|event_class| *event_class));
            vec.sort();
            vec
        })
        .collect()
}

#[test]
fn test_repeat_set_tree2() {
    let log = Rc::new(RefCell::new(create_maximal_repeats_log()));
    let context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::PrimitiveTandemArrays(20),
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let context =
        ActivitiesDiscoveryContext::new(context, 0, |sub_array| create_activity_name(&log.borrow(), sub_array));

    let repeats = build_repeat_set_tree(&context);

    assert_eq!(
        get_top_level_activities_event_classes(&repeats.borrow()),
        [[
            3102445089172487244,
            7393736521911212725,
            8186225505942432243,
            16993177596579750922
        ]]
    );
}

#[test]
fn test_repeat_set_tree3() {
    let log = Rc::new(RefCell::new(create_maximal_repeats_log()));
    let context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::SuperMaximalRepeats,
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let context =
        ActivitiesDiscoveryContext::new(context, 0, |sub_array| create_activity_name(&log.borrow(), sub_array));
    let repeats = build_repeat_set_tree(&context);

    assert_eq!(
        get_top_level_activities_event_classes(&repeats.borrow()),
        [
            vec![
                3102445089172487244,
                7393736521911212725,
                8186225505942432243,
                16993177596579750922
            ],
            vec![16528679900032520146]
        ]
    );
}

#[test]
fn test_repeat_set_tree4() {
    let log = Rc::new(RefCell::new(create_maximal_repeats_log()));
    let context = PatternsDiscoveryContext::new(
        Rc::clone(&log),
        PatternsKind::MaximalRepeats,
        PatternsDiscoveryStrategy::FromAllTraces,
        default_class_extractor,
    );

    let context =
        ActivitiesDiscoveryContext::new(context, 0, |sub_array| create_activity_name(&log.borrow(), sub_array));

    let repeats = build_repeat_set_tree(&context);

    assert_eq!(
        get_top_level_activities_event_classes(&repeats.borrow()),
        [
            vec![
                3102445089172487244,
                7393736521911212725,
                8186225505942432243,
                16993177596579750922
            ],
            vec![16528679900032520146]
        ]
    );
}
