use std::collections::HashSet;

use super::types::PipelineType;

pub struct PipelinePart {
    required_types: HashSet<PipelineType>,
    created_types: HashSet<PipelineType>,
    modified_types: HashSet<PipelineType>,
}

impl PipelinePart {
    pub fn new(
        required_types: HashSet<PipelineType>,
        created_types: HashSet<PipelineType>,
        modified_types: HashSet<PipelineType>,
    ) -> Self {
        Self {
            required_types,
            created_types,
            modified_types,
        }
    }

    pub fn required_types(&self) -> &HashSet<PipelineType> {
        &self.required_types
    }

    pub fn created_types(&self) -> &HashSet<PipelineType> {
        &self.created_types
    }

    pub fn modified_types(&self) -> &HashSet<PipelineType> {
        &self.modified_types
    }
}
