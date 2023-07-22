use super::{
    event::Event,
    event_hasher::EventHasher,
    trace::{Trace, TraceInfo},
};
use std::{cell::RefCell, rc::Rc};

pub trait EventLog {
    type TEvent: Event;
    type TTraceInfo: TraceInfo;
    type TTrace: Trace<TEvent = Self::TEvent, TTraceInfo = Self::TTraceInfo>;

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>>;

    fn to_hashes_event_log<THasher>(&self) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<Self::TEvent>;

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent);
}
