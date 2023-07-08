use crate::event_log::{
    core::{
        event::EventPayloadValue,
        lifecycle::{Lifecycle, XesStandardLifecycle},
    },
    xes::{utils, xes_event::XesEventImpl},
};

use crate::event_log::xes::constants::*;

use chrono::{DateTime, Utc};
use quick_xml::Reader;
use std::{cell::RefCell, collections::HashMap, fs::File, io::BufReader, rc::Rc, str::FromStr};

pub struct TraceXesEventLogIterator {
    buffer: Vec<u8>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>,
    globals: Rc<RefCell<HashMap<String, HashMap<String, String>>>>,
}

impl Iterator for TraceXesEventLogIterator {
    type Item = XesEventImpl;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = self.reader.borrow_mut().read_event_into(&mut self.buffer);
            match event {
                Ok(quick_xml::events::Event::Start(e)) => match e.name().0 {
                    EVENT_TAG_NAME => match self.parse_event_from() {
                        None => continue,
                        Some(parsed_event) => return Some(parsed_event),
                    },
                    _ => continue,
                },
                Ok(quick_xml::events::Event::End(e)) => match e.name().0 {
                    b"trace" => return None,
                    _ => continue,
                },
                Err(_) => return None,
                _ => continue,
            }
        }
    }
}

impl TraceXesEventLogIterator {
    pub(crate) fn new(
        reader: Rc<RefCell<Reader<BufReader<File>>>>,
        seen_globals: Rc<RefCell<HashMap<String, HashMap<String, String>>>>,
    ) -> TraceXesEventLogIterator {
        TraceXesEventLogIterator {
            reader,
            buffer: Vec::new(),
            globals: seen_globals,
        }
    }

    fn parse_event_from(&mut self) -> Option<XesEventImpl> {
        let mut name = None;
        let mut date = None;
        let mut lifecycle = None;
        let payload = Rc::new(RefCell::new(HashMap::new()));

        self.set_defaults_value(&mut name, &mut date, &mut lifecycle, &payload);

        loop {
            match self.reader.borrow_mut().read_event_into(&mut self.buffer) {
                Ok(quick_xml::events::Event::End(end)) => match end.name().0 {
                    EVENT_TAG_NAME => {
                        if !name.is_some() {
                            return None;
                        }
                        if !date.is_some() {
                            return None;
                        }

                        let event = XesEventImpl::new(name.unwrap(), date.unwrap(), lifecycle, payload);
                        return Some(event);
                    }
                    _ => continue,
                },
                Ok(quick_xml::events::Event::Empty(empty)) => {
                    let kv = utils::extract_key_value(&empty);
                    if !kv.value.is_some() || !kv.key.is_some() {
                        return None;
                    }

                    let key = kv.key.as_ref().unwrap().as_str();
                    let value = kv.value.as_ref().unwrap().as_str();
                    let payload_type = empty.name().0;

                    Self::set_parsed_value(payload_type, key, value, &mut name, &mut date, &mut lifecycle, &payload);
                }
                _ => continue,
            }
        }
    }

    fn set_defaults_value(
        &self,
        name: &mut Option<String>,
        date: &mut Option<DateTime<Utc>>,
        lifecycle: &mut Option<Lifecycle>,
        payload: &Rc<RefCell<HashMap<String, EventPayloadValue>>>,
    ) {
        let globals = self.globals.borrow_mut();
        if !globals.contains_key(EVENT_TAG_NAME_STR) {
            return;
        }

        for (key, value) in globals.get(EVENT_TAG_NAME_STR).unwrap() {
            Self::set_parsed_value(STRING_TAG_NAME, key, value, name, date, lifecycle, payload);
        }
    }

    fn set_parsed_value(
        payload_type: &[u8],
        key: &str,
        value: &str,
        name: &mut Option<String>,
        date: &mut Option<DateTime<Utc>>,
        lifecycle: &mut Option<Lifecycle>,
        payload: &Rc<RefCell<HashMap<String, EventPayloadValue>>>,
    ) -> bool {
        let payload_value = Self::extract_payload_value(payload_type, value);
        if !payload_value.is_some() {
            return false;
        }

        Self::update_event_data(key, payload_value.unwrap(), date, name, lifecycle, &payload);

        true
    }

    fn extract_payload_value(name: &[u8], value: &str) -> Option<EventPayloadValue> {
        match name {
            DATE_TAG_NAME => match DateTime::parse_from_rfc3339(value) {
                Err(_) => None,
                Ok(date) => Some(EventPayloadValue::Date(date.with_timezone(&Utc))),
            },
            INT_TAG_NAME => match value.parse::<i32>() {
                Err(_) => None,
                Ok(int_value) => Some(EventPayloadValue::Int(int_value)),
            },
            FLOAT_TAG_NAME => match value.parse::<f32>() {
                Err(_) => None,
                Ok(float_value) => Some(EventPayloadValue::Float(float_value)),
            },
            STRING_TAG_NAME => Some(EventPayloadValue::String(value.to_owned())),
            BOOLEAN_TAG_NAME => match value.parse::<bool>() {
                Err(_) => None,
                Ok(bool_value) => Some(EventPayloadValue::Boolean(bool_value)),
            },
            _ => None,
        }
    }

    fn update_event_data(
        key: &str,
        payload_value: EventPayloadValue,
        date: &mut Option<DateTime<Utc>>,
        name: &mut Option<String>,
        lifecycle: &mut Option<Lifecycle>,
        payload: &Rc<RefCell<HashMap<String, EventPayloadValue>>>,
    ) {
        match key {
            TIME_TIMESTAMP_STR => {
                if let EventPayloadValue::Date(parsed_date) = payload_value {
                    *date = Some(parsed_date);
                }
            }
            CONCEPT_NAME_STR => {
                if let EventPayloadValue::String(parsed_string) = payload_value {
                    *name = Some(parsed_string);
                }
            }
            LIFECYCLE_TRANSITION_STR => {
                if let EventPayloadValue::String(parsed_string) = payload_value {
                    match XesStandardLifecycle::from_str(parsed_string.as_str()) {
                        Ok(lifecycle_value) => *lifecycle = Some(Lifecycle::XesStandardLifecycle(lifecycle_value)),
                        _ => {}
                    }
                }
            }
            _ => {
                let mut map = payload.borrow_mut();
                if map.contains_key(key) {
                    *map.get_mut(key).unwrap() = payload_value;
                } else {
                    map.insert(key.to_owned(), payload_value);
                }
            }
        }
    }
}
