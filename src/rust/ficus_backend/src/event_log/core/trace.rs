use super::event::Event;
use std::{cell::RefCell, rc::Rc};

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>>;

    fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;
}
