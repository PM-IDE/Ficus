use std::{any::Any, sync::Arc};

use crate::utils::user_data::{
    keys::{DefaultKey, Key},
    user_data::{UserData, UserDataImpl},
};

use super::{errors::pipeline_errors::PipelinePartExecutionError, keys::context_keys::ContextKeys};

pub trait LogMessageHandler: Send + Sync {
    fn handle(&self, message: String) -> Result<(), PipelinePartExecutionError>;
}

#[derive(Clone)]
pub struct PipelineContext {
    user_data: UserDataImpl,
    log_message_handler: Option<Arc<Box<dyn LogMessageHandler>>>,
}

impl PipelineContext {
    pub fn new_with_logging(message_handler: Arc<Box<dyn LogMessageHandler>>) -> Self {
        Self {
            user_data: UserDataImpl::new(),
            log_message_handler: Some(message_handler),
        }
    }

    pub fn empty() -> Self {
        Self {
            user_data: UserDataImpl::new(),
            log_message_handler: None,
        }
    }
}

impl UserData for PipelineContext {
    fn get_any(&self, key: &dyn Key) -> Option<&dyn Any> {
        self.user_data.get_any(key)
    }

    fn get_concrete<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&T> {
        self.user_data.get_concrete(key)
    }

    fn put_concrete<T: 'static>(&mut self, key: &DefaultKey<T>, value: T) {
        self.user_data.put_concrete(key, value)
    }

    fn put_any<T: 'static>(&mut self, key: &dyn Key, value: T) {
        self.user_data.put_any(key, value)
    }

    fn get_concrete_mut<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&mut T> {
        self.user_data.get_concrete_mut(key)
    }

    fn remove_concrete<T: 'static>(&mut self, key: &DefaultKey<T>) {
        self.user_data.remove_concrete(key)
    }

    fn remove_any<T: 'static>(&mut self, key: &dyn Key) {
        self.user_data.remove_any::<T>(key)
    }

    fn len(&self) -> usize {
        self.user_data.len()
    }
}

impl PipelineContext {
    pub fn log(&self, message: String) -> Result<(), PipelinePartExecutionError> {
        if let Some(handler) = self.log_message_handler.as_ref() {
            handler.handle(message)
        } else {
            Ok(())
        }
    }
}
