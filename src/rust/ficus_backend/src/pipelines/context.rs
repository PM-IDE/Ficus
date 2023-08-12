use std::sync::Arc;

use crate::utils::user_data::UserData;

use super::context_keys::{ContextKeys, DefaultContextKey};

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

    pub fn get_concrete<T: 'static>(&self, key: &DefaultContextKey<T>) -> Option<&T> {
        self.user_data.get(key.key())
    }

    pub fn put_concrete<T: 'static>(&mut self, key: &DefaultContextKey<T>, value: Box<T>) {
        self.user_data.put(key.key(), value)
    }
}
