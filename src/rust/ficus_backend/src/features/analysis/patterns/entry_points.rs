use std::{cell::RefCell, rc::Rc};

use crate::event_log::{core::event_log::EventLog, simple::simple_event_log::SimpleEventLog};

use super::{
    repeat_sets::{
        build_repeat_set_tree_from_repeats, build_repeat_sets, create_new_log_from_activities_instances,
        extract_activities_instances, ActivitiesDiscoveryContext, ActivityInTraceInfo, ActivityNode,
        SubArrayWithTraceIndex, UndefActivityHandlingStrategy,
    },
    repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
    tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
};

pub enum PatternsKind {
    PrimitiveTandemArrays(usize),
    MaximalTandemArrays(usize),

    MaximalRepeats,
    SuperMaximalRepeats,
    NearSuperMaximalRepeats,
}

pub fn find_patterns(log: &Vec<Vec<u64>>, patterns_kind: PatternsKind) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    match patterns_kind {
        PatternsKind::MaximalRepeats => find_maximal_repeats(log),
        PatternsKind::SuperMaximalRepeats => find_super_maximal_repeats(log),
        PatternsKind::NearSuperMaximalRepeats => find_near_super_maximal_repeats(log),
        PatternsKind::PrimitiveTandemArrays(length) => find_primitive_tandem_arrays(log, length),
        PatternsKind::MaximalTandemArrays(length) => find_maximal_tandem_arrays(log, length),
    }
}

pub fn find_repeats(log: &Vec<Vec<u64>>, patterns_kind: PatternsKind) -> Rc<RefCell<Vec<SubArrayWithTraceIndex>>> {
    let patterns = find_patterns(log, patterns_kind);
    build_repeat_sets(log, &patterns)
}

pub fn build_repeat_set_tree<TNameCreator>(
    log: &Vec<Vec<u64>>,
    patterns_kind: PatternsKind,
    context: ActivitiesDiscoveryContext<TNameCreator>,
) -> Rc<RefCell<Vec<Rc<RefCell<ActivityNode>>>>>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeats = find_repeats(log, patterns_kind);
    build_repeat_set_tree_from_repeats(log, &repeats, context)
}

pub fn discover_activities_instances<TNameCreator>(
    log: &Vec<Vec<u64>>,
    patterns_kind: PatternsKind,
    context: ActivitiesDiscoveryContext<TNameCreator>,
) -> Rc<RefCell<Vec<Vec<ActivityInTraceInfo>>>>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeat_set_tree = build_repeat_set_tree(log, patterns_kind, context);
    extract_activities_instances(log, repeat_set_tree, true)
}

pub fn discover_activities_and_create_new_log<TLog, TNameCreator>(
    original_log: &TLog,
    log: &Vec<Vec<u64>>,
    patterns_kind: PatternsKind,
    context: ActivitiesDiscoveryContext<TNameCreator>,
    undef_events_handling_strategy: UndefActivityHandlingStrategy,
) -> Rc<RefCell<SimpleEventLog>>
where
    TLog: EventLog,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let activity_instances = discover_activities_instances(log, patterns_kind, context);
    let activity_instances = activity_instances.borrow();

    create_new_log_from_activities_instances(original_log, &activity_instances, undef_events_handling_strategy)
}
