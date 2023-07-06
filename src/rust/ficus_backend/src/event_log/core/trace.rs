use std::rc::Rc;
use super::event::Event;

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<Self::TEvent>>;
}