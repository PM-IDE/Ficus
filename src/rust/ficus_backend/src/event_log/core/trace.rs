use super::event::Event;
use std::{cell::RefCell, rc::Rc};

pub trait Trace {
    type TEvent: Event;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>>;
    fn to_names_vec(&self) -> Vec<String>;

    fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent);
}
