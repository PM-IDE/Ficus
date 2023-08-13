use std::{any::Any, sync::Arc};

use crate::{
    ficus_proto::{
        grpc_context_value::ContextValue, GrpcContextKeyValue, GrpcContextValue, GrpcHashesEventLog,
        GrpcHashesEventLogContextValue, GrpcHashesLogTrace, GrpcStringContextValue,
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
        return try_convert_to_string_context_value(value);
    }

    if keys.is_hashes_event_log(key) {
        return try_convert_to_hashes_event_log(value);
    }

    None
}

fn try_convert_to_string_context_value(value: &dyn Any) -> Option<GrpcContextValue> {
    if value.is::<String>() {
        let value = GrpcStringContextValue {
            value: value.downcast_ref::<String>().unwrap().clone(),
        };

        return Some(GrpcContextValue {
            context_value: Some(ContextValue::String(value)),
        });
    }

    None
}

fn try_convert_to_hashes_event_log(value: &dyn Any) -> Option<GrpcContextValue> {
    if value.is::<Vec<Vec<u64>>>() {
        let vec = value.downcast_ref::<Vec<Vec<u64>>>().unwrap();
        let mut traces = vec![];
        for trace in vec {
            let mut events = vec![];
            for event in trace {
                events.push(*event);
            }

            traces.push(GrpcHashesLogTrace { events });
        }

        let value = GrpcHashesEventLogContextValue {
            log: Some(GrpcHashesEventLog { traces }),
        };

        return Some(GrpcContextValue {
            context_value: Some(ContextValue::HashesLog(value)),
        });
    }

    None
}
