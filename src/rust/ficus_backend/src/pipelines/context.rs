use std::{any::Any, sync::Arc};

use crate::utils::user_data::{
    keys::{DefaultKey, Key},
    user_data::{UserData, UserDataImpl},
};

use super::keys::context_keys::ContextKeys;

#[derive(Clone)]
pub struct PipelineContext {
    user_data: UserDataImpl,
    types: Arc<Box<ContextKeys>>,
}

impl PipelineContext {
    pub fn new(types: &Arc<Box<ContextKeys>>) -> Self {
        Self {
            user_data: UserDataImpl::new(),
            types: Arc::clone(types),
        }
    }

    pub fn types(&self) -> &ContextKeys {
        &self.types
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
}
