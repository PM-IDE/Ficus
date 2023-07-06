use std::{collections::HashMap, cell::RefCell, rc::Rc};

use chrono::Utc;

use crate::event_log::core::{lifecycle::Lifecycle, event::{EventPayloadValue, Event}};


pub struct XesEventImpl {
    name: String,
    timestamp: chrono::DateTime<Utc>,
    lifecycle: Option<Lifecycle>,

    payload: Rc<RefCell<HashMap<String, EventPayloadValue>>>,
}

impl XesEventImpl {
    pub(crate) fn new(
        name: String,
        timestamp: chrono::DateTime<Utc>,
        lifecycle: Option<Lifecycle>,
        payload: Rc<RefCell<HashMap<String, EventPayloadValue>>>,
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

    fn get_payload(&self) -> Rc<RefCell<HashMap<String, EventPayloadValue>>> {
        Rc::clone(&self.payload)
    }
}
