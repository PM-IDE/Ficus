use super::event::Event;
use std::{cell::RefCell, rc::Rc};

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>>;
}
