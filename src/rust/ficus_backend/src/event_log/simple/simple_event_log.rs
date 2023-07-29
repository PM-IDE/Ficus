use chrono::{DateTime, Duration, Utc};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    event_log::core::{
        event::event::{Event, EventPayloadValue},
        event::event_hasher::EventHasher,
        event::events_holder::{EventSequenceInfo, EventsHolder, EventsPositions},
        event::lifecycle::Lifecycle,
        event_log::EventLog,
        trace::trace::Trace,
        trace::traces_holder::TracesHolder,
    },
    utils::user_data::UserData,
};

#[derive(Debug)]
pub struct SimpleEventLog {
    traces_holder: TracesHolder<SimpleTrace, SimpleEvent>,
}

impl SimpleEventLog {
    pub fn empty() -> Self {
        Self {
            traces_holder: TracesHolder::empty(),
        }
    }

    pub fn new(raw_event_log: &Vec<Vec<&str>>) -> SimpleEventLog {
        let mut traces = Vec::new();
        for raw_trace in raw_event_log {
            traces.push(Rc::new(RefCell::new(SimpleTrace::new(raw_trace))));
        }

        SimpleEventLog {
            traces_holder: TracesHolder::new(traces),
        }
    }

    pub fn to_raw_vector(&self) -> Vec<Vec<String>> {
        let mut raw_log = Vec::new();
        for trace in self.traces_holder.get_traces() {
            let mut events = Vec::new();
            for event in trace.borrow().get_events() {
                events.push(event.borrow().get_name().to_owned());
            }

            raw_log.push(events);
        }

        raw_log
    }

    pub fn push(&mut self, trace: Rc<RefCell<<SimpleEventLog as EventLog>::TTrace>>) {
        self.traces_holder.push(trace);
    }
}

impl EventLog for SimpleEventLog {
    type TEvent = SimpleEvent;
    type TTrace = SimpleTrace;
    type TTraceInfo = EventSequenceInfo;

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
        &self.traces_holder.get_traces()
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

    fn to_hashes_event_log<THasher>(&self) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<Self::TEvent>,
    {
        self.traces_holder.to_hashes_vectors::<THasher>()
    }
}

#[derive(Debug)]
pub struct SimpleTrace {
    events_holder: EventsHolder<SimpleEvent>,
}

impl Trace for SimpleTrace {
    type TEvent = SimpleEvent;
    type TTraceInfo = EventSequenceInfo;
    type TTracePositions = EventsPositions;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>> {
        &self.events_holder.get_events()
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
    name: String,
    timestamp: DateTime<Utc>,
    user_data: UserData,
}

impl SimpleEvent {
    pub fn new(name: &str, stamp: DateTime<Utc>) -> Self {
        Self {
            name: name.to_owned(),
            timestamp: stamp,
            user_data: UserData::new(),
        }
    }

    pub fn new_with_min_date(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            timestamp: DateTime::<Utc>::MIN_UTC,
            user_data: UserData::new(),
        }
    }
}

impl Event for SimpleEvent {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    fn get_lifecycle(&self) -> Option<Lifecycle> {
        panic!("Not supported")
    }

    fn get_payload_map(&self) -> &HashMap<String, EventPayloadValue> {
        panic!("Not supported")
    }

    fn get_ordered_payload(&self) -> Vec<(&String, &EventPayloadValue)> {
        panic!("Not supported")
    }

    fn set_name(&mut self, new_name: &String) {
        self.name = new_name.to_owned();
    }

    fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>) {
        self.timestamp = new_timestamp;
    }

    fn set_lifecycle(&mut self, _: Lifecycle) {
        panic!("Not supported")
    }

    fn add_or_update_payload(&mut self, _: String, _: EventPayloadValue) {
        panic!("Not supported")
    }

    fn get_user_data(&self) -> &UserData {
        &self.user_data
    }
}
