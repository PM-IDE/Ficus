use std::collections::HashMap;

use crate::event_log::xes::{reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_log};

use super::context::PipelineContext;

pub struct PipelinePart {
    name: String,
    executor: Box<dyn Fn(&mut PipelineContext) -> ()>,
}

impl PipelinePart {
    pub fn new(name: String, executor: Box<dyn Fn(&mut PipelineContext) -> ()>) -> Self {
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
    pub fn new() -> Self {
        let parts = vec![Self::read_log_from_xes(), Self::write_log_to_xes()];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            names_to_parts.insert((&part.name).to_owned(), part);
        }

        Self { names_to_parts }
    }

    fn read_log_from_xes() -> PipelinePart {
        PipelinePart::new(
            "ReadLogFromXes".to_string(),
            Box::new(|context| {
                let path = context.get(&context.types().path()).unwrap();
                context.put(&context.types().event_log(), Box::new(read_event_log(path).unwrap()))
            }),
        )
    }

    fn write_log_to_xes() -> PipelinePart {
        PipelinePart::new(
            "WriteLogToXes".to_string(),
            Box::new(|context| {
                let path = context.get(&context.types().path()).unwrap();
                write_log(&context.get(&context.types().event_log()).unwrap(), path).ok();
            }),
        )
    }
}
