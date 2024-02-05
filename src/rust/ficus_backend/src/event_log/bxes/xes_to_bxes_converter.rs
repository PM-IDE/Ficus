use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bxes::{
    models::{BxesEvent, BxesValue},
    read::errors::BxesReadError,
};
use chrono::{TimeZone, Utc};

use crate::event_log::{
    core::{event::event::EventPayloadValue, event_log::EventLog, trace::trace::Trace},
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
};

use super::conversions::{bxes_value_to_payload_value, convert_bxes_to_xes_lifecycle};

pub enum BxesToXesReadError {
    BxesReadError(BxesReadError),
    ConversionError(String),
}

impl ToString for BxesToXesReadError {
    fn to_string(&self) -> String {
        match self {
            BxesToXesReadError::BxesReadError(err) => err.to_string(),
            BxesToXesReadError::ConversionError(err) => err.to_string(),
        }
    }
}

pub fn read_bxes_into_xes_log(path: &str) -> Result<XesEventLogImpl, BxesToXesReadError> {
    let log = match bxes::read::single_file_bxes_reader::read_bxes(path) {
        Ok(log) => log,
        Err(error) => return Err(BxesToXesReadError::BxesReadError(error)),
    };

    let mut xes_log = XesEventLogImpl::empty();
    for variant in &log.variants {
        let mut xes_trace = XesTraceImpl::empty();
        for event in &variant.events {
            xes_trace.push(Rc::new(RefCell::new(create_xes_event(event)?)));
        }

        xes_log.push(Rc::new(RefCell::new(xes_trace)));
    }

    Ok(xes_log)
}

fn create_xes_event(bxes_event: &BxesEvent) -> Result<XesEventImpl, BxesToXesReadError> {
    let name = if let BxesValue::String(string) = bxes_event.name.as_ref().as_ref() {
        string.clone()
    } else {
        let message = format!("The name of event was not a string: {:?}", bxes_event.name);
        return Err(BxesToXesReadError::ConversionError(message));
    };

    let timestamp = Utc.timestamp_nanos(bxes_event.timestamp);
    let lifecycle = convert_bxes_to_xes_lifecycle(&bxes_event.lifecycle);
    let payload = create_xes_payload(bxes_event.attributes.as_ref())?;

    Ok(XesEventImpl::new_all_fields(name, timestamp, Some(lifecycle), payload))
}

fn create_xes_payload(
    attributes: Option<&Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
) -> Result<Option<HashMap<String, EventPayloadValue>>, BxesToXesReadError> {
    if let Some(attributes) = attributes {
        let mut payload = HashMap::new();

        for (key, value) in attributes {
            let key = if let BxesValue::String(string) = key.as_ref().as_ref() {
                string.as_ref().as_ref().to_owned()
            } else {
                let message = format!("The attribute key is not a string: {:?}", key);
                return Err(BxesToXesReadError::ConversionError(message));
            };

            payload.insert(key, bxes_value_to_payload_value(&value));
        }

        Ok(Some(payload))
    } else {
        Ok(None)
    }
}
