use std::{any::Any, sync::Arc};

use crate::utils::user_data::UserData;

use super::keys::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys::ContextKeys,
};

#[derive(Clone)]
pub struct PipelineContext {
    user_data: UserData,
    types: Arc<Box<ContextKeys>>,
}

impl PipelineContext {
    pub fn new(types: &Arc<Box<ContextKeys>>) -> Self {
        Self {
            user_data: UserData::new(),
            types: Arc::clone(types),
        }
    }

    pub fn types(&self) -> Arc<Box<ContextKeys>> {
        Arc::clone(&self.types)
    }

    pub fn get_any(&self, key: &dyn ContextKey) -> Option<&dyn Any> {
        self.user_data.get_any(key.key())
    }

    pub fn get_concrete<T: 'static>(&self, key: &DefaultContextKey<T>) -> Option<&T> {
        self.user_data.get(key.key())
    }

    pub fn put_concrete<T: 'static>(&mut self, key: &DefaultContextKey<T>, value: T) {
        self.user_data.put(key.key(), value)
    }

    pub fn put_any<T: 'static>(&mut self, key: &dyn ContextKey, value: T) {
        self.user_data.put(key.key(), value)
    }
}
