use std::{
    cell::RefCell,
    fs,
    io::{self, Cursor},
    string::FromUtf8Error,
};

use quick_xml::{
    events::{BytesEnd, BytesStart},
    Writer,
};

use crate::event_log::{
    core::{
        event::{Event, EventPayloadValue},
        event_log::EventLog,
        trace::Trace,
    },
    xes::{constants::*, xes_event_log::XesEventLogImpl},
};

pub enum WriteLogError {
    FromUt8Error(FromUtf8Error),
    IOError(io::Error),
    WriterError(quick_xml::Error),
}

pub fn write_log(log: &XesEventLogImpl, save_path: &str) -> Result<(), WriteLogError> {
    match serialize_event_log(log) {
        Ok(content) => match fs::write(save_path, content) {
            Ok(_) => Ok(()),
            Err(error) => Err(WriteLogError::IOError(error)),
        },
        Err(error) => Err(error),
    }
}

pub fn serialize_event_log(log: &XesEventLogImpl) -> Result<String, WriteLogError> {
    let writer = RefCell::new(Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2));

    {
        let _log_cookie = StartEndElementCookie::new(&writer, LOG_TAG_NAME_STR);

        for ext in log.get_extensions() {
            let attrs = vec![
                (NAME_ATTR_NAME_STR, ext.name.as_str()),
                (URI_ATTR_NAME_STR, ext.uri.as_str()),
                (PREFIX_ATTR_NAME_STR, ext.prefix.as_str()),
            ];

            write_empty(&writer, EXTENSION_TAG_NAME_STR, &attrs)?;
        }

        for classifier in log.get_classifiers() {
            let keys = classifier.keys.join(" ");
            let attrs = vec![
                (NAME_ATTR_NAME_STR, classifier.name.as_str()),
                (KEYS_ATTR_NAME_STR, keys.as_str()),
            ];

            write_empty(&writer, CLASSIFIER_TAG_NAME_STR, &attrs)?;
        }

        for (name, value) in log.get_properties() {
            write_payload_tag(&writer, name, value)?;
        }

        for (scope, defaults) in log.get_globals() {
            let mut attrs = vec![(SCOPE_ATTR_NAME_STR, scope.as_str())];

            let _global_cookie = StartEndElementCookie::new_with_attrs(&writer, GLOBAL_TAG_NAME_STR, &attrs);

            for (key, value) in defaults {
                attrs.clear();
                attrs.push((KEY_ATTR_NAME_STR, key.as_str()));
                attrs.push((VALUE_ATTR_NANE_STR, value.as_str()));
                write_empty(&writer, STRING_TAG_NAME_STR, &attrs)?;
            }
        }

        for trace in log.get_traces() {
            let events = trace.get_events();
            if events.len() == 0 {
                continue;
            }

            let _trace_cookie = StartEndElementCookie::new(&writer, TRACE_TAG_NAME_STR);

            for event in events {
                let _event_cookie = StartEndElementCookie::new(&writer, EVENT_TAG_NAME_STR);

                let attrs = vec![
                    (KEY_ATTR_NAME_STR, NAME_ATTR_NAME_STR),
                    (VALUE_ATTR_NANE_STR, event.get_name()),
                ];

                write_empty(&writer, STRING_TAG_NAME_STR, &attrs)?;

                let date_string = event.get_timestamp().to_rfc3339();
                let attrs = vec![
                    (KEY_ATTR_NAME_STR, TIME_TIMESTAMP_STR),
                    (VALUE_ATTR_NANE_STR, date_string.as_str()),
                ];

                write_empty(&writer, DATE_TAG_NAME_STR, &attrs)?;

                if let Some(lifecycle) = event.get_lifecycle() {
                    let lifecycle_string = lifecycle.to_string();
                    let attrs = vec![
                        (KEY_ATTR_NAME_STR, LIFECYCLE_TRANSITION_STR),
                        (VALUE_ATTR_NANE_STR, lifecycle_string.as_str()),
                    ];

                    write_empty(&writer, STRING_TAG_NAME_STR, &attrs)?;
                }

                let payload = event.get_payload();
                for (key, value) in payload.borrow().iter() {
                    write_payload_tag(&writer, key, value)?;
                }
            }
        }
    }

    let content = writer.borrow().get_ref().get_ref().clone();
    match String::from_utf8(content) {
        Ok(string) => Ok(string),
        Err(error) => Err(WriteLogError::FromUt8Error(error)),
    }
}

fn write_payload_tag(
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    key: &str,
    value: &EventPayloadValue,
) -> Result<(), WriteLogError> {
    let tag_name = match value {
        EventPayloadValue::Date(_) => DATE_TAG_NAME_STR,
        EventPayloadValue::String(_) => STRING_TAG_NAME_STR,
        EventPayloadValue::Boolean(_) => BOOLEAN_TAG_NAME_STR,
        EventPayloadValue::Int(_) => INT_TAG_NAME_STR,
        EventPayloadValue::Float(_) => FLOAT_TAG_NAME_STR,
    };

    let string_value = value.to_string();
    let attrs = vec![(KEY_ATTR_NAME_STR, key), (VALUE_ATTR_NANE_STR, string_value.as_str())];

    write_empty(&writer, tag_name, &attrs)
}

fn write_empty(
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    tag_name: &str,
    attrs: &Vec<(&str, &str)>,
) -> Result<(), WriteLogError> {
    let mut empty_tag = BytesStart::new(tag_name);
    for (name, value) in attrs {
        empty_tag.push_attribute((*name, *value));
    }

    let empty = quick_xml::events::Event::Empty(empty_tag);

    match writer.borrow_mut().write_event(empty) {
        Ok(_) => Ok(()),
        Err(error) => Err(WriteLogError::WriterError(error)),
    }
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
    fn new(
        writer: &'a RefCell<Writer<Cursor<Vec<u8>>>>,
        tag_name: &'a str,
    ) -> Result<StartEndElementCookie<'a>, WriteLogError> {
        let start = quick_xml::events::Event::Start(BytesStart::new(tag_name));

        match writer.borrow_mut().write_event(start) {
            Err(error) => Err(WriteLogError::WriterError(error)),
            Ok(_) => Ok(StartEndElementCookie { tag_name, writer }),
        }
    }

    fn new_with_attrs(
        writer: &'a RefCell<Writer<Cursor<Vec<u8>>>>,
        tag_name: &'a str,
        attrs: &Vec<(&str, &str)>,
    ) -> Result<StartEndElementCookie<'a>, WriteLogError> {
        let mut start_tag = BytesStart::new(tag_name);
        for (name, value) in attrs {
            start_tag.push_attribute((*name, *value));
        }

        let start_event = quick_xml::events::Event::Start(start_tag);
        match writer.borrow_mut().write_event(start_event) {
            Err(error) => Err(WriteLogError::WriterError(error)),
            Ok(_) => Ok(StartEndElementCookie { tag_name, writer }),
        }
    }
}
