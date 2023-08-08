use crate::utils::user_data::UserData;

use super::types::PipelineType;

pub struct PipelineContext {
    user_data: UserData
}

impl PipelineContext {
    pub fn new() -> Self {
        Self { user_data: UserData::new() }
    }


    fn get<T: Clone + 'static>(&self, pipeline_type: PipelineType<T>) -> Option<&T> {
        self.user_data.get(pipeline_type.key())
    }

    fn put<T: Clone + 'static>(&mut self, pipeline_type: PipelineType<T>, value: Box<T>) {
        self.user_data.put(pipeline_type.key(), value)
    }
}