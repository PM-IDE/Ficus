use std::{rc::Rc, cell::RefCell};

use bxes::models::BxesValue;
use chrono::{Utc, TimeZone};

use crate::event_log::{xes::{xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl, xes_event::XesEventImpl}, core::{event_log::EventLog, trace::trace::Trace, event::event::Event}};

pub fn read_bxes_into_xes_log(path: &str) -> Option<XesEventLogImpl> {
    let log = match bxes::read::single_file_bxes_reader::read_bxes(path) {
        Ok(log) => log,
        Err(_) => return None
    };

    let mut xes_log = XesEventLogImpl::empty();
    for variant in &log.variants {
        let mut xes_trace = XesTraceImpl::empty();
        for event in &variant.events {
            let name = if let BxesValue::String(string) = event.name.as_ref().as_ref() {
                string.as_ref().as_ref().to_owned()
            } else {
                panic!("Name is not a string")
            };

            let timestamp = Utc.timestamp_nanos(event.timestamp);

            let xes_event = XesEventImpl::new(name, timestamp);
            xes_trace.push(Rc::new(RefCell::new(xes_event)));
        }

        xes_log.push(Rc::new(RefCell::new(xes_trace)));
    }

    Some(xes_log)
}