use std::{cell::RefCell, rc::Rc};

use crate::event_log::{
    core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    simple::simple_event_log::SimpleEventLog,
};

use super::{
    repeat_sets::{
        build_repeat_set_tree_from_repeats, build_repeat_sets, create_new_log_from_activities_instances,
        extract_activities_instances, ActivityInTraceInfo, ActivityNode, RepeatsSetsDiscoveryContext,
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

pub fn find_patterns<TClassExtractor, TLog, TEvent>(
    context: &RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>,
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

pub struct RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>
where
    TLog: EventLog + EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
{
    pub log: Rc<RefCell<TLog>>,
    pub pattern_kind: PatternsKind,
    pub class_extractor: TClassExtractor,

    processed_log: Vec<Vec<u64>>,
}

impl<TClassExtractor, TLog, TEvent> RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
{
    pub fn get_processed_log(&self) -> &Vec<Vec<u64>> {
        &self.processed_log
    }

    pub fn new(log: Rc<RefCell<TLog>>, pattern_kind: PatternsKind, class_extractor: TClassExtractor) -> Self {
        let mut processed_log = vec![];
        for trace in log.borrow().get_traces() {
            let mut processed_trace = vec![];
            for event in trace.borrow().get_events() {
                processed_trace.push((&class_extractor)(&event.borrow()));
            }

            processed_log.push(processed_trace);
        }

        Self {
            log,
            pattern_kind,
            class_extractor,
            processed_log,
        }
    }
}

pub fn find_repeats<TClassExtractor, TLog, TEvent>(
    context: &RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>,
) -> Rc<RefCell<Vec<SubArrayWithTraceIndex>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
{
    let patterns = find_patterns(context);
    build_repeat_sets(context.get_processed_log(), &patterns)
}

pub struct ActivityDiscoveryContext<TNameCreator>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub patterns_kind: PatternsKind,
    pub repeat_sets_context: RepeatsSetsDiscoveryContext<TNameCreator>,
}

impl<TNameCreator> ActivityDiscoveryContext<TNameCreator>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub fn new(repeat_sets_context: RepeatsSetsDiscoveryContext<TNameCreator>, patterns_kind: PatternsKind) -> Self {
        Self {
            patterns_kind,
            repeat_sets_context,
        }
    }
}

pub fn build_repeat_set_tree<TClassExtractor, TLog, TEvent, TNameCreator>(
    repeats_context: &RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>,
    activities_context: &ActivityDiscoveryContext<TNameCreator>,
) -> Rc<RefCell<Vec<Rc<RefCell<ActivityNode>>>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeats = find_repeats(repeats_context);
    build_repeat_set_tree_from_repeats(
        repeats_context.get_processed_log(),
        &repeats,
        &activities_context.repeat_sets_context,
    )
}

pub fn discover_activities_instances<TNameCreator, TClassExtractor, TLog, TEvent>(
    repeats_context: &RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>,
    context: &ActivityDiscoveryContext<TNameCreator>,
) -> Rc<RefCell<Vec<Vec<ActivityInTraceInfo>>>>
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeat_set_tree = build_repeat_set_tree(repeats_context, context);
    extract_activities_instances(repeats_context.get_processed_log(), repeat_set_tree, true)
}

pub struct ActivitiesInstancesDiscovery<TNameCreator>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    activities_context: ActivityDiscoveryContext<TNameCreator>,
    undef_events_handling_strategy: UndefActivityHandlingStrategy,
}

impl<TNameCreator> ActivitiesInstancesDiscovery<TNameCreator>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub fn new(
        strategy: UndefActivityHandlingStrategy,
        activities_context: ActivityDiscoveryContext<TNameCreator>,
    ) -> Self {
        Self {
            activities_context,
            undef_events_handling_strategy: strategy,
        }
    }
}

pub fn discover_activities_and_create_new_log<TClassExtractor, TLog, TEvent, TNameCreator>(
    repeats_context: &RepeatsDiscoveryContext<TClassExtractor, TLog, TEvent>,
    context: &ActivitiesInstancesDiscovery<TNameCreator>,
) -> Rc<RefCell<SimpleEventLog>>
where
    TLog: EventLog<TEvent = TEvent> + EventLog,
    TEvent: Event,
    TClassExtractor: Fn(&TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let activity_instances = discover_activities_instances(repeats_context, &context.activities_context);
    let activity_instances = activity_instances.borrow();

    create_new_log_from_activities_instances(
        &repeats_context.log,
        &activity_instances,
        &context.undef_events_handling_strategy,
    )
}
