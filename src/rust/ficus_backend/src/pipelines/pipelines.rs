use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
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

pub struct Pipeline {
    parts: Vec<Rc<Box<dyn PipelinePart>>>,
}

impl Pipeline {
    pub fn empty() -> Self {
        Self { parts: vec![] }
    }

    pub fn push(&mut self, part: Rc<Box<dyn PipelinePart>>) {
        self.parts.push(part);
    }
}

impl PipelinePart for Pipeline {
    fn execute(&self, context: &mut PipelineContext) -> Result<(), PipelinePartExecutionError> {
        for part in &self.parts {
            part.execute(context)?;
        }

        Ok(())
    }
}

pub trait PipelinePart {
    fn execute(&self, context: &mut PipelineContext) -> Result<(), PipelinePartExecutionError>;
}

pub struct ParallelPipelinePart {
    parallel_pipelines: Vec<Pipeline>,
}

impl PipelinePart for ParallelPipelinePart {
    fn execute(&self, context: &mut PipelineContext) -> Result<(), PipelinePartExecutionError> {
        for pipeline in &self.parallel_pipelines[0..(self.parallel_pipelines.len() - 1)] {
            pipeline.execute(&mut context.clone())?;
        }

        if let Some(last_pipeline) = self.parallel_pipelines.last() {
            last_pipeline.execute(context)?;
        }

        Ok(())
    }
}

pub struct DefaultPipelinePart {
    name: String,
    executor: Box<dyn Fn(&mut PipelineContext) -> Result<(), PipelinePartExecutionError>>,
}

impl DefaultPipelinePart {
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

impl PipelinePart for DefaultPipelinePart {
    fn execute(&self, context: &mut PipelineContext) -> Result<(), PipelinePartExecutionError> {
        (self.executor)(context)
    }
}

pub struct PipelineParts {
    names_to_parts: HashMap<String, Rc<Box<dyn PipelinePart>>>,
}

impl PipelineParts {
    pub fn find_part(&self, name: &String) -> Option<Rc<Box<dyn PipelinePart>>> {
        match self.names_to_parts.get(name) {
            Some(part) => Some(Rc::clone(part)),
            None => None,
        }
    }
}

impl PipelineParts {
    pub fn new() -> Self {
        let parts = vec![Self::read_log_from_xes(), Self::write_log_to_xes()];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            names_to_parts.insert(
                (&part.name).to_owned(),
                Rc::new(Box::new(part) as Box<dyn PipelinePart>),
            );
        }

        Self { names_to_parts }
    }

    fn read_log_from_xes() -> DefaultPipelinePart {
        DefaultPipelinePart::new(
            "ReadLogFromXes".to_string(),
            Box::new(|context| {
                let path = context.get_concrete(&context.types().path()).unwrap();
                let log = read_event_log(path);
                if log.is_none() {
                    let message = format!("Failed to read event log from {}", path.as_str());
                    return Err(PipelinePartExecutionError::new(message));
                }

                context.put_concrete(&context.types().event_log(), Box::new(log.unwrap()));
                Ok(())
            }),
        )
    }

    fn write_log_to_xes() -> DefaultPipelinePart {
        DefaultPipelinePart::new(
            "WriteLogToXes".to_string(),
            Box::new(|context| {
                let path = context.get_concrete(&context.types().path()).unwrap();
                match write_log(&context.get_concrete(&context.types().event_log()).unwrap(), path) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(PipelinePartExecutionError::new(err.to_string())),
                }
            }),
        )
    }
}

unsafe impl Sync for PipelineParts {}

unsafe impl Send for PipelineParts {}
