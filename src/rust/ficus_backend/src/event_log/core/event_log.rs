use std::rc::Rc;

use super::event::Event;

pub trait EventLog {
    type TEvent: Event;
    type TTrace: Trace<TEvent = Self::TEvent>;

    fn get_traces(&self) -> &Vec<Rc<Self::TTrace>>;
}

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<Self::TEvent>>;
}