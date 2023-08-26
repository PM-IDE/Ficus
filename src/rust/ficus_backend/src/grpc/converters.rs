use std::{any::Any, sync::Arc, ops::Add, rc::Rc, cell::{RefCell, Ref}};

use chrono::{DateTime, Utc, Duration};

use crate::{
    features::analysis::patterns::{repeat_sets::SubArrayWithTraceIndex, tandem_arrays::SubArrayInTraceInfo},
    ficus_proto::{
        grpc_context_value::ContextValue, GrpcContextKeyValue, GrpcContextValue,
        GrpcEventLogTraceSubArraysContextValue, GrpcHashesEventLog, GrpcHashesEventLogContextValue, GrpcHashesLogTrace,
        GrpcNamesEventLog, GrpcNamesEventLogContextValue, GrpcNamesTrace, GrpcSubArrayWithTraceIndex,
        GrpcSubArraysWithTraceIndexContextValue, GrpcTraceSubArray, GrpcTraceSubArrays,
    },
    pipelines::{
        context::PipelineContext,
        keys::{context_key::ContextKey, context_keys::ContextKeys},
    },
    utils::user_data::{keys::Key, user_data::UserData}, event_log::{xes::{xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl, xes_event::XesEventImpl}, core::{event_log::EventLog, trace::trace::Trace}},
};

pub(super) fn create_initial_context(
    values: &Vec<GrpcContextKeyValue>,
    keys: &Arc<Box<ContextKeys>>,
) -> PipelineContext {
    let mut context = PipelineContext::new(keys);

    for value in values {
        let key = keys.find_key(&value.key.as_ref().unwrap().name).unwrap();
        let value = value.value.as_ref().unwrap().context_value.as_ref().unwrap();
        put_into_user_data(key.key(), value, &mut context);
    }

    context
}

pub(super) fn put_into_user_data(key: &dyn Key, value: &ContextValue, user_data: &mut impl UserData) {
    match value {
        ContextValue::String(string) => user_data.put_any::<String>(key, string.clone()),
        ContextValue::HashesLog(_) => todo!(),
        ContextValue::NamesLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
        ContextValue::Uint32(number) => user_data.put_any::<u32>(key, number.clone()),
        ContextValue::TracesSubArrays(_) => todo!(),
        ContextValue::TraceIndexSubArrays(_) => todo!(),
        ContextValue::Bool(bool) => user_data.put_any::<bool>(key, bool.clone()),
        ContextValue::XesEventLog(grpc_log) => put_names_log_to_context(key, grpc_log, user_data),
    }
}

fn put_names_log_to_context(key: &dyn Key, grpc_log: &GrpcNamesEventLogContextValue, user_data: &mut impl UserData) {
    let grpc_log = grpc_log.log.as_ref().unwrap();
    let mut log = XesEventLogImpl::empty();
    for grpc_trace in &grpc_log.traces {
        let mut trace = XesTraceImpl::empty();
        let mut date = DateTime::<Utc>::MIN_UTC;

        for grpc_event in &grpc_trace.events {
            let event = XesEventImpl::new_with_date(grpc_event.clone(), date.clone());
            trace.push(Rc::new(RefCell::new(event)));
            date = date + Duration::seconds(1);
        }

        log.push(Rc::new(RefCell::new(trace)));
    }

    user_data.put_any::<XesEventLogImpl>(key, log);
}

pub(super) fn convert_to_grpc_context_value(
    key: &dyn ContextKey,
    value: &dyn Any,
    keys: &ContextKeys,
) -> Option<GrpcContextValue> {
    if keys.is_path(key) {
        try_convert_to_string_context_value(value)
    } else if keys.is_hashes_event_log(key) {
        try_convert_to_hashes_event_log(value)
    } else if keys.is_names_event_log(key) {
        try_convert_to_names_event_log(value)
    } else if keys.is_patterns(key) {
        try_convert_to_grpc_traces_sub_arrays(value)
    } else if keys.is_repeat_sets(key) {
        try_convert_to_grpc_sub_arrays_with_index(value)
    } else {
        None
    }
}

fn try_convert_to_string_context_value(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<String>() {
        None
    } else {
        Some(GrpcContextValue {
            context_value: Some(ContextValue::String(value.downcast_ref::<String>().unwrap().clone())),
        })
    }
}

fn try_convert_to_hashes_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<u64>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<u64>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(*event);
            }

            traces.push(GrpcHashesLogTrace { events });
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::HashesLog(GrpcHashesEventLogContextValue {
                log: Some(GrpcHashesEventLog { traces }),
            })),
        })
    }
}

fn try_convert_to_names_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<String>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<String>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(event.clone());
            }

            traces.push(GrpcNamesTrace { events });
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::NamesLog(GrpcNamesEventLogContextValue {
                log: Some(GrpcNamesEventLog { traces }),
            })),
        })
    }
}

fn try_convert_to_grpc_traces_sub_arrays(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<Vec<SubArrayInTraceInfo>>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<Vec<SubArrayInTraceInfo>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut sub_arrays = vec![];
            for array in trace {
                sub_arrays.push(GrpcTraceSubArray {
                    start: array.start_index as u32,
                    end: (array.start_index + array.length) as u32,
                })
            }

            traces.push(GrpcTraceSubArrays { sub_arrays })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::TracesSubArrays(GrpcEventLogTraceSubArraysContextValue {
                traces_sub_arrays: traces,
            })),
        })
    }
}

fn try_convert_to_grpc_sub_arrays_with_index(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<Vec<SubArrayWithTraceIndex>>() {
        None
    } else {
        let vec = value.downcast_ref::<Vec<SubArrayWithTraceIndex>>().unwrap();
        let mut sub_arrays = vec![];

        for array in vec {
            sub_arrays.push(GrpcSubArrayWithTraceIndex {
                sub_array: Some(GrpcTraceSubArray {
                    start: array.sub_array.start_index as u32,
                    end: (array.sub_array.start_index + array.sub_array.length) as u32,
                }),
                trace_index: array.trace_index as u32,
            })
        }

        Some(GrpcContextValue {
            context_value: Some(ContextValue::TraceIndexSubArrays(
                GrpcSubArraysWithTraceIndexContextValue { sub_arrays },
            )),
        })
    }
}
