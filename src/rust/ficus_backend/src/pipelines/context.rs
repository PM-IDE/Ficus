use std::{rc::Rc, sync::Arc};

use crate::utils::user_data::UserData;

use super::types::{PipelineType, Types};

#[derive(Clone)]
pub struct PipelineContext {
    user_data: UserData,
    types: Arc<Box<Types>>,
}

impl PipelineContext {
    pub fn new(types: &Arc<Box<Types>>) -> Self {
        Self {
            user_data: UserData::new(),
            types: Arc::clone(types),
        }
    }

    pub fn types(&self) -> Arc<Box<Types>> {
        Arc::clone(&self.types)
    }

    pub fn get<T: 'static>(&self, pipeline_type: &PipelineType<T>) -> Option<&T> {
        self.user_data.get(pipeline_type.key())
    }

    pub fn put<T: 'static>(&mut self, pipeline_type: &PipelineType<T>, value: Box<T>) {
        self.user_data.put(pipeline_type.key(), value)
    }
}
