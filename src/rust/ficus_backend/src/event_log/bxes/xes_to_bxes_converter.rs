use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bxes::models::BxesValue;
use chrono::{TimeZone, Utc};

use crate::event_log::{
    core::{event_log::EventLog, trace::trace::Trace},
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
};

use super::conversions::{bxes_value_to_payload_value, convert_bxes_to_xes_lifecycle};

pub fn read_bxes_into_xes_log(path: &str) -> Option<XesEventLogImpl> {
    let log = match bxes::read::single_file_bxes_reader::read_bxes(path) {
        Ok(log) => log,
        Err(_) => return None,
    };

    let mut xes_log = XesEventLogImpl::empty();
    for variant in &log.variants {
        let mut xes_trace = XesTraceImpl::empty();
        for event in &variant.events {
            let name = if let BxesValue::String(string) = event.name.as_ref().as_ref() {
                string.clone()
            } else {
                panic!("Name is not a string")
            };

            let timestamp = Utc.timestamp_nanos(event.timestamp);
            let lifecycle = convert_bxes_to_xes_lifecycle(&event.lifecycle);

            let payload = if let Some(attributes) = event.attributes.as_ref() {
                let mut payload = HashMap::new();

                for (key, value) in attributes {
                    let key = if let BxesValue::String(string) = key.as_ref().as_ref() {
                        string.as_ref().as_ref().to_owned()
                    } else {
                        panic!("Key is not a string");
                    };

                    payload.insert(key, bxes_value_to_payload_value(&value));
                }

                Some(payload)
            } else {
                None
            };

            let xes_event = XesEventImpl::new_all_fields(name, timestamp, Some(lifecycle), payload);
            xes_trace.push(Rc::new(RefCell::new(xes_event)));
        }

        xes_log.push(Rc::new(RefCell::new(xes_trace)));
    }

    Some(xes_log)
}
