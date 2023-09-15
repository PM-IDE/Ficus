use crate::utils::user_data::user_data::UserDataImpl;

use super::lifecycle::Lifecycle;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

pub trait Event: Clone {
    fn name(&self) -> &String;
    fn timestamp(&self) -> &DateTime<Utc>;
    fn lifecycle(&self) -> Option<Lifecycle>;
    fn payload_map(&self) -> Option<&HashMap<String, EventPayloadValue>>;
    fn ordered_payload(&self) -> Vec<(&String, &EventPayloadValue)>;
    fn user_data(&mut self) -> &mut UserDataImpl;

    fn set_name(&mut self, new_name: &String);
    fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>);
    fn set_lifecycle(&mut self, lifecycle: Lifecycle);
    fn add_or_update_payload(&mut self, key: String, value: EventPayloadValue);
}
