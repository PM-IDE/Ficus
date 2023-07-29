use std::cell::RefCell;
use std::rc::Rc;

use crate::event_log::core::{event_log::EventLog, trace::trace::Trace};

use super::{
    activity_instances::UndefActivityHandlingStrategy, entry_points::PatternsKind, repeat_sets::SubArrayWithTraceIndex,
};

pub struct PatternsDiscoveryContext<TClassExtractor, TLog>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
{
    pub log: Rc<RefCell<TLog>>,
    pub pattern_kind: PatternsKind,
    pub class_extractor: TClassExtractor,

    processed_log: Vec<Vec<u64>>,
}

impl<TClassExtractor, TLog> PatternsDiscoveryContext<TClassExtractor, TLog>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
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

pub struct ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub patterns_context: PatternsDiscoveryContext<TClassExtractor, TLog>,
    pub activity_level: usize,
    pub name_creator: TNameCreator,
}

impl<TClassExtractor, TLog, TNameCreator> ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub fn new(
        patterns_context: PatternsDiscoveryContext<TClassExtractor, TLog>,
        activity_level: usize,
        name_creator: TNameCreator,
    ) -> Self {
        Self {
            patterns_context,
            activity_level,
            name_creator,
        }
    }
}

pub struct ActivitiesInstancesDiscoveryContext<TClassExtractor, TLog, TNameCreator>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub activities_context: ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
    pub undef_events_handling_strategy: UndefActivityHandlingStrategy,
}

impl<TClassExtractor, TLog, TNameCreator> ActivitiesInstancesDiscoveryContext<TClassExtractor, TLog, TNameCreator>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub fn new(
        activities_context: ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
        strategy: UndefActivityHandlingStrategy,
    ) -> Self {
        Self {
            activities_context,
            undef_events_handling_strategy: strategy,
        }
    }
}
