use std::{rc::Rc, cell::RefCell, io::BufReader, fs::File, collections::HashMap, str::FromStr};
use chrono::{Utc, DateTime};
use quick_xml::Reader;

use super::event::{EventImpl, Lifecycle, EventPayloadValue, XesStandardLifecycle};

pub(crate) struct FromFileXesEventLogReader {
    storage: Vec<u8>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>
}

pub(crate) struct TraceXesEventLogIterator {
    buffer: Vec<u8>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>
}

const EVENT_TAG_NAME: &[u8] = b"event";
const TRACE_TAG_NAME: &[u8] = b"trace";

const DATE_TAG_NAME: &[u8] = b"date";
const STRING_TAG_NAME: &[u8] = b"string";
const BOOLEAN_TAG_NAME: &[u8] = b"boolean";
const INT_TAG_NAME: &[u8] = b"int";
const FLOAT_TAG_NAME: &[u8] = b"float";

const KEY_ATTR_NAME: &[u8] = b"key";
const VALUE_ATTR_NAME: &[u8] = b"value";

const TIME_TIMESTAMP: &str = "time:timestamp";
const CONCEPT_NAME: &str = "concept:name";
const LIFECYCLE_TRANSITION: &str = "lifecycle:transition";

impl Iterator for TraceXesEventLogIterator {
    type Item = EventImpl;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let read_content = self.reader.borrow_mut().read_event_into(&mut self.buffer);
            match read_content {
                Ok(quick_xml::events::Event::Start(e)) => {
                    match e.name().0 {
                        EVENT_TAG_NAME => match self.parse_event_from() {
                            None => continue,
                            Some(parsed_event) => return Some(parsed_event)
                        },
                        _ => continue
                    }
                },
                Ok(quick_xml::events::Event::End(e)) => {
                    match e.name().0 {
                        b"trace" => return None,
                        _ => continue
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                    return None;
                },
                _ => continue
            }
        }
    }
}

impl TraceXesEventLogIterator {
    fn new(reader: Rc<RefCell<Reader<BufReader<File>>>>) -> TraceXesEventLogIterator {
        TraceXesEventLogIterator { reader, buffer: Vec::new() }
    }

    fn parse_event_from(&mut self) -> Option<EventImpl> {
        let mut name: Option<String> = None;
        let mut date: Option<DateTime<Utc>> = None;
        let mut lifecycle: Option<Lifecycle> = None;
        let mut payload: HashMap<String, EventPayloadValue> = HashMap::new();

        loop {
            let x = self.reader.borrow_mut().read_event_into(&mut self.buffer);
            match x {
                Ok(quick_xml::events::Event::End(end)) => {
                    match end.name().0 {
                        EVENT_TAG_NAME => {
                            if !name.is_some() { return None }
                            if !date.is_some() { return None }

                            let event = EventImpl::new(name.unwrap(), date.unwrap(), lifecycle, payload);
                            return Some(event)
                        },
                        _ => continue
                    }
                },
                Ok(quick_xml::events::Event::Empty(start)) => {
                    let mut key: Option<String> = None;
                    let mut value: Option<String> = None;
                    let mut payload_value: Option<EventPayloadValue> = None;

                    for attr in start.attributes() {
                        match attr {
                            Err(_) => return None,
                            Ok(real_attr) => {
                                match real_attr.key.0 {
                                    KEY_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                                        Err(_) => return None,
                                        Ok(string) => key = Some(string)
                                    },
                                    VALUE_ATTR_NAME => match String::from_utf8(real_attr.value.to_owned().to_vec()) {
                                        Err(_) => return None,
                                        Ok(string) => value = Some(string)
                                    },
                                    _ => continue
                                }
                            }
                        }
                    }

                    if !value.is_some() || !key.is_some() { return None }

                    match start.name().0 {
                        DATE_TAG_NAME => {
                            match DateTime::parse_from_rfc3339(value.unwrap().as_str()) {
                                Err(_) => return None,
                                Ok(date) => payload_value = Some(EventPayloadValue::Date(date.with_timezone(&Utc)))
                            }
                        },
                        INT_TAG_NAME => {
                            match value.unwrap().parse::<i32>() {
                                Err(_) => return None,
                                Ok(int_value) => payload_value = Some(EventPayloadValue::Int(int_value))
                            }
                        },
                        FLOAT_TAG_NAME => {
                            match value.unwrap().parse::<f32>() {
                                Err(_) => return None,
                                Ok(float_value) => payload_value = Some(EventPayloadValue::Float(float_value))
                            }
                        },
                        STRING_TAG_NAME => {
                            payload_value = Some(EventPayloadValue::String(value.unwrap()))
                        },
                        BOOLEAN_TAG_NAME => {
                            match value.unwrap().parse::<bool>() {
                                Err(_) => return None,
                                Ok(bool_value) => payload_value = Some(EventPayloadValue::Boolean(bool_value))
                            }
                        },
                        _ => continue
                    }

                    match key.as_ref().unwrap().as_str() {
                        TIME_TIMESTAMP => {
                            if let Some(EventPayloadValue::Date(parsed_date)) = payload_value {
                                date = Some(parsed_date);
                            }
                        },
                        CONCEPT_NAME => {
                            if let Some(EventPayloadValue::String(parsed_string)) = payload_value {
                                name = Some(parsed_string);
                            }
                        },
                        LIFECYCLE_TRANSITION => {
                            if let Some(EventPayloadValue::String(parsed_string)) = payload_value {
                                match XesStandardLifecycle::from_str(parsed_string.as_str()) {
                                    Ok(lifecycle_value) => lifecycle = Some(Lifecycle::XesStandardLifecycle(lifecycle_value)),
                                    Err(_) => return None
                                }
                            }
                        },
                        _ => {
                            payload.insert(key.unwrap(), payload_value.unwrap());
                        }
                    }
                },
                _ => continue
            }
        }
    }
}

impl<'a> Iterator for FromFileXesEventLogReader {
    type Item = TraceXesEventLogIterator;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.borrow_mut().read_event_into(&mut self.storage) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    match e.name().as_ref() {
                        TRACE_TAG_NAME => {
                            let copy_rc = Rc::clone(&self.reader);
                            return Some(TraceXesEventLogIterator::new(copy_rc))
                        },
                        _ => continue
                    }
                },
                Ok(quick_xml::events::Event::Eof) => return None,
                Err(error) => {
                    println!("Error: {}", error);
                    return None
                }
                _ => continue
            }
        }
    }
}

impl FromFileXesEventLogReader {
    pub fn new(file_path: &str) -> Option<FromFileXesEventLogReader> {
        match Reader::from_file(file_path) {
            Ok(reader) => Some(FromFileXesEventLogReader {
                reader: Rc::new(RefCell::new(reader)),
                storage: Vec::new()
            }),
            Err(_) => None
        }
    }
}