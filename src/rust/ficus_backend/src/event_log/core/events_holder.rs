use lazycell::LazyCell;

use crate::{
    event_log::xes::reader::xes_log_trace_reader::TraceXesEventLogIterator,
    utils::hash_map_utils::{add_to_list_in_map, increase_in_map},
};

use super::{
    event::Event,
    trace::{TraceEventsPositions, TraceInfo},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct EventsHolder<TEvent>
where
    TEvent: Event,
{
    events: Vec<Rc<RefCell<TEvent>>>,
    events_sequence_info: LazyCell<EventSequenceInfo>,
    events_positions: LazyCell<EventsPositions>,
}

impl<TEvent> EventsHolder<TEvent>
where
    TEvent: Event,
{
    pub fn new(events: Vec<Rc<RefCell<TEvent>>>) -> EventsHolder<TEvent> {
        EventsHolder {
            events,
            events_sequence_info: LazyCell::new(),
            events_positions: LazyCell::new(),
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

    pub fn get_or_create_event_sequence_info(&mut self) -> &EventSequenceInfo {
        if !self.events_sequence_info.filled() {
            self.events_sequence_info.fill(EventSequenceInfo::new(self)).ok();
        }

        self.events_sequence_info.borrow().unwrap()
    }

    pub fn get_or_create_events_positions(&mut self) -> &EventsPositions {
        if !self.events_positions.filled() {
            self.events_positions.fill(EventsPositions::new(self));
        }

        self.events_positions.borrow().unwrap()
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

#[derive(Debug)]
pub struct EventsPositions {
    positions: HashMap<String, Vec<usize>>,
}

impl EventsPositions {
    pub fn new<TEvent>(events: &EventsHolder<TEvent>) -> EventsPositions
    where
        TEvent: Event,
    {
        let mut positions = HashMap::new();
        let mut index = 0;

        for event in events.get_events() {
            add_to_list_in_map(&mut positions, event.borrow().get_name(), index);
            index += 1;
        }

        EventsPositions { positions }
    }
}

impl TraceEventsPositions for EventsPositions {
    fn get_event_positions(&self, event_class: &String) -> Option<&Vec<usize>> {
        self.positions.get(event_class)
    }
}
