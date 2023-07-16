use std::{rc::Rc, cell::RefCell};

use super::{event::Event, trace::Trace};

pub trait EventLog {
    type TEvent: Event;
    type TTrace: Trace<TEvent = Self::TEvent>;

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>>;
}
