use std::{cell::RefCell, collections::HashMap, rc::Rc};

use bxes::{
    models::{
        BxesClassifier, BxesEvent, BxesEventLog, BxesEventLogMetadata, BxesExtension, BxesGlobal, BxesGlobalKind, BxesTraceVariant,
        BxesValue,
    },
    writer::{errors::BxesWriteError, single_file_bxes_writer::write_bxes},
};
use chrono::{TimeZone, Utc};

use crate::event_log::{
    core::{
        event::{
            event::{Event, EventPayloadValue},
            lifecycle::{Lifecycle, XesBrafLifecycle, XesStandardLifecycle},
        },
        event_log::EventLog,
        trace::trace::Trace,
    },
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
};

pub fn write_event_log_to_bxes(log: &XesEventLogImpl, path: &str) -> Result<(), BxesWriteError> {
    let variants = log
        .traces()
        .iter()
        .map(|trace| BxesTraceVariant {
            traces_count: 1,
            metadata: vec![],
            events: trace
                .borrow()
                .events()
                .iter()
                .map(|event| {
                    let event = event.borrow();
                    BxesEvent {
                        name: Rc::new(Box::new(BxesValue::String(event.name_pointer().clone()))),
                        lifecycle: match event.lifecycle() {
                            None => bxes::models::Lifecycle::Braf(bxes::models::BrafLifecycle::Unspecified),
                            Some(lifecycle) => convert_xes_to_bxes_lifecycle(lifecycle),
                        },
                        timestamp: event.timestamp().timestamp_nanos(),
                        attributes: Some(event.ordered_payload().iter().map(|kv| kv_pair_to_bxes_pair(kv)).collect()),
                    }
                })
                .collect(),
        })
        .collect();

    let metadata = BxesEventLogMetadata {
        classifiers: Some(
            log.classifiers()
                .iter()
                .map(|c| BxesClassifier {
                    keys: c
                        .keys
                        .iter()
                        .map(|x| Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(x.to_owned()))))))
                        .collect(),
                    name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(c.name.to_owned()))))),
                })
                .collect(),
        ),
        extensions: Some(
            log.extensions()
                .iter()
                .map(|e| BxesExtension {
                    name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.name.to_owned()))))),
                    prefix: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.prefix.to_owned()))))),
                    uri: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.uri.to_owned()))))),
                })
                .collect(),
        ),
        globals: Some(
            log.ordered_globals()
                .iter()
                .map(|g| BxesGlobal {
                    entity_kind: match g.0.as_str() {
                        "event" => BxesGlobalKind::Event,
                        "trace" => BxesGlobalKind::Trace,
                        "log" => BxesGlobalKind::Log,
                        _ => panic!(),
                    },
                    globals: g
                        .1
                        .iter()
                        .map(|kv| {
                            let key = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.0.to_owned())))));
                            let value = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.1.to_owned())))));

                            (key, value)
                        })
                        .collect(),
                })
                .collect(),
        ),
        properties: Some(log.ordered_properties().iter().map(|kv| kv_pair_to_bxes_pair(kv)).collect()),
    };

    let bxes_log = BxesEventLog {
        metadata,
        variants,
        version: 0,
    };

    write_bxes(path, &bxes_log)
}

fn kv_pair_to_bxes_pair(kv: &(&String, &EventPayloadValue)) -> (Rc<Box<BxesValue>>, Rc<Box<BxesValue>>) {
    let bxes_value = payload_value_to_bxes_value(kv.1);
    let key = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.0.to_owned())))));

    (key, Rc::new(Box::new(bxes_value)))
}

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

fn payload_value_to_bxes_value(value: &EventPayloadValue) -> BxesValue {
    match value {
        EventPayloadValue::Date(value) => BxesValue::Timestamp(value.timestamp_nanos()),
        EventPayloadValue::String(value) => BxesValue::String(value.clone()),
        EventPayloadValue::Boolean(value) => BxesValue::Bool(*value),
        EventPayloadValue::Int32(value) => BxesValue::Int32(*value),
        EventPayloadValue::Int64(value) => BxesValue::Int64(*value),
        EventPayloadValue::Float32(value) => BxesValue::Float32(*value),
        EventPayloadValue::Float64(value) => BxesValue::Float64(*value),
        EventPayloadValue::Uint32(value) => BxesValue::Uint32(*value),
        EventPayloadValue::Uint64(value) => BxesValue::Uint64(*value),
        EventPayloadValue::Guid(value) => BxesValue::Guid(value.clone()),
        EventPayloadValue::Timestamp(value) => BxesValue::Timestamp(*value),
    }
}

