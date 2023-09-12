use chrono::{DateTime, Duration, Utc};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    event_log::core::{
        event::event::{Event, EventPayloadValue},
        event::event_hasher::EventHasher,
        event::events_holder::{EventSequenceInfo, EventsHolder, EventsPositions},
        event::{event_base::EventBase, lifecycle::Lifecycle},
        event_log::EventLog,
        trace::trace::Trace,
        trace::traces_holder::TracesHolder,
    },
    utils::user_data::user_data::UserDataImpl,
};

#[derive(Debug)]
pub struct SimpleEventLog {
    traces_holder: TracesHolder<SimpleTrace>,
}

impl SimpleEventLog {
    pub fn new(raw_event_log: &Vec<Vec<&str>>) -> SimpleEventLog {
        let mut traces = Vec::new();
        for raw_trace in raw_event_log {
            traces.push(Rc::new(RefCell::new(SimpleTrace::new(raw_trace))));
        }

        SimpleEventLog {
            traces_holder: TracesHolder::new(traces),
        }
    }

    pub fn push(&mut self, trace: Rc<RefCell<<SimpleEventLog as EventLog>::TTrace>>) {
        self.traces_holder.push(trace);
    }
}

impl Clone for SimpleEventLog {
    fn clone(&self) -> Self {
        Self {
            traces_holder: self.traces_holder.clone(),
        }
    }
}

impl EventLog for SimpleEventLog {
    type TEvent = SimpleEvent;
    type TTrace = SimpleTrace;
    type TTraceInfo = EventSequenceInfo;

    fn empty() -> Self {
        Self {
            traces_holder: TracesHolder::empty(),
        }
    }

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
        &self.traces_holder.get_traces()
    }

    fn push(&mut self, trace: Rc<RefCell<Self::TTrace>>) {
        self.traces_holder.push(trace);
    }

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool,
    {
        self.traces_holder.filter_events_by(predicate);
    }

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent),
    {
        self.traces_holder.mutate_events(mutator);
    }

    fn to_hashes_event_log<THasher>(&self, hasher: &THasher) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<Self::TEvent>,
    {
        self.traces_holder.to_hashes_vectors(hasher)
    }

    fn filter_traces(&mut self, predicate: &impl Fn(&Self::TTrace, &usize) -> bool) {
        self.traces_holder.filter_traces(predicate);
    }

    fn to_raw_vector(&self) -> Vec<Vec<String>> {
        self.traces_holder.to_raw_vector()
    }
}

#[derive(Debug)]
pub struct SimpleTrace {
    events_holder: EventsHolder<SimpleEvent>,
}

impl Clone for SimpleTrace {
    fn clone(&self) -> Self {
        Self {
            events_holder: self.events_holder.clone(),
        }
    }
}

impl Trace for SimpleTrace {
    type TEvent = SimpleEvent;
    type TTraceInfo = EventSequenceInfo;
    type TTracePositions = EventsPositions;

    fn empty() -> Self {
        Self {
            events_holder: EventsHolder::empty(),
        }
    }

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>> {
        &self.events_holder.get_events()
    }

    fn push(&mut self, event: Rc<RefCell<Self::TEvent>>) {
        self.events_holder.push(event);
    }

    fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool,
    {
        self.events_holder.remove_events_by(predicate);
    }

    fn to_names_vec(&self) -> Vec<String> {
        self.events_holder.to_names_vec()
    }

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent),
    {
        self.events_holder.mutate_events(mutator);
    }

    fn get_or_create_trace_info(&mut self) -> &Self::TTraceInfo {
        self.events_holder.get_or_create_event_sequence_info()
    }

    fn get_or_create_events_positions(&mut self) -> &Self::TTracePositions {
        self.events_holder.get_or_create_events_positions()
    }
}

const TRACE_EVENT_START_DATE: DateTime<Utc> = DateTime::<Utc>::MIN_UTC;

impl SimpleTrace {
    pub fn empty() -> Self {
        Self {
            events_holder: EventsHolder::empty(),
        }
    }

    pub fn new(raw_trace: &Vec<&str>) -> Self {
        let mut events = Vec::new();
        let mut current_date = TRACE_EVENT_START_DATE;
        for raw_event in raw_trace {
            events.push(Rc::new(RefCell::new(SimpleEvent::new(raw_event, current_date))));
            current_date = current_date + Duration::seconds(1);
        }

        Self {
            events_holder: EventsHolder::new(events),
        }
    }

    pub fn push(&mut self, event: Rc<RefCell<<SimpleTrace as Trace>::TEvent>>) {
        self.events_holder.push(event);
    }
}

#[derive(Debug)]
pub struct SimpleEvent {
    event_base: EventBase,
}

impl SimpleEvent {
    pub fn new(name: &str, timestamp: DateTime<Utc>) -> Self {
        Self {
            event_base: EventBase::new(name.to_owned(), timestamp),
        }
    }

    pub fn new_with_min_date(name: &str) -> Self {
        Self {
            event_base: EventBase::new(name.to_owned(), DateTime::<Utc>::MIN_UTC),
        }
    }
}

impl Event for SimpleEvent {
    fn get_name(&self) -> &String {
        &self.event_base.name
    }

    fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.event_base.timestamp
    }

    fn get_lifecycle(&self) -> Option<Lifecycle> {
        panic!("Not supported")
    }

    fn get_payload_map(&self) -> Option<&HashMap<String, EventPayloadValue>> {
        panic!("Not supported")
    }

    fn get_ordered_payload(&self) -> Vec<(&String, &EventPayloadValue)> {
        panic!("Not supported")
    }

    fn set_name(&mut self, new_name: &String) {
        self.event_base.name = new_name.to_owned();
    }

    fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>) {
        self.event_base.timestamp = new_timestamp;
    }

    fn set_lifecycle(&mut self, _: Lifecycle) {
        panic!("Not supported")
    }

    fn add_or_update_payload(&mut self, _: String, _: EventPayloadValue) {
        panic!("Not supported")
    }

    fn get_user_data(&mut self) -> &mut UserDataImpl {
        self.event_base.user_data_holder.get_mut()
    }
}

impl Clone for SimpleEvent {
    fn clone(&self) -> Self {
        Self {
            event_base: self.event_base.clone(),
        }
    }
}
