use std::{rc::Rc, cell::RefCell, collections::HashMap};

use bxes::models::BxesValue;
use chrono::{Utc, TimeZone};

use crate::event_log::{xes::{xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl, xes_event::XesEventImpl}, core::{event_log::EventLog, trace::trace::Trace, event::{event::{Event, EventPayloadValue}, lifecycle::{Lifecycle, XesBrafLifecycle, XesStandardLifecycle}}}};

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
                string.clone()
            } else {
                panic!("Name is not a string")
            };

            let timestamp = Utc.timestamp_nanos(event.timestamp);
            let lifecycle = convert_lifecycle(&event.lifecycle);

            let payload = 
            if let Some(attributes) = event.attributes.as_ref() {
                let mut payload = HashMap::new();

                for (key, value) in attributes {
                    let key = if let BxesValue::String(string) = key.as_ref().as_ref() {
                        string.as_ref().as_ref().to_owned()
                    } else {
                        panic!("Key is not a string");
                    };

                    let paylaod_value = match value.as_ref().as_ref() {
                        BxesValue::Int32(value) => EventPayloadValue::Int32(*value),
                        BxesValue::Int64(value) => EventPayloadValue::Int64(*value),
                        BxesValue::Uint32(value) => EventPayloadValue::Uint32(*value),
                        BxesValue::Uint64(value) => EventPayloadValue::Uint64(*value),
                        BxesValue::Float32(value) => EventPayloadValue::Float32(*value),
                        BxesValue::Float64(value) => EventPayloadValue::Float64(*value),
                        BxesValue::String(string) => EventPayloadValue::String(string.clone()),
                        BxesValue::Bool(bool) => EventPayloadValue::Boolean(*bool),
                        BxesValue::Timestamp(stamp) => EventPayloadValue::Date(Utc.timestamp_nanos(*stamp)),
                        BxesValue::BrafLifecycle(_) => todo!(),
                        BxesValue::StandardLifecycle(_) => todo!(),
                        BxesValue::Artifact(_) => todo!(),
                        BxesValue::Drivers(_) => todo!(),
                        BxesValue::Guid(value) => EventPayloadValue::Guid(*value),
                        BxesValue::SoftwareEventType(_) => todo!(),
                    };

                    payload.insert(key, paylaod_value);
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

fn convert_lifecycle(bxes_lifecycle: &bxes::models::Lifecycle) -> Lifecycle {
    match bxes_lifecycle {
        bxes::models::Lifecycle::Braf(braf_lifecycle) => Lifecycle::BrafLifecycle(match braf_lifecycle {
            bxes::models::BrafLifecycle::Unspecified => XesBrafLifecycle::Unspecified,
            bxes::models::BrafLifecycle::Closed => XesBrafLifecycle::Closed,
            bxes::models::BrafLifecycle::ClosedCancelled => XesBrafLifecycle::ClosedCancelled,
            bxes::models::BrafLifecycle::ClosedCancelledAborted => XesBrafLifecycle::ClosedCancelledAborted,
            bxes::models::BrafLifecycle::ClosedCancelledError => XesBrafLifecycle::ClosedCancelledError,
            bxes::models::BrafLifecycle::ClosedCancelledExited => XesBrafLifecycle::ClosedCancelledExited,
            bxes::models::BrafLifecycle::ClosedCancelledObsolete => XesBrafLifecycle::ClosedCancelledObsolete,
            bxes::models::BrafLifecycle::ClosedCancelledTerminated => XesBrafLifecycle::ClosedCancelledTerminated,
            bxes::models::BrafLifecycle::Completed => XesBrafLifecycle::Completed,
            bxes::models::BrafLifecycle::CompletedFailed => XesBrafLifecycle::CompletedFailed,
            bxes::models::BrafLifecycle::CompletedSuccess => XesBrafLifecycle::CompletedSuccess,
            bxes::models::BrafLifecycle::Open => XesBrafLifecycle::Open,
            bxes::models::BrafLifecycle::OpenNotRunning => XesBrafLifecycle::OpenNotRunning,
            bxes::models::BrafLifecycle::OpenNotRunningAssigned => XesBrafLifecycle::OpenNotRunningAssigned,
            bxes::models::BrafLifecycle::OpenNotRunningReserved => XesBrafLifecycle::OpenNotRunningReserved,
            bxes::models::BrafLifecycle::OpenNotRunningSuspendedAssigned => XesBrafLifecycle::OpenNotRunningSuspendedAssigned,
            bxes::models::BrafLifecycle::OpenNotRunningSuspendedReserved => XesBrafLifecycle::OpenNotRunningSuspendedReserved,
            bxes::models::BrafLifecycle::OpenRunning => XesBrafLifecycle::OpenRunning,
            bxes::models::BrafLifecycle::OpenRunningInProgress => XesBrafLifecycle::OpenRunningInProgress,
            bxes::models::BrafLifecycle::OpenRunningSuspended => XesBrafLifecycle::OpenRunningSuspended,
        }),
        bxes::models::Lifecycle::Standard(standard_lifecycle) => Lifecycle::XesStandardLifecycle(match standard_lifecycle {
            bxes::models::StandardLifecycle::Unspecified => XesStandardLifecycle::Unspecified,
            bxes::models::StandardLifecycle::Assign => XesStandardLifecycle::Assign,
            bxes::models::StandardLifecycle::AteAbort => XesStandardLifecycle::AteAbort,
            bxes::models::StandardLifecycle::Autoskip => XesStandardLifecycle::Autoskip,
            bxes::models::StandardLifecycle::Complete => XesStandardLifecycle::Complete,
            bxes::models::StandardLifecycle::ManualSkip => XesStandardLifecycle::ManualSkip,
            bxes::models::StandardLifecycle::PiAbort => XesStandardLifecycle::PiAbort,
            bxes::models::StandardLifecycle::ReAssign => XesStandardLifecycle::ReAssign,
            bxes::models::StandardLifecycle::Resume => XesStandardLifecycle::Resume,
            bxes::models::StandardLifecycle::Schedule => XesStandardLifecycle::Schedule,
            bxes::models::StandardLifecycle::Start => XesStandardLifecycle::Start,
            bxes::models::StandardLifecycle::Suspend => XesStandardLifecycle::Suspend,
            bxes::models::StandardLifecycle::Unknown => XesStandardLifecycle::Unknown,
            bxes::models::StandardLifecycle::Withdraw => XesStandardLifecycle::Withdraw,
        }),
    }
}