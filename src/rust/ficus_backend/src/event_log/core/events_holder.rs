use crate::utils::hash_map_utils::increase_in_map;

use super::{event::Event, trace::TraceInfo};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct EventsHolder<TEvent>
where
    TEvent: Event,
{
    events: Vec<Rc<RefCell<TEvent>>>,
    events_sequence_info: Option<EventSequenceInfo>,
}

impl<TEvent> EventsHolder<TEvent>
where
    TEvent: Event,
{
    pub fn new(events: Vec<Rc<RefCell<TEvent>>>) -> EventsHolder<TEvent> {
        EventsHolder {
            events,
            events_sequence_info: None,
        }
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

    pub fn to_names_vec(&self) -> Vec<String> {
        let mut names = Vec::new();
        for event in &self.events {
            names.push(event.borrow().get_name().to_owned());
        }

        names
    }

    pub fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut TEvent),
    {
        for event in &self.events {
            mutator(&mut event.borrow_mut());
        }
    }

    pub fn get_event_sequence_info(&mut self) -> &EventSequenceInfo {
        //todo: invalidate on changes
        if self.events_sequence_info.is_some() {
            return self.events_sequence_info.as_ref().unwrap();
        }

        let info = EventSequenceInfo::new(&self);
        self.events_sequence_info = Some(info);

        self.events_sequence_info.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub struct EventSequenceInfo {
    events_counts: HashMap<String, usize>,
    events_count: usize,
}

impl TraceInfo for EventSequenceInfo {
    fn get_events_counts(&self) -> &HashMap<String, usize> {
        &self.events_counts
    }

    fn get_events_count(&self) -> usize {
        self.events_count
    }
}

impl EventSequenceInfo {
    fn new<TEvent>(events_holder: &EventsHolder<TEvent>) -> EventSequenceInfo
    where
        TEvent: Event,
    {
        let mut events_counts = HashMap::new();
        for event in events_holder.get_events() {
            increase_in_map(&mut events_counts, event.borrow().get_name());
        }

        EventSequenceInfo {
            events_counts,
            events_count: events_holder.get_events().len(),
        }
    }
}
