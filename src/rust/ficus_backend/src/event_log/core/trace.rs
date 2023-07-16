use super::event::Event;
use std::{rc::Rc, cell::RefCell};

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>>;
}
