use std::{
    collections::HashMap,
    rc::Rc,
};

use super::types::{PipelineType, Types};

pub struct PipelinePart {
    name: String,
    required_types: Vec<Rc<Box<PipelineType>>>,
    modified_types: Option<Vec<Rc<Box<PipelineType>>>>,
}

impl PipelinePart {
    pub fn new(
        name: String,
        required_types: Vec<Rc<Box<PipelineType>>>,
        modified_types: Option<Vec<Rc<Box<PipelineType>>>>,
    ) -> Self {
        Self {
            name,
            required_types,
            modified_types,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn required_types(&self) -> &Vec<Rc<Box<PipelineType>>> {
        &self.required_types
    }

    pub fn modified_types(&self) -> Option<&Vec<Rc<Box<PipelineType>>>> {
        self.modified_types.as_ref()
    }
}

pub struct PipelineParts {
    names_to_parts: HashMap<String, PipelinePart>,
}

impl PipelineParts {
    pub fn new(types: &Types) -> Self {
        let parts = vec![
            PipelinePart::new("ReadLogFromXes".to_string(), vec![types.path()], Some(vec![types.event_log()])),
            PipelinePart::new("WriteLogToXes".to_string(), vec![types.event_log()], None),
        ];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            names_to_parts.insert((&part.name).to_owned(), part);
        }

        Self { names_to_parts }
    }
}
