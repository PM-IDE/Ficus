use super::constants::*;
use super::event::{EventImpl, EventPayloadValue, Lifecycle, XesStandardLifecycle};
use chrono::{DateTime, Utc};
use quick_xml::{events::BytesStart, Reader};
use std::{cell::RefCell, collections::HashMap, fs::File, io::BufReader, rc::Rc, str::FromStr};

pub(crate) struct TraceXesEventLogIterator {
    buffer: Vec<u8>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>,
}

impl Iterator for TraceXesEventLogIterator {
    type Item = EventImpl;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let read_content = self.reader.borrow_mut().read_event_into(&mut self.buffer);
            match read_content {
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
                Err(error) => {
                    println!("Error: {}", error);
                    return None;
                }
                _ => continue,
            }
        }
    }
}

struct KeyValuePair<TKey, TValue> {
    pub key: Option<TKey>,
    pub value: Option<TValue>,
}

impl TraceXesEventLogIterator {
    pub(crate) fn new(reader: Rc<RefCell<Reader<BufReader<File>>>>) -> TraceXesEventLogIterator {
        TraceXesEventLogIterator {
            reader,
            buffer: Vec::new(),
        }
    }

    fn parse_event_from(&mut self) -> Option<EventImpl> {
        let mut name: Option<String> = None;
        let mut date: Option<DateTime<Utc>> = None;
        let mut lifecycle: Option<Lifecycle> = None;
        let payload: Rc<RefCell<HashMap<String, EventPayloadValue>>> =
            Rc::new(RefCell::new(HashMap::new()));

        loop {
            let x = self.reader.borrow_mut().read_event_into(&mut self.buffer);
            match x {
                Ok(quick_xml::events::Event::End(end)) => match end.name().0 {
                    EVENT_TAG_NAME => {
                        if !name.is_some() {
                            return None;
                        }
                        if !date.is_some() {
                            return None;
                        }

                        let event =
                            EventImpl::new(name.unwrap(), date.unwrap(), lifecycle, payload);
                        return Some(event);
                    }
                    _ => continue,
                },
                Ok(quick_xml::events::Event::Empty(empty)) => {
                    let kv = Self::extract_key_value(&empty);
                    if !kv.value.is_some() || !kv.key.is_some() {
                        return None;
                    }

                    let key = kv.key.as_ref().unwrap().as_str();
                    let value = kv.value.as_ref().unwrap().as_str();

                    let payload_value = Self::extract_payload_value(&empty, value);
                    if !payload_value.is_some() {
                        return None;
                    }
                    Self::update_event_data(
                        key,
                        payload_value.unwrap(),
                        &mut date,
                        &mut name,
                        &mut lifecycle,
                        &payload,
                    )
                }
                _ => continue,
            }
        }
    }

    fn extract_key_value(start: &BytesStart) -> KeyValuePair<String, String> {
        let mut key: Option<String> = None;
        let mut value: Option<String> = None;

        for attr in start.attributes() {
            match attr {
                Err(_) => continue,
                Ok(real_attr) => match real_attr.key.0 {
                    KEY_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                        Err(_) => continue,
                        Ok(string) => key = Some(string),
                    },
                    VALUE_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec())
                    {
                        Err(_) => continue,
                        Ok(string) => value = Some(string),
                    },
                    _ => continue,
                },
            }
        }

        return KeyValuePair { key, value };
    }

    fn extract_payload_value(empty: &BytesStart, value: &str) -> Option<EventPayloadValue> {
        match empty.name().0 {
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
            TIME_TIMESTAMP => {
                if let EventPayloadValue::Date(parsed_date) = payload_value {
                    *date = Some(parsed_date);
                }
            }
            CONCEPT_NAME => {
                if let EventPayloadValue::String(parsed_string) = payload_value {
                    *name = Some(parsed_string);
                }
            }
            LIFECYCLE_TRANSITION => {
                if let EventPayloadValue::String(parsed_string) = payload_value {
                    match XesStandardLifecycle::from_str(parsed_string.as_str()) {
                        Ok(lifecycle_value) => {
                            *lifecycle = Some(Lifecycle::XesStandardLifecycle(lifecycle_value))
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                payload.borrow_mut().insert(key.to_owned(), payload_value);
            }
        }
    }
}
