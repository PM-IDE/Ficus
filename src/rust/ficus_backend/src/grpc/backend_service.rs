use tonic::{Request, Response, Status};

use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, GrpcContextValue, GrpcGetContextValueRequest, GrpcGuid,
        GrpcPipelineExecutionRequest,
    },
    pipelines::pipelines::PipelineParts,
};

pub struct FicusService {
    pipeline_parts: PipelineParts,
}

impl FicusService {
    pub fn new() -> Self {
        Self {
            pipeline_parts: PipelineParts::new(),
        }
    }
}

#[tonic::async_trait]
impl GrpcBackendService for FicusService {
    async fn execute_pipeline(
        &self,
        request: Request<GrpcPipelineExecutionRequest>,
    ) -> Result<Response<GrpcGuid>, Status> {
        let execution_request = request.get_ref();
        todo!();
    }

    async fn get_context_value(
        &self,
        request: Request<GrpcGetContextValueRequest>,
    ) -> Result<Response<GrpcContextValue>, Status> {
        todo!();
    }
}
