use std::{cell::RefCell, rc::Rc};

use super::{event::Event, trace::Trace};

pub trait EventLog {
    type TEvent: Event;
    type TTrace: Trace<TEvent = Self::TEvent>;

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>>;

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent);
}
