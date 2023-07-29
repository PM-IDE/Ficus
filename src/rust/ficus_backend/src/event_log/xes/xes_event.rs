use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{
    event_log::core::event::{
        event::{Event, EventPayloadValue},
        lifecycle::Lifecycle,
    },
    utils::{
        user_data::{UserData, UserDataHolder},
        vec_utils,
    },
};

pub struct XesEventImpl {
    name: String,
    timestamp: DateTime<Utc>,
    lifecycle: Option<Lifecycle>,

    payload: HashMap<String, EventPayloadValue>,
    user_data_holder: UserDataHolder,
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
            user_data_holder: UserDataHolder::new(),
        }
    }
}

impl Event for XesEventImpl {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
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

    fn set_name(&mut self, new_name: &String) {
        self.name = new_name.to_owned();
    }

    fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>) {
        self.timestamp = new_timestamp;
    }

    fn set_lifecycle(&mut self, lifecycle: Lifecycle) {
        self.lifecycle = Some(lifecycle);
    }

    fn add_or_update_payload(&mut self, key: String, value: EventPayloadValue) {
        *self.payload.get_mut(&key).unwrap() = value;
    }

    fn get_user_data(&mut self) -> &mut UserData {
        self.user_data_holder.get_mut()
    }
}
