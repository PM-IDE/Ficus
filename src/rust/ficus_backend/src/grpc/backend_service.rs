use tonic::{Request, Response, Status};

use crate::ficus_proto::{
    backend_service_server::BackendService, ContextValue, GrpcGuid, PipelineExecutionRequest, GetContextValueRequest,
};

pub struct FicusService {}

#[tonic::async_trait]
impl BackendService for FicusService {
    async fn execute_pipeline(&self, request: Request<PipelineExecutionRequest>) -> Result<Response<GrpcGuid>, Status> {
        todo!();
    }

    async fn get_context_value(&self, request: Request<GetContextValueRequest>) -> Result<Response<ContextValue>, Status> {
        todo!();
    }
}
