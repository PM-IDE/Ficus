use std::sync::Arc;

use crate::{
    ficus_proto::{GrpcPipelinePartExecutionResult, GrpcPipelinePartResult},
    pipelines::{
        context::PipelineContext,
        errors::pipeline_errors::{MissingContextError, PipelinePartExecutionError},
        keys::{context_key::ContextKey, context_keys::ContextKeys},
        pipelines::{DefaultPipelinePart, PipelinePart},
    },
    utils::user_data::user_data::UserData,
};

use super::{
    backend_service::{GrpcResult, GrpcSender},
    converters::convert_to_grpc_context_value,
};

type GetContextHandler =
    Box<dyn Fn(&mut PipelineContext, &ContextKeys, &Box<dyn ContextKey>) -> Result<(), PipelinePartExecutionError>>;

pub struct GetContextValuePipelinePart {
    before_part: Option<Box<DefaultPipelinePart>>,
    key_name: String,
    handler: GetContextHandler,
}

impl GetContextValuePipelinePart {
    pub fn new(key_name: String, handler: GetContextHandler) -> Self {
        Self {
            before_part: None,
            key_name,
            handler,
        }
    }

    pub fn new_with_default_part(
        before_part: Box<DefaultPipelinePart>,
        key_name: String,
        handler: GetContextHandler,
    ) -> Self {
        Self {
            before_part: Some(before_part),
            key_name,
            handler,
        }
    }

    pub fn create_context_pipeline_part(
        key_name: String,
        sender: Arc<Box<GrpcSender>>,
        before_part: Option<Box<DefaultPipelinePart>>,
    ) -> Box<GetContextValuePipelinePart> {
        Box::new(GetContextValuePipelinePart::new(
            key_name,
            Box::new(move |context, keys, key| {
                if let Some(before_part) = before_part.as_ref() {
                    before_part.execute(context, keys)?;
                }

                key.try_create_value_into_context(context, keys);

                match context.get_any(key.key()) {
                    Some(context_value) => {
                        let grpc_value = convert_to_grpc_context_value(key.as_ref(), context_value, keys);

                        sender
                            .blocking_send(Ok(GrpcPipelinePartExecutionResult {
                                result: Some(GrpcResult::PipelinePartResult(GrpcPipelinePartResult {
                                    context_value: grpc_value,
                                })),
                            }))
                            .ok();

                        Ok(())
                    }
                    None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                        key.key().name().clone(),
                    ))),
                }
            }),
        ))
    }
}

impl PipelinePart for GetContextValuePipelinePart {
    fn execute(&self, context: &mut PipelineContext, keys: &ContextKeys) -> Result<(), PipelinePartExecutionError> {
        match keys.find_key(&self.key_name) {
            Some(key) => {
                key.try_create_value_into_context(context, keys);
                (self.handler)(context, keys, key)
            }
            None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                self.key_name.clone(),
            ))),
        }
    }
}
