use std::{rc::Rc, sync::Arc};

use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, grpc_pipeline_part_base::Part, GrpcContextValue,
        GrpcGetContextValueRequest, GrpcGuid, GrpcPipeline, GrpcPipelineExecutionRequest,
    },
    pipelines::{
        context::PipelineContext,
        pipelines::{Pipeline, PipelinePart, PipelinePartExecutionError, PipelineParts},
        types::Types,
    },
};

pub struct FicusService {
    pipeline_parts: PipelineParts,
    types: Arc<Box<Types>>,
}

impl FicusService {
    pub fn new(types: Arc<Box<Types>>) -> Self {
        Self {
            pipeline_parts: PipelineParts::new(),
            types,
        }
    }
}

#[tonic::async_trait]
impl GrpcBackendService for FicusService {
    async fn execute_pipeline(
        &self,
        request: Request<GrpcPipelineExecutionRequest>,
    ) -> Result<Response<GrpcGuid>, Status> {
        let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();
        self.execute_grpc_pipeline(grpc_pipeline).ok();

        todo!();
    }

    async fn get_context_value(
        &self,
        request: Request<GrpcGetContextValueRequest>,
    ) -> Result<Response<GrpcContextValue>, Status> {
        todo!();
    }
}

impl FicusService {
    fn execute_grpc_pipeline(&self, grpc_pipeline: &GrpcPipeline) -> Result<GrpcGuid, PipelinePartExecutionError> {
        let id = Uuid::new_v4();
        let pipeline = self.to_pipeline(grpc_pipeline);
        pipeline.execute(&mut PipelineContext::new(&self.types))?;

        Ok(GrpcGuid { guid: id.to_string() })
    }

    fn to_pipeline(&self, grpc_pipeline: &GrpcPipeline) -> Pipeline {
        let mut pipeline = Pipeline::empty();
        for grpc_part in &grpc_pipeline.parts {
            match grpc_part.part.as_ref().unwrap() {
                Part::DefaultPart(grpc_default_part) => match self.pipeline_parts.find_part(&grpc_default_part.name) {
                    Some(default_part) => pipeline.push(default_part),
                    None => todo!(),
                },
                Part::ParallelPart(_) => todo!(),
            }
        }

        pipeline
    }
}
