use std::{cell::RefCell, rc::Rc};

use crate::event_log::{
    core::{event::event::Event, event_log::EventLog},
    simple::simple_event_log::SimpleEventLog,
};

use super::{
    contexts::{ActivitiesDiscoveryContext, ActivitiesInstancesDiscoveryContext, PatternsDiscoveryContext},
    repeat_sets::{
        build_repeat_set_tree_from_repeats, build_repeat_sets, create_new_log_from_activities_instances,
        extract_activities_instances, ActivityInTraceInfo, ActivityNode, SubArrayWithTraceIndex,
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

pub fn find_patterns<TClassExtractor, TLog, TEvent>(
    context: &PatternsDiscoveryContext<TClassExtractor, TLog, TEvent>,
) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
{
    let log = context.get_processed_log();
    match &context.pattern_kind {
        PatternsKind::MaximalRepeats => find_maximal_repeats(log),
        PatternsKind::SuperMaximalRepeats => find_super_maximal_repeats(log),
        PatternsKind::NearSuperMaximalRepeats => find_near_super_maximal_repeats(log),
        PatternsKind::PrimitiveTandemArrays(length) => find_primitive_tandem_arrays(log, *length),
        PatternsKind::MaximalTandemArrays(length) => find_maximal_tandem_arrays(log, *length),
    }
}

pub fn find_repeats<TClassExtractor, TLog, TEvent>(
    context: &PatternsDiscoveryContext<TClassExtractor, TLog, TEvent>,
) -> Rc<RefCell<Vec<SubArrayWithTraceIndex>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
{
    let patterns = find_patterns(context);
    build_repeat_sets(context.get_processed_log(), &patterns)
}

pub fn build_repeat_set_tree<TClassExtractor, TLog, TEvent, TNameCreator>(
    activities_context: &ActivitiesDiscoveryContext<TClassExtractor, TLog, TEvent, TNameCreator>,
) -> Rc<RefCell<Vec<Rc<RefCell<ActivityNode>>>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeats = find_repeats(&activities_context.patterns_context);
    build_repeat_set_tree_from_repeats(
        activities_context.patterns_context.get_processed_log(),
        &repeats,
        &activities_context,
    )
}

pub fn discover_activities_instances<TClassExtractor, TLog, TEvent, TNameCreator>(
    activities_context: &ActivitiesDiscoveryContext<TClassExtractor, TLog, TEvent, TNameCreator>,
) -> Rc<RefCell<Vec<Vec<ActivityInTraceInfo>>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeat_set_tree = build_repeat_set_tree(activities_context);
    extract_activities_instances(
        activities_context.patterns_context.get_processed_log(),
        repeat_set_tree,
        true,
    )
}

pub fn discover_activities_and_create_new_log<TClassExtractor, TLog, TEvent, TNameCreator>(
    context: &ActivitiesInstancesDiscoveryContext<TClassExtractor, TLog, TEvent, TNameCreator>,
) -> Rc<RefCell<SimpleEventLog>>
where
    TLog: EventLog<TEvent = TEvent> + EventLog,
    TEvent: Event + 'static,
    TClassExtractor: Fn(&TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let activity_instances = discover_activities_instances(&context.activities_context);
    let activity_instances = activity_instances.borrow();

    create_new_log_from_activities_instances(
        &context.activities_context.patterns_context.log,
        &activity_instances,
        &context.undef_events_handling_strategy,
    )
}
