use std::{collections::HashMap, str::FromStr, rc::Rc, cell::RefCell};

use chrono::Utc;

#[derive(Debug)]
pub enum EventPayloadValue {
    Date(chrono::DateTime<Utc>),
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32)
}

#[derive(Debug, Clone, Copy)]
pub enum Lifecycle {
    XesStandardLifecycle(XesStandardLifecycle)
}

#[derive(Debug, Clone, Copy)]
pub enum XesStandardLifecycle {
    Schedule,
    Start,
    Complete,
    Unknown
}

impl FromStr for XesStandardLifecycle {
    type Err = ParseXesStandardLifecycleError;

    fn from_str(s: &str) -> Result<XesStandardLifecycle, Self::Err> {
        match s {
            "schedule" => Ok(XesStandardLifecycle::Schedule),
            "start" => Ok(XesStandardLifecycle::Start),
            "complete" => Ok(XesStandardLifecycle::Complete),
            "unknown" => Ok(XesStandardLifecycle::Unknown),
            _ => Err(ParseXesStandardLifecycleError)
        }
    }
}

pub struct ParseXesStandardLifecycleError;

pub trait Event {
    fn get_name(&self) -> &str;
    fn get_timestamp(&self) -> chrono::DateTime<Utc>;
    fn get_lifecycle(&self) -> Option<Lifecycle>;
    fn get_payload(&self) -> Rc<RefCell<HashMap<String, EventPayloadValue>>>;
}

pub struct EventImpl {
    name: String,
    timestamp: chrono::DateTime<Utc>,
    lifecycle: Option<Lifecycle>,

    payload: Rc<RefCell<HashMap<String, EventPayloadValue>>>
}

impl EventImpl {
    pub(crate) fn new(name: String,
                      timestamp: chrono::DateTime<Utc>,
                      lifecycle: Option<Lifecycle>,
                      payload: Rc<RefCell<HashMap<String, EventPayloadValue>>>) -> EventImpl {
        EventImpl { name: name.to_owned(), timestamp, lifecycle, payload }
    }
}

impl Event for EventImpl {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_timestamp(&self) -> chrono::DateTime<Utc> {
        self.timestamp
    }

    fn get_lifecycle(&self) -> Option<Lifecycle> {
        match self.lifecycle.as_ref() {
            Some(value) => Some(*value),
            None => None
        }
    }

    fn get_payload(&self) -> Rc<RefCell<HashMap<String, EventPayloadValue>>> {
        Rc::clone(&self.payload)
    }
}