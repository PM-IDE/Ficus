use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, grpc_get_context_value_result::ContextValueResult,
        grpc_pipeline_execution_result::ExecutionResult, grpc_pipeline_part_base::Part, GrpcContextKeyValue,
        GrpcGetContextValueRequest, GrpcGetContextValueResult, GrpcGuid, GrpcPipeline, GrpcPipelineExecutionRequest,
        GrpcPipelineExecutionResult,
    },
    pipelines::{
        context::PipelineContext,
        keys::{context_key::ContextKey, context_keys::ContextKeys},
        pipelines::{Pipeline, PipelinePart, PipelinePartExecutionError, PipelineParts},
    },
    utils::user_data::UserData,
};

use super::converters::{convert_to_grpc_context_value, create_initial_context};

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
        let initial_context_values = &request.get_ref().initial_context;

        let result = match self.execute_grpc_pipeline(grpc_pipeline, initial_context_values) {
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
                match self.contexts.lock().as_mut().unwrap().get_mut(&id.guid) {
                    None => Self::create_get_context_value_error("Failed to get context for guid".to_string()),
                    Some(value) => match value.get_any(key.as_ref()) {
                        None => {
                            key.try_create_value_into_context(value, &self.context_keys);

                            if let Some(created_value) = value.get_any(key.as_ref()) {
                                self.try_convert_context_value(key, created_value)
                            } else {
                                Self::create_get_context_value_error("Failed to find context value for key".to_string())
                            }
                        }
                        Some(context_value) => self.try_convert_context_value(key, context_value),
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
        initial_context_value: &Vec<GrpcContextKeyValue>,
    ) -> Result<(GrpcGuid, PipelineContext), PipelinePartExecutionError> {
        let id = Uuid::new_v4();
        let pipeline = self.to_pipeline(grpc_pipeline);
        let mut context = create_initial_context(initial_context_value, &self.context_keys);

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
                    Some(default_part) => pipeline.push(Box::new(default_part(UserData::new()))),
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

    fn try_convert_context_value(
        &self,
        key: &Box<dyn ContextKey>,
        context_value: &dyn Any,
    ) -> GrpcGetContextValueResult {
        let value = convert_to_grpc_context_value(key.as_ref(), context_value, &self.context_keys);
        if let Some(grpc_context_value) = value {
            GrpcGetContextValueResult {
                context_value_result: Some(ContextValueResult::Value(grpc_context_value)),
            }
        } else {
            let msg = "Can not convert context value to grpc model".to_string();
            Self::create_get_context_value_error(msg)
        }
    }
}
