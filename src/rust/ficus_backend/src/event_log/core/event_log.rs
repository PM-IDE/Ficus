use std::rc::Rc;

use super::{event::Event, trace::Trace};

pub trait EventLog {
    type TEvent: Event;
    type TTrace: Trace<TEvent = Self::TEvent>;

    fn get_traces(&self) -> &Vec<Rc<Self::TTrace>>;
}