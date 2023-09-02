use std::{
    any::Any,
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex},
};

use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use super::{converters::{convert_to_grpc_context_value, create_initial_context, put_into_user_data}, get_context_pipeline::GetContextValuePipelinePart};
use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, grpc_get_context_value_result::ContextValueResult,
        grpc_pipeline_final_result::ExecutionResult, grpc_pipeline_part_base::Part, GrpcContextKeyValue,
        GrpcGetContextValueRequest, GrpcGetContextValueResult, GrpcGuid, GrpcPipeline, GrpcPipelineExecutionRequest,
        GrpcPipelineFinalResult, GrpcPipelinePartExecutionResult,
    },
    pipelines::{
        context::PipelineContext,
        errors::pipeline_errors::PipelinePartExecutionError,
        keys::{context_key::ContextKey, context_keys::ContextKeys},
        pipelines::{Pipeline, PipelinePart, PipelineParts},
    },
    utils::user_data::user_data::{UserData, UserDataImpl},
};

pub(super) type GrpcResult = crate::ficus_proto::grpc_pipeline_part_execution_result::Result;
pub(super) type GrpcSender = mpsc::Sender<Result<GrpcPipelinePartExecutionResult, Status>>;

pub struct FicusService {
    pipeline_parts: Arc<Box<PipelineParts>>,
    context_keys: Arc<Box<ContextKeys>>,
    contexts: Arc<Box<Mutex<HashMap<String, PipelineContext>>>>,
}

impl FicusService {
    pub fn new(types: Arc<Box<ContextKeys>>) -> Self {
        Self {
            pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
            context_keys: types,
            contexts: Arc::new(Box::new(Mutex::new(HashMap::new()))),
        }
    }
}

#[tonic::async_trait]
impl GrpcBackendService for FicusService {
    type ExecutePipelineStream =
        Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

    async fn execute_pipeline(
        &self,
        request: Request<GrpcPipelineExecutionRequest>,
    ) -> Result<Response<Self::ExecutePipelineStream>, Status> {
        let (sender, receiver) = mpsc::channel(4);
        let sender = Arc::new(Box::new(sender));
        let context_keys = self.context_keys.clone();
        let pipeline_parts = self.pipeline_parts.clone();
        let contexts = self.contexts.clone();

        tokio::task::spawn_blocking(move || {
            let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();
            let initial_context_values = &request.get_ref().initial_context;

            match Self::execute_grpc_pipeline(
                grpc_pipeline,
                initial_context_values,
                context_keys,
                pipeline_parts,
                sender.clone(),
            ) {
                Ok((guid, context)) => {
                    contexts.lock().as_mut().unwrap().insert(guid.guid.to_owned(), context);

                    sender
                        .blocking_send(Ok(Self::create_final_result(ExecutionResult::Success(guid))))
                        .ok();
                }
                Err(error) => {
                    sender
                        .blocking_send(Ok(Self::create_final_result(ExecutionResult::Error(error.to_string()))))
                        .ok();
                }
            };
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
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
                    Some(value) => match value.get_any(key.key()) {
                        None => {
                            key.try_create_value_into_context(value, &self.context_keys);

                            if let Some(created_value) = value.get_any(key.key()) {
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
        grpc_pipeline: &GrpcPipeline,
        initial_context_value: &Vec<GrpcContextKeyValue>,
        context_keys: Arc<Box<ContextKeys>>,
        pipeline_parts: Arc<Box<PipelineParts>>,
        sender: Arc<Box<GrpcSender>>,
    ) -> Result<(GrpcGuid, PipelineContext), PipelinePartExecutionError> {
        let id = Uuid::new_v4();
        let pipeline = Self::to_pipeline(grpc_pipeline, &context_keys, &pipeline_parts, sender);
        let mut context = create_initial_context(initial_context_value, &context_keys);

        match pipeline.execute(&mut context, &context_keys) {
            Ok(()) => Ok((GrpcGuid { guid: id.to_string() }, context)),
            Err(err) => Err(err),
        }
    }

    fn to_pipeline(
        grpc_pipeline: &GrpcPipeline,
        context_keys: &ContextKeys,
        pipeline_parts: &PipelineParts,
        sender: Arc<Box<GrpcSender>>,
    ) -> Pipeline {
        let mut pipeline = Pipeline::empty();
        for grpc_part in &grpc_pipeline.parts {
            match grpc_part.part.as_ref().unwrap() {
                Part::DefaultPart(grpc_default_part) => {
                    let mut part_config = UserDataImpl::new();
                    let grpc_config = &grpc_default_part.configuration.as_ref().unwrap();

                    for conf_value in &grpc_config.configuration_parameters {
                        let key_name = conf_value.key.as_ref().unwrap().name.as_ref();
                        if let Some(key) = context_keys.find_key(key_name) {
                            let value = conf_value.value.as_ref().unwrap().context_value.as_ref().unwrap();
                            put_into_user_data(key.key(), value, &mut part_config);
                        }
                    }

                    match pipeline_parts.find_part(&grpc_default_part.name) {
                        Some(default_part) => pipeline.push(Box::new(default_part(Box::new(part_config)))),
                        None => todo!(),
                    };
                }
                Part::ParallelPart(_) => todo!(),
                Part::ContextRequestPart(part) => {
                    let key_name = part.key.as_ref().unwrap().name.clone();
                    let sender = sender.clone();

                    pipeline.push(GetContextValuePipelinePart::create_get_context_pipeline_part(key_name, sender));
                }
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

    fn create_final_result(execution_result: ExecutionResult) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::FinalResult(GrpcPipelineFinalResult {
                execution_result: Some(execution_result),
            })),
        }
    }
}
