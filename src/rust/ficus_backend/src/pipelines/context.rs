use std::{rc::Rc, sync::Arc};

use crate::utils::user_data::UserData;

use super::types::{ContextKeys, DefaultContextKey};

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

    pub fn get<T: 'static>(&self, pipeline_type: &DefaultContextKey<T>) -> Option<&T> {
        self.user_data.get(pipeline_type.key())
    }

    pub fn put<T: 'static>(&mut self, pipeline_type: &DefaultContextKey<T>, value: Box<T>) {
        self.user_data.put(pipeline_type.key(), value)
    }
}
