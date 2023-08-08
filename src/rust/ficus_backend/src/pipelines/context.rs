use std::rc::Rc;

use crate::utils::user_data::UserData;

use super::types::{PipelineType, Types};

pub struct PipelineContext {
    user_data: UserData,
    types: Rc<Box<Types>>,
}

impl PipelineContext {
    pub fn new(types: Rc<Box<Types>>) -> Self {
        Self {
            user_data: UserData::new(),
            types: Rc::clone(&types),
        }
    }

    pub fn types(&self) -> Rc<Box<Types>> {
        Rc::clone(&self.types)
    }

    pub fn get<T: 'static>(&self, pipeline_type: &PipelineType<T>) -> Option<&T> {
        self.user_data.get(pipeline_type.key())
    }

    pub fn put<T: 'static>(&mut self, pipeline_type: &PipelineType<T>, value: Box<T>) {
        self.user_data.put(pipeline_type.key(), value)
    }
}
