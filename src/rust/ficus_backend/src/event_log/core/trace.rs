use super::event::Event;
use std::rc::Rc;

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<Self::TEvent>>;
}
