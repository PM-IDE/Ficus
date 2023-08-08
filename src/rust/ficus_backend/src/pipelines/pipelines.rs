use std::collections::HashMap;

use super::types::Types;

pub struct PipelinePart {
    name: String,
    executor: Box<dyn Fn() -> ()>,
}

impl PipelinePart {
    pub fn new(name: String, executor: Box<dyn Fn() -> ()>) -> Self {
        Self { name, executor }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct PipelineParts {
    names_to_parts: HashMap<String, PipelinePart>,
}

impl PipelineParts {
    pub fn new(types: &Types) -> Self {
        let parts = vec![Self::read_log_from_xes(types), Self::write_log_to_xes(types)];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            names_to_parts.insert((&part.name).to_owned(), part);
        }

        Self { names_to_parts }
    }

    fn read_log_from_xes(types: &Types) -> PipelinePart {
        PipelinePart::new("ReadLogFromXes".to_string(), Box::new(|| {}))
    }

    fn write_log_to_xes(types: &Types) -> PipelinePart {
        PipelinePart::new("WriteLogToXes".to_string(), Box::new(|| {}))
    }
}
