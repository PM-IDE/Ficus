use std::{any::Any, sync::Arc};

use crate::utils::user_data::{DefaultKey, Key, UserData};

use super::keys::context_keys::ContextKeys;

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

    pub fn types(&self) -> &ContextKeys {
        &self.types
    }

    pub fn get_any(&self, key: &dyn Key) -> Option<&dyn Any> {
        self.user_data.get_any(key)
    }

    pub fn get_concrete<T: 'static>(&self, key: &DefaultKey<T>) -> Option<&T> {
        self.user_data.get(key)
    }

    pub fn put_concrete<T: 'static>(&mut self, key: &DefaultKey<T>, value: T) {
        self.user_data.put(key, value)
    }

    pub fn put_any<T: 'static>(&mut self, key: &dyn Key, value: T) {
        self.user_data.put(key, value)
    }
}
