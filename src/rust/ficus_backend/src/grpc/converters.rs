use std::{any::Any, sync::Arc};

use crate::{
    ficus_proto::{
        grpc_context_value::ContextValue, GrpcContextKeyValue, GrpcContextValue, GrpcHashesEventLog,
        GrpcHashesEventLogContextValue, GrpcHashesLogTrace, GrpcNamesEventLog, GrpcNamesEventLogContextValue,
        GrpcNamesTrace, GrpcStringContextValue,
    },
    pipelines::{
        context::PipelineContext,
        keys::{context_key::ContextKey, context_keys::ContextKeys},
    },
};

pub(super) fn create_initial_context(
    values: &Vec<GrpcContextKeyValue>,
    keys: &Arc<Box<ContextKeys>>,
) -> PipelineContext {
    let mut context = PipelineContext::new(keys);

    for value in values {
        let key = keys.find_key(&value.key.as_ref().unwrap().name).unwrap();
        let value = value.value.as_ref().unwrap().context_value.as_ref().unwrap();
        match value {
            ContextValue::String(ctx_value) => context.put_any::<String>(key.as_ref(), ctx_value.value.clone()),
            ContextValue::HashesLog(_) => todo!(),
            ContextValue::NamesLog(_) => todo!(),
        }
    }

    context
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
    } else {
        None
    }
}

fn try_convert_to_string_context_value(value: &dyn Any) -> Option<GrpcContextValue> {
    if !value.is::<String>() {
        None
    } else {
        Some(GrpcContextValue {
            context_value: Some(ContextValue::String(GrpcStringContextValue {
                value: value.downcast_ref::<String>().unwrap().clone(),
            })),
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
