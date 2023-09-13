use std::collections::HashMap;

use crate::{
    pipelines::errors::pipeline_errors::MissingContextError,
    utils::{
        colors::ColorsHolder,
        user_data::{
            keys::Key,
            user_data::{UserData, UserDataImpl},
        },
    },
};

use super::{
    context::PipelineContext,
    errors::pipeline_errors::PipelinePartExecutionError,
    keys::{context_key::DefaultContextKey, context_keys::ContextKeys},
};

pub struct Pipeline {
    parts: Vec<Box<dyn PipelinePart>>,
}

impl Pipeline {
    pub fn empty() -> Self {
        Self { parts: vec![] }
    }

    pub fn push(&mut self, part: Box<dyn PipelinePart>) {
        self.parts.push(part);
    }
}

impl PipelinePart for Pipeline {
    fn execute(&self, context: &mut PipelineContext, keys: &ContextKeys) -> Result<(), PipelinePartExecutionError> {
        self.put_default_concrete_keys(context, keys);

        for part in &self.parts {
            part.execute(context, keys)?;
        }

        Ok(())
    }
}

impl Pipeline {
    fn put_default_concrete_keys(&self, context: &mut PipelineContext, keys: &ContextKeys) {
        context.put_concrete(keys.colors_holder().key(), ColorsHolder::empty());
    }
}

pub trait PipelinePart {
    fn execute(&self, context: &mut PipelineContext, keys: &ContextKeys) -> Result<(), PipelinePartExecutionError>;
}

pub struct ParallelPipelinePart {
    parallel_pipelines: Vec<Pipeline>,
}

impl PipelinePart for ParallelPipelinePart {
    fn execute(&self, context: &mut PipelineContext, keys: &ContextKeys) -> Result<(), PipelinePartExecutionError> {
        for pipeline in &self.parallel_pipelines[0..(self.parallel_pipelines.len() - 1)] {
            pipeline.execute(&mut context.clone(), keys)?;
        }

        if let Some(last_pipeline) = self.parallel_pipelines.last() {
            last_pipeline.execute(context, keys)?;
        }

        Ok(())
    }
}

type PipelinePartExecutor =
    Box<dyn Fn(&mut PipelineContext, &ContextKeys, &UserDataImpl) -> Result<(), PipelinePartExecutionError>>;

pub struct DefaultPipelinePart {
    name: String,
    config: Box<UserDataImpl>,
    executor: PipelinePartExecutor,
}

impl DefaultPipelinePart {
    pub fn new(name: String, config: Box<UserDataImpl>, executor: PipelinePartExecutor) -> Self {
        Self { name, config, executor }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn config(&self) -> &UserDataImpl {
        &self.config
    }
}

impl PipelinePart for DefaultPipelinePart {
    fn execute(&self, context: &mut PipelineContext, keys: &ContextKeys) -> Result<(), PipelinePartExecutionError> {
        (self.executor)(context, keys, &self.config)
    }
}

pub(super) type PipelinePartFactory = Box<dyn Fn(Box<UserDataImpl>) -> DefaultPipelinePart>;

pub struct PipelineParts {
    names_to_parts: HashMap<String, PipelinePartFactory>,
}

impl PipelineParts {
    pub fn find_part(&self, name: &str) -> Option<&PipelinePartFactory> {
        self.names_to_parts.get(name)
    }
}

unsafe impl Sync for PipelineParts {}
unsafe impl Send for PipelineParts {}

impl PipelineParts {
    pub fn new() -> Self {
        let parts = vec![
            Self::read_log_from_xes(),
            Self::write_log_to_xes(),
            Self::find_primitive_tandem_arrays(),
            Self::find_maximal_tandem_arrays(),
            Self::find_maximal_repeats(),
            Self::find_super_maximal_repeats(),
            Self::find_near_super_maximal_repeats(),
            Self::discover_activities(),
            Self::discover_activities_instances(),
            Self::create_log_from_activities(),
            Self::filter_log_by_event_name(),
            Self::filter_log_by_regex(),
            Self::filter_log_by_variants(),
            Self::draw_placements_of_event_by_name(),
            Self::draw_events_placements_by_regex(),
            Self::draw_full_activities_diagram(),
            Self::draw_short_activities_diagram(),
            Self::get_event_log_info(),
            Self::clear_activities_related_stuff(),
            Self::get_number_of_underlying_events(),
            Self::filter_traces_by_count(),
            Self::traces_diversity_diagram(),
            Self::get_names_event_log(),
            Self::get_hashes_event_log(),
            Self::use_names_event_log(),
            Self::discover_activities_instances_for_several_levels(),
            Self::discover_activities_in_unattached_subtraces(),
            Self::discover_activities_from_pattern_source(),
        ];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            let prev = names_to_parts.insert((&part.0).to_owned(), part.1);
            assert!(prev.is_none());
        }

        Self { names_to_parts }
    }

    pub fn len(&self) -> usize {
        self.names_to_parts.len()
    }

    pub(super) fn create_pipeline_part(
        name: &'static str,
        executor: &'static impl Fn(
            &mut PipelineContext,
            &ContextKeys,
            &UserDataImpl,
        ) -> Result<(), PipelinePartExecutionError>,
    ) -> (String, PipelinePartFactory) {
        (
            name.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    name.to_string(),
                    config,
                    Box::new(|context, keys, config| executor(context, keys, config)),
                )
            }),
        )
    }

    pub(super) fn get_context_value<'a, T>(
        context: &'a impl UserData,
        key: &DefaultContextKey<T>,
    ) -> Result<&'a T, PipelinePartExecutionError> {
        match context.get_concrete(key.key()) {
            Some(value) => Ok(value),
            None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                key.key().name().to_owned(),
            ))),
        }
    }

    pub(super) fn get_context_value_mut<'a, T>(
        context: &'a PipelineContext,
        key: &DefaultContextKey<T>,
    ) -> Result<&'a mut T, PipelinePartExecutionError> {
        match context.get_concrete_mut(key.key()) {
            Some(value) => Ok(value),
            None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                key.key().name().to_owned(),
            ))),
        }
    }
}
