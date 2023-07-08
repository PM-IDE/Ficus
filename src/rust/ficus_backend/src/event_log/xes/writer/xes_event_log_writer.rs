use std::{io::{Cursor, self}, fs, string::FromUtf8Error, cell::RefCell};

use quick_xml;
use quick_xml::{Writer, events::{BytesStart, BytesEnd}};

use crate::event_log::{xes::{xes_event_log::XesEventLogImpl, constants::*}, core::{event_log::EventLog, trace::Trace, event::{Event, EventPayloadValue}}};

pub enum WriteLogError {
    FromUt8Error(FromUtf8Error),
    IOError(io::Error),
}

pub fn write_log(log: &XesEventLogImpl, save_path: &str) -> Result<(), WriteLogError> {
    match serialize_log(log) {
        Ok(content) => match fs::write(save_path, content) {
            Ok(_) => Ok(()),
            Err(error) => Err(WriteLogError::IOError(error)),
        },
        Err(error) => Err(WriteLogError::FromUt8Error(error)),
    }
}

fn serialize_log(log: &XesEventLogImpl) -> Result<String, FromUtf8Error> {
    //todo: refactor this trash
    let writer = RefCell::new(Writer::new(Cursor::new(Vec::new())));
    let mut attrs = Vec::new();
    
    {
        let _log_cookie = StartEndElementCookie::new(&writer, LOG_TAG_NAME_STR);

        for (scope, defaults) in log.get_globals() {
            attrs.clear();
            attrs.push((SCOPE_ATTR_NAME_STR, scope.as_str()));
            let _global_cookie = StartEndElementCookie::new_with_attrs(&writer, GLOBAL_TAG_NAME_STR, &attrs);

            for (key, value) in defaults {
                attrs.clear();
                attrs.push((KEY_ATTR_NAME_STR, key.as_str()));
                attrs.push((VALUE_ATTR_NANE_STR, value.as_str()));
                write_empty(&writer, STRING_TAG_NAME_STR, &attrs);
            }
        }

        for classifier in log.get_classifiers() {
            let mut attrs = Vec::new();
            let keys = classifier.keys.join(" ");
            attrs.clear();
            attrs.push((NAME_ATTR_NAME_STR, classifier.name.as_str()));
            attrs.push((KEYS_ATTR_NAME_STR, keys.as_str()));

            write_empty(&writer, CLASSIFIER_TAG_NAME_STR, &attrs);
        }

        for ext in log.get_extensions() {
            attrs.clear();
            attrs.push((NAME_ATTR_NAME_STR, ext.name.as_str()));
            attrs.push((URI_ATTR_NAME_STR, ext.uri.as_str()));
            attrs.push((PREFIX_ATTR_NAME_STR, ext.prefix.as_str()));
            write_empty(&writer, EXTENSION_TAG_NAME_STR, &attrs);
        }

        for trace in log.get_traces() {
            let events = trace.get_events();
            if events.len() == 0 { continue; }

            let _trace_cookie = StartEndElementCookie::new(&writer, TRACE_TAG_NAME_STR);

            for event in events {
                let mut attrs = Vec::new();
                let _event_cookie = StartEndElementCookie::new(&writer, EVENT_TAG_NAME_STR);

                attrs.push((KEY_ATTR_NAME_STR, NAME_ATTR_NAME_STR));
                attrs.push((VALUE_ATTR_NANE_STR, event.get_name()));
                write_empty(&writer, STRING_TAG_NAME_STR, &attrs);

                attrs.clear();
                let date_string = event.get_timestamp().to_rfc3339();
                attrs.push((VALUE_ATTR_NANE_STR, date_string.as_str()));
                attrs.push((KEY_ATTR_NAME_STR, TIME_TIMESTAMP_STR));
                write_empty(&writer, DATE_TAG_NAME_STR, &attrs);

                if let Some(lifecycle) = event.get_lifecycle() {
                    let mut attrs = Vec::new();
                    let lifecycle_string = lifecycle.to_string();
                    attrs.push((VALUE_ATTR_NANE_STR, lifecycle_string.as_str()));
                    attrs.push((KEY_ATTR_NAME_STR, LIFECYCLE_TRANSITION_STR));
                    write_empty(&writer, STRING_TAG_NAME_STR, &attrs);
                }

                let payload = event.get_payload();
                for (key, value) in payload.borrow().iter() {
                    let mut attrs = Vec::new();
                    let tag_name = match value {
                        EventPayloadValue::Date(_) => DATE_TAG_NAME_STR,
                        EventPayloadValue::String(_) => STRING_TAG_NAME_STR,
                        EventPayloadValue::Boolean(_) => BOOLEAN_TAG_NAME_STR,
                        EventPayloadValue::Int(_) => INT_TAG_NAME_STR,
                        EventPayloadValue::Float(_) => FLOAT_TAG_NAME_STR,
                    };

                    attrs.clear();
                    attrs.push((KEY_ATTR_NAME_STR, key.as_str()));
                    let string_value = value.to_string();
                    attrs.push((VALUE_ATTR_NANE_STR, string_value.as_str()));
                    write_empty(&writer, tag_name, &attrs);
                }
            }
        }
    }

    let content = writer.borrow().get_ref().get_ref().clone();
    String::from_utf8(content)
}

fn write_empty(
    writer: & RefCell<Writer<Cursor<Vec<u8>>>>,
    tag_name: &str,
    attrs: &Vec<(&str, &str)>
) {
    let mut empty_tag = BytesStart::new(tag_name);
    for (name, value) in attrs {
        empty_tag.push_attribute((*name, *value));
    }

    assert!(writer.borrow_mut().write_event(quick_xml::events::Event::Empty(empty_tag)).is_ok());
}

struct StartEndElementCookie<'a> {
    tag_name: &'a str,
    writer: &'a RefCell<Writer<Cursor<Vec<u8>>>>,
}

impl<'a> Drop for StartEndElementCookie<'a> {
    fn drop(&mut self) {
        let end = quick_xml::events::Event::End(BytesEnd::new(self.tag_name));
        assert!(self.writer.borrow_mut().write_event(end).is_ok());
    }
}

impl<'a> StartEndElementCookie<'a> {
    fn new(writer: &'a RefCell<Writer<Cursor<Vec<u8>>>>, tag_name: &'a str) -> StartEndElementCookie<'a> {
        let start = quick_xml::events::Event::Start(BytesStart::new(tag_name));
        assert!(writer.borrow_mut().write_event(start).is_ok());
        assert!(writer.borrow_mut().write_indent().is_ok());
        StartEndElementCookie { tag_name, writer }
    }

    fn new_with_attrs(
        writer: &'a RefCell<Writer<Cursor<Vec<u8>>>>,
        tag_name: &'a str,
        attrs: &Vec<(&str, &str)>
    ) -> StartEndElementCookie<'a> {
        let mut start_tag = BytesStart::new(tag_name);
        for (name, value) in attrs {
            start_tag.push_attribute((*name, *value));
        }

        assert!(writer.borrow_mut().write_event(quick_xml::events::Event::Start(start_tag)).is_ok());
        assert!(writer.borrow_mut().write_indent().is_ok());
        StartEndElementCookie { tag_name, writer }
    }
}