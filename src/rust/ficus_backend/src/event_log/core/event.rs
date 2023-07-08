use super::lifecycle::Lifecycle;
use chrono::{Utc, DateTime};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub enum EventPayloadValue {
    Date(DateTime<Utc>),
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32),
}

impl ToString for EventPayloadValue {
    fn to_string(&self) -> String {
        match self {
            EventPayloadValue::Date(date) => date.to_rfc3339(),
            EventPayloadValue::String(string) => string.to_owned(),
            EventPayloadValue::Boolean(bool) => bool.to_string(),
            EventPayloadValue::Int(int) => int.to_string(),
            EventPayloadValue::Float(float) => float.to_string(),
        }
    }
}

pub trait Event {
    fn get_name(&self) -> &str;
    fn get_timestamp(&self) -> chrono::DateTime<Utc>;
    fn get_lifecycle(&self) -> Option<Lifecycle>;
    fn get_payload(&self) -> Rc<RefCell<HashMap<String, EventPayloadValue>>>;
}