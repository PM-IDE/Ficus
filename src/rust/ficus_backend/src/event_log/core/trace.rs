use super::event::Event;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait Trace {
    type TEvent: Event;
    type TTraceInfo: TraceInfo;
    type TTracePositions: TraceEventsPositions;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>>;
    fn to_names_vec(&self) -> Vec<String>;
    fn get_or_create_trace_info(&mut self) -> &Self::TTraceInfo;
    fn get_or_create_events_positions(&mut self) -> &Self::TTracePositions;

    fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent);
}

pub trait TraceInfo {
    fn get_events_counts(&self) -> &HashMap<String, usize>;
    fn get_events_count(&self) -> usize;
}

pub trait TraceEventsPositions {
    fn get_event_positions(&self, event_class: &String) -> Option<&Vec<usize>>;
}
