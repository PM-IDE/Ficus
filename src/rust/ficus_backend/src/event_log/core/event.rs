use super::lifecycle::Lifecycle;
use chrono::Utc;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub enum EventPayloadValue {
    Date(chrono::DateTime<Utc>),
    String(String),
    Boolean(bool),
    Int(i32),
    Float(f32),
}

pub trait Event {
    fn get_name(&self) -> &str;
    fn get_timestamp(&self) -> chrono::DateTime<Utc>;
    fn get_lifecycle(&self) -> Option<Lifecycle>;
    fn get_payload(&self) -> Rc<RefCell<HashMap<String, EventPayloadValue>>>;
}
