use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, grpc_get_context_value_result::ContextValueResult,
        grpc_pipeline_execution_result::ExecutionResult, grpc_pipeline_part_base::Part, GrpcGetContextValueRequest,
        GrpcGetContextValueResult, GrpcGuid, GrpcPipeline, GrpcPipelineExecutionRequest, GrpcPipelineExecutionResult,
    },
    pipelines::{
        context::PipelineContext,
        keys::context_keys::ContextKeys,
        pipelines::{Pipeline, PipelinePart, PipelinePartExecutionError, PipelineParts},
    },
};

use super::converters::IntoGrpcContextValue;

pub struct FicusService {
    pipeline_parts: PipelineParts,
    context_keys: Arc<Box<ContextKeys>>,
    contexts: Mutex<HashMap<String, PipelineContext>>,
}

impl FicusService {
    pub fn new(types: Arc<Box<ContextKeys>>) -> Self {
        Self {
            pipeline_parts: PipelineParts::new(),
            context_keys: types,
            contexts: Mutex::new(HashMap::new()),
        }
    }
}

#[tonic::async_trait]
impl GrpcBackendService for FicusService {
    async fn execute_pipeline(
        &self,
        request: Request<GrpcPipelineExecutionRequest>,
    ) -> Result<Response<GrpcPipelineExecutionResult>, Status> {
        let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();

        let result = match self.execute_grpc_pipeline(grpc_pipeline) {
            Ok((guid, context)) => {
                self.contexts
                    .lock()
                    .as_mut()
                    .unwrap()
                    .insert(guid.guid.to_owned(), context);

                GrpcPipelineExecutionResult {
                    execution_result: Some(ExecutionResult::Success(guid)),
                }
            }
            Err(error) => GrpcPipelineExecutionResult {
                execution_result: Some(ExecutionResult::Error(error.to_string())),
            },
        };

        Ok(Response::new(result))
    }

    async fn get_context_value(
        &self,
        request: Request<GrpcGetContextValueRequest>,
    ) -> Result<Response<GrpcGetContextValueResult>, Status> {
        let key_name = &request.get_ref().key.as_ref().unwrap().name;
        let result = match self.context_keys.find_key(key_name) {
            None => Self::create_get_context_value_error("Failed to find key for key name".to_string()),
            Some(key) => {
                let id = request.get_ref().execution_id.as_ref().unwrap();
                match self.contexts.lock().as_ref().unwrap().get(&id.guid) {
                    None => Self::create_get_context_value_error("Failed to get context for guid".to_string()),
                    Some(value) => match value.get_any(key.as_ref()) {
                        None => {
                            Self::create_get_context_value_error("Failed to find context value for key".to_string())
                        }
                        Some(context_value) => {
                            if !context_value.is::<&dyn IntoGrpcContextValue>() {
                                let msg = "Can not convert context value to grpc model".to_string();
                                Self::create_get_context_value_error(msg)
                            } else {
                                let context_value = context_value.downcast_ref::<&dyn IntoGrpcContextValue>().unwrap();
                                let model = context_value.to_grpc_context_value();

                                GrpcGetContextValueResult {
                                    context_value_result: Some(ContextValueResult::Value(model)),
                                }
                            }
                        }
                    },
                }
            }
        };

        Ok(Response::new(result))
    }
}

impl FicusService {
    fn execute_grpc_pipeline(
        &self,
        grpc_pipeline: &GrpcPipeline,
    ) -> Result<(GrpcGuid, PipelineContext), PipelinePartExecutionError> {
        let id = Uuid::new_v4();
        let pipeline = self.to_pipeline(grpc_pipeline);
        let mut context = PipelineContext::new(&self.context_keys);
        match pipeline.execute(&mut context) {
            Ok(()) => Ok((GrpcGuid { guid: id.to_string() }, context)),
            Err(err) => Err(err),
        }
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

    fn create_get_context_value_error(message: String) -> GrpcGetContextValueResult {
        GrpcGetContextValueResult {
            context_value_result: Some(ContextValueResult::Error(message)),
        }
    }
}
