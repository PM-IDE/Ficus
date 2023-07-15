use chrono::{DateTime, Duration, Utc};
use std::{collections::HashMap, rc::Rc};

use crate::event_log::core::{
    event::{Event, EventPayloadValue},
    event_log::EventLog,
    lifecycle::Lifecycle,
    trace::Trace,
};

pub struct SimpleEventLog {
    traces: Vec<Rc<SimpleTrace>>,
}

impl SimpleEventLog {
    fn new(raw_event_log: &Vec<Vec<&str>>) -> SimpleEventLog {
        let mut traces = Vec::new();
        for raw_trace in raw_event_log {
            traces.push(Rc::new(SimpleTrace::new(raw_trace)));
        }

        SimpleEventLog { traces }
    }
}

impl EventLog for SimpleEventLog {
    type TEvent = SimpleEvent;
    type TTrace = SimpleTrace;

    fn get_traces(&self) -> &Vec<Rc<Self::TTrace>> {
        &self.traces
    }
}

pub struct SimpleTrace {
    events: Vec<Rc<SimpleEvent>>,
}

impl Trace for SimpleTrace {
    type TEvent = SimpleEvent;

    fn get_events(&self) -> &Vec<Rc<Self::TEvent>> {
        &self.events
    }
}

const TRACE_EVENT_START_DATE: DateTime<Utc> = DateTime::<Utc>::MIN_UTC;

impl SimpleTrace {
    fn new(raw_trace: &Vec<&str>) -> SimpleTrace {
        let mut events = Vec::new();
        let mut current_date = TRACE_EVENT_START_DATE;
        for raw_event in raw_trace {
            events.push(Rc::new(SimpleEvent::new(raw_event, current_date)));
            current_date = current_date + Duration::seconds(1);
        }

        SimpleTrace { events }
    }
}

pub struct SimpleEvent {
    name: String,
    timestamp: DateTime<Utc>,
}

impl SimpleEvent {
    fn new(name: &str, stamp: DateTime<Utc>) -> SimpleEvent {
        SimpleEvent {
            name: name.to_owned(),
            timestamp: stamp,
        }
    }
}

impl Event for SimpleEvent {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
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
}
