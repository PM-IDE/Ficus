use std::{collections::HashMap, hash::Hash};

use chrono::{DateTime, Utc};

use crate::{
    event_log::core::event::{
        event::{Event, EventPayloadValue},
        event_base::EventBase,
        lifecycle::Lifecycle,
    },
    utils::{user_data::user_data::UserDataImpl, vec_utils},
};

pub struct XesEventImpl {
    event_base: EventBase,
    lifecycle: Option<Lifecycle>,
    payload: Option<HashMap<String, EventPayloadValue>>,
}

impl XesEventImpl {
    pub fn new(
        name: String,
        timestamp: DateTime<Utc>,
        lifecycle: Option<Lifecycle>,
        payload: HashMap<String, EventPayloadValue>,
    ) -> Self {
        Self {
            event_base: EventBase::new(name, timestamp),
            lifecycle,
            payload: Some(payload),
        }
    }

    pub fn new_min_date(name: String) -> Self {
        Self {
            event_base: EventBase::new(name, DateTime::<Utc>::MIN_UTC),
            lifecycle: None,
            payload: None,
        }
    }

    pub fn new_with_date(name: String, stamp: DateTime<Utc>) -> Self {
        Self {
            event_base: EventBase::new(name, stamp),
            lifecycle: None,
            payload: None,
        }
    }
}

impl Event for XesEventImpl {
    fn get_name(&self) -> &String {
        &self.event_base.name
    }

    fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.event_base.timestamp
    }

    fn get_lifecycle(&self) -> Option<Lifecycle> {
        match self.lifecycle.as_ref() {
            Some(value) => Some(*value),
            None => None,
        }
    }

    fn get_payload_map(&self) -> Option<&HashMap<String, EventPayloadValue>> {
        self.payload.as_ref()
    }

    fn get_ordered_payload(&self) -> Vec<(&String, &EventPayloadValue)> {
        let mut payload = Vec::new();
        if let Some(payload_map) = self.get_payload_map() {
            for (key, value) in payload_map {
                payload.push((key, value));
            }

            vec_utils::sort_by_first(&mut payload);
            payload
        } else {
            payload
        }
    }

    fn set_name(&mut self, new_name: &String) {
        self.event_base.name = new_name.to_owned();
    }

    fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>) {
        self.event_base.timestamp = new_timestamp;
    }

    fn set_lifecycle(&mut self, lifecycle: Lifecycle) {
        self.lifecycle = Some(lifecycle);
    }

    fn add_or_update_payload(&mut self, key: String, value: EventPayloadValue) {
        if self.payload.is_none() {
            self.payload = Some(HashMap::new());
        }

        *self.payload.as_mut().unwrap().get_mut(&key).unwrap() = value;
    }

    fn get_user_data(&mut self) -> &mut UserDataImpl {
        self.event_base.user_data_holder.get_mut()
    }
}

impl Clone for XesEventImpl {
    fn clone(&self) -> Self {
        Self {
            event_base: self.event_base.clone(),
            lifecycle: self.lifecycle.clone(),
            payload: self.payload.clone(),
        }
    }
}
