use chrono::{DateTime, Duration, Utc};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::event_log::core::{
    event::{Event, EventPayloadValue},
    event_log::EventLog,
    events_holder::EventsHolder,
    lifecycle::Lifecycle,
    trace::Trace,
    traces_holder::TracesHolder,
};

#[derive(Debug)]
pub struct SimpleEventLog {
    traces_holder: TracesHolder<SimpleTrace, SimpleEvent>,
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
}

impl EventLog for SimpleEventLog {
    type TEvent = SimpleEvent;
    type TTrace = SimpleTrace;

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
        &self.traces_holder.get_traces()
    }

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool,
    {
        self.traces_holder.filter_events_by(predicate);
    }
}

#[derive(Debug)]
pub struct SimpleTrace {
    events_holder: EventsHolder<SimpleEvent>,
}

impl Trace for SimpleTrace {
    type TEvent = SimpleEvent;

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
}

const TRACE_EVENT_START_DATE: DateTime<Utc> = DateTime::<Utc>::MIN_UTC;

impl SimpleTrace {
    pub fn new(raw_trace: &Vec<&str>) -> SimpleTrace {
        let mut events = Vec::new();
        let mut current_date = TRACE_EVENT_START_DATE;
        for raw_event in raw_trace {
            events.push(Rc::new(RefCell::new(SimpleEvent::new(raw_event, current_date))));
            current_date = current_date + Duration::seconds(1);
        }

        SimpleTrace {
            events_holder: EventsHolder::new(events),
        }
    }
}

#[derive(Debug)]
pub struct SimpleEvent {
    name: String,
    timestamp: DateTime<Utc>,
}

impl SimpleEvent {
    pub fn new(name: &str, stamp: DateTime<Utc>) -> SimpleEvent {
        SimpleEvent {
            name: name.to_owned(),
            timestamp: stamp,
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
}
