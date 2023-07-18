use super::event::Event;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct EventsHolder<TEvent>
where
    TEvent: Event,
{
    events: Vec<Rc<RefCell<TEvent>>>,
}

impl<TEvent> EventsHolder<TEvent>
where
    TEvent: Event,
{
    pub fn new(events: Vec<Rc<RefCell<TEvent>>>) -> EventsHolder<TEvent> {
        EventsHolder { events }
    }

    pub fn get_events(&self) -> &Vec<Rc<RefCell<TEvent>>> {
        &self.events
    }

    pub fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&TEvent) -> bool,
    {
        let events = &mut self.events;
        for index in (0..events.len()).rev() {
            if predicate(&events[index].borrow()) {
                events.remove(index);
            }
        }
    }
}
