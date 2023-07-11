use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chrono::Utc;

use crate::{event_log::core::{
    event::{Event, EventPayloadValue},
    lifecycle::Lifecycle,
}, utils::vec_utils};

pub struct XesEventImpl {
    name: String,
    timestamp: chrono::DateTime<Utc>,
    lifecycle: Option<Lifecycle>,

    payload: HashMap<String, EventPayloadValue>,
}

impl XesEventImpl {
    pub(crate) fn new(
        name: String,
        timestamp: chrono::DateTime<Utc>,
        lifecycle: Option<Lifecycle>,
        payload: HashMap<String, EventPayloadValue>,
    ) -> XesEventImpl {
        XesEventImpl {
            name: name.to_owned(),
            timestamp,
            lifecycle,
            payload,
        }
    }
}

impl Event for XesEventImpl {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_timestamp(&self) -> chrono::DateTime<Utc> {
        self.timestamp
    }

    fn get_lifecycle(&self) -> Option<Lifecycle> {
        match self.lifecycle.as_ref() {
            Some(value) => Some(*value),
            None => None,
        }
    }

    fn get_payload_map(&self) -> &HashMap<String, EventPayloadValue> {
        &self.payload
    }

    fn get_ordered_payload(&self) -> Vec<(&String, &EventPayloadValue)> {
        let mut payload = Vec::new();
        for (key, value) in self.get_payload_map() {
            payload.push((key, value));
        }

        vec_utils::sort_by_first(&mut payload);

        payload
    }
}