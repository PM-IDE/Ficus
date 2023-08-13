use prost::Message;
use std::sync::Arc;

use prost_types::Any;

use crate::{
    event_log::{simple::simple_event_log::SimpleEventLog, xes::xes_event_log::XesEventLogImpl},
    ficus_proto::{grpc_context_value::ContextValue, GrpcContextKeyValue, GrpcContextValue, GrpcStringContextValue},
    pipelines::{context::PipelineContext, keys::context_keys::ContextKeys},
};

pub trait IntoGrpcContextValue {
    fn to_grpc_context_value(&self) -> GrpcContextValue;
}

impl IntoGrpcContextValue for SimpleEventLog {
    fn to_grpc_context_value(&self) -> GrpcContextValue {
        todo!()
    }
}

impl IntoGrpcContextValue for XesEventLogImpl {
    fn to_grpc_context_value(&self) -> GrpcContextValue {
        todo!()
    }
}

pub(super) fn create_initial_context(
    values: &Vec<GrpcContextKeyValue>,
    keys: &Arc<Box<ContextKeys>>,
) -> PipelineContext {
    let mut context = PipelineContext::new(keys);

    for value in values {
        let key = keys.find_key(&value.key.as_ref().unwrap().name).unwrap();
        let value = value.value.as_ref().unwrap().context_value.as_ref().unwrap();
        match value {
            ContextValue::String(string) => context.put_any(key.as_ref(), Box::new(string.clone())),
        }
    }

    context
}
