use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
};

use crate::event_log::xes::{reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_log};

use super::context::PipelineContext;

pub struct PipelinePartExecutionError {
    message: String,
}

impl Display for PipelinePartExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Debug for PipelinePartExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelinePartExecutionError")
            .field("message", &self.message)
            .finish()
    }
}

impl Error for PipelinePartExecutionError {}

impl PipelinePartExecutionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub struct PipelinePart {
    name: String,
    executor: Box<dyn Fn(&mut PipelineContext) -> Result<(), PipelinePartExecutionError>>,
}

impl PipelinePart {
    pub fn new(
        name: String,
        executor: Box<dyn Fn(&mut PipelineContext) -> Result<(), PipelinePartExecutionError>>,
    ) -> Self {
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
                let log = read_event_log(path);
                if log.is_none() {
                    return Err(PipelinePartExecutionError::new("Failed to read event log".to_string()));
                }

                context.put(&context.types().event_log(), Box::new(log.unwrap()));
                Ok(())
            }),
        )
    }

    fn write_log_to_xes() -> PipelinePart {
        PipelinePart::new(
            "WriteLogToXes".to_string(),
            Box::new(|context| {
                let path = context.get(&context.types().path()).unwrap();
                match write_log(&context.get(&context.types().event_log()).unwrap(), path) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(PipelinePartExecutionError::new(err.to_string())),
                }
            }),
        )
    }
}
