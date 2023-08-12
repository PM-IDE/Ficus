use crate::{event_log::{simple::simple_event_log::SimpleEventLog, xes::{xes_event_log::XesEventLogImpl}}, ficus_proto::GrpcContextValue};

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
