use std::{collections::HashMap, str::FromStr};

use chrono::Utc;

pub enum EventPayloadValue {
    Date(chrono::DateTime<Utc>),
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32)
}

#[derive(Clone, Copy)]
pub enum Lifecycle {
    XesStandardLifecycle(XesStandardLifecycle)
}

#[derive(Clone, Copy)]
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
}

pub struct EventImpl {
    name: String,
    timestamp: chrono::DateTime<Utc>,
    lifecycle: Option<Lifecycle>,

    payload: HashMap<String, EventPayloadValue>
}

impl EventImpl {
    pub(crate) fn new(name: String,
                      timestamp: chrono::DateTime<Utc>,
                      lifecycle: Option<Lifecycle>,
                      payload: HashMap<String, EventPayloadValue>) -> EventImpl {
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
}