fn convert_bxes_to_xes_lifecycle(bxes_lifecycle: &bxes::models::Lifecycle) -> Lifecycle {
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

fn convert_xes_to_bxes_lifecycle(ficus_lifecycle: Lifecycle) -> bxes::models::Lifecycle {
    match ficus_lifecycle {
        Lifecycle::BrafLifecycle(braf_lifecycle) => bxes::models::Lifecycle::Braf(match braf_lifecycle {
            XesBrafLifecycle::Unspecified => bxes::models::BrafLifecycle::Unspecified,
            XesBrafLifecycle::Closed => bxes::models::BrafLifecycle::Closed,
            XesBrafLifecycle::ClosedCancelled => bxes::models::BrafLifecycle::ClosedCancelled,
            XesBrafLifecycle::ClosedCancelledAborted => bxes::models::BrafLifecycle::ClosedCancelledAborted,
            XesBrafLifecycle::ClosedCancelledError => bxes::models::BrafLifecycle::ClosedCancelledError,
            XesBrafLifecycle::ClosedCancelledExited => bxes::models::BrafLifecycle::ClosedCancelledExited,
            XesBrafLifecycle::ClosedCancelledObsolete => bxes::models::BrafLifecycle::ClosedCancelledObsolete,
            XesBrafLifecycle::ClosedCancelledTerminated => bxes::models::BrafLifecycle::ClosedCancelledTerminated,
            XesBrafLifecycle::Completed => bxes::models::BrafLifecycle::Completed,
            XesBrafLifecycle::CompletedFailed => bxes::models::BrafLifecycle::CompletedFailed,
            XesBrafLifecycle::CompletedSuccess => bxes::models::BrafLifecycle::CompletedSuccess,
            XesBrafLifecycle::Open => bxes::models::BrafLifecycle::Open,
            XesBrafLifecycle::OpenNotRunning => bxes::models::BrafLifecycle::OpenNotRunning,
            XesBrafLifecycle::OpenNotRunningAssigned => bxes::models::BrafLifecycle::OpenNotRunningAssigned,
            XesBrafLifecycle::OpenNotRunningReserved => bxes::models::BrafLifecycle::OpenNotRunningReserved,
            XesBrafLifecycle::OpenNotRunningSuspendedAssigned => bxes::models::BrafLifecycle::OpenNotRunningSuspendedAssigned,
            XesBrafLifecycle::OpenNotRunningSuspendedReserved => bxes::models::BrafLifecycle::OpenNotRunningSuspendedReserved,
            XesBrafLifecycle::OpenRunning => bxes::models::BrafLifecycle::OpenRunning,
            XesBrafLifecycle::OpenRunningInProgress => bxes::models::BrafLifecycle::OpenRunningInProgress,
            XesBrafLifecycle::OpenRunningSuspended => bxes::models::BrafLifecycle::OpenRunningSuspended,
        }),
        Lifecycle::XesStandardLifecycle(standard_lifecycle) => bxes::models::Lifecycle::Standard(match standard_lifecycle {
            XesStandardLifecycle::Unspecified => bxes::models::StandardLifecycle::Unspecified,
            XesStandardLifecycle::Assign => bxes::models::StandardLifecycle::Assign,
            XesStandardLifecycle::AteAbort => bxes::models::StandardLifecycle::AteAbort,
            XesStandardLifecycle::Autoskip => bxes::models::StandardLifecycle::Autoskip,
            XesStandardLifecycle::Complete => bxes::models::StandardLifecycle::Complete,
            XesStandardLifecycle::ManualSkip => bxes::models::StandardLifecycle::ManualSkip,
            XesStandardLifecycle::PiAbort => bxes::models::StandardLifecycle::PiAbort,
            XesStandardLifecycle::ReAssign => bxes::models::StandardLifecycle::ReAssign,
            XesStandardLifecycle::Resume => bxes::models::StandardLifecycle::Resume,
            XesStandardLifecycle::Schedule => bxes::models::StandardLifecycle::Schedule,
            XesStandardLifecycle::Start => bxes::models::StandardLifecycle::Start,
            XesStandardLifecycle::Suspend => bxes::models::StandardLifecycle::Suspend,
            XesStandardLifecycle::Unknown => bxes::models::StandardLifecycle::Unknown,
            XesStandardLifecycle::Withdraw => bxes::models::StandardLifecycle::Withdraw,
        }),
    }
}
