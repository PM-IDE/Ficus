use std::{collections::HashMap, hash};

use crate::{
    event_log::{
        core::{
            event::event_hasher::{default_class_extractor, NameEventHasher},
            event_log::EventLog,
        },
        xes::{reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_log},
    },
    features::analysis::patterns::{
        activity_instances::create_activity_name,
        contexts::PatternsDiscoveryStrategy,
        repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets},
        repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
        tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
    },
    pipelines::errors::pipeline_errors::{MissingContextError, RawPartExecutionError},
    utils::user_data::{
        keys::Key,
        user_data::{UserData, UserDataImpl},
    },
};

use super::{
    context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, keys::context_key::DefaultContextKey,
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

type PipelinePartExecutor = Box<dyn Fn(&mut PipelineContext, &UserDataImpl) -> Result<(), PipelinePartExecutionError>>;

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
    fn execute(&self, context: &mut PipelineContext) -> Result<(), PipelinePartExecutionError> {
        (self.executor)(context, &self.config)
    }
}

type PipelinePartFactory = Box<dyn Fn(Box<UserDataImpl>) -> DefaultPipelinePart>;

pub struct PipelineParts {
    names_to_parts: HashMap<String, PipelinePartFactory>,
}

impl PipelineParts {
    pub fn find_part(&self, name: &String) -> Option<&PipelinePartFactory> {
        self.names_to_parts.get(name)
    }
}

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
        ];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            names_to_parts.insert((&part.0).to_owned(), part.1);
        }

        Self { names_to_parts }
    }

    fn read_log_from_xes() -> (String, PipelinePartFactory) {
        const NAME: &str = "ReadLogFromXes";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, _| {
                        let path = Self::get_context_value(context, &context.types().path())?;
                        let log = read_event_log(path);
                        if log.is_none() {
                            let message = format!("Failed to read event log from {}", path.as_str());
                            return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
                        }

                        context.put_concrete(&context.types().event_log().key().clone(), log.unwrap());
                        Ok(())
                    }),
                )
            }),
        )
    }

    fn get_context_value<'a, T>(
        context: &'a PipelineContext,
        key: &DefaultContextKey<T>,
    ) -> Result<&'a T, PipelinePartExecutionError> {
        match context.get_concrete(key.key()) {
            Some(value) => Ok(value),
            None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                key.key().name().to_owned(),
            ))),
        }
    }

    fn write_log_to_xes() -> (String, PipelinePartFactory) {
        const NAME: &str = "WriteLogToXes";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, _| {
                        let path = Self::get_context_value(context, &context.types().path())?;
                        match write_log(&context.get_concrete(&context.types().event_log().key()).unwrap(), path) {
                            Ok(()) => Ok(()),
                            Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(
                                err.to_string(),
                            ))),
                        }
                    }),
                )
            }),
        )
    }

    fn find_primitive_tandem_arrays() -> (String, PipelinePartFactory) {
        const NAME: &str = "FindPrimitiveTandemArrays";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, config| {
                        Self::find_tandem_arrays_and_put_to_context(context, &config, find_primitive_tandem_arrays)
                    }),
                )
            }),
        )
    }

    fn find_maximal_tandem_arrays() -> (String, PipelinePartFactory) {
        const NAME: &str = "FindMaximalTandemArrays";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, config| {
                        Self::find_tandem_arrays_and_put_to_context(context, &config, find_maximal_tandem_arrays)
                    }),
                )
            }),
        )
    }

    fn find_tandem_arrays_and_put_to_context(
        context: &mut PipelineContext,
        part_config: &UserDataImpl,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, usize) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let types = context.types();
        let log = Self::get_context_value(context, &types.event_log())?;
        let array_length = part_config
            .get_concrete(context.types().tandem_array_length().key())
            .unwrap();

        let arrays = patterns_finder(&log.to_hashes_event_log::<NameEventHasher>(), *array_length as usize);
        context.put_concrete(&types.patterns().key().clone(), arrays);
        Ok(())
    }

    fn find_repeats_and_put_to_context(
        context: &mut PipelineContext,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let types = context.types();
        let log = Self::get_context_value(context, &types.event_log())?;
        let strategy = PatternsDiscoveryStrategy::FromAllTraces;
        let arrays = patterns_finder(&log.to_hashes_event_log::<NameEventHasher>(), &strategy);

        context.put_concrete(&types.patterns().key().clone(), arrays);

        Ok(())
    }

    fn find_maximal_repeats() -> (String, PipelinePartFactory) {
        const NAME: &str = "FindMaximalRepeats";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, _| Self::find_repeats_and_put_to_context(context, find_maximal_repeats)),
                )
            }),
        )
    }

    fn find_super_maximal_repeats() -> (String, PipelinePartFactory) {
        const NAME: &str = "FindSuperMaximalRepeats";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, _| Self::find_repeats_and_put_to_context(context, find_super_maximal_repeats)),
                )
            }),
        )
    }

    fn find_near_super_maximal_repeats() -> (String, PipelinePartFactory) {
        const NAME: &str = "FindNearSuperMaximalRepeats";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, _| {
                        Self::find_repeats_and_put_to_context(context, find_near_super_maximal_repeats)
                    }),
                )
            }),
        )
    }

    fn discover_activities() -> (String, PipelinePartFactory) {
        const NAME: &str = "DiscoverActivities";

        (
            NAME.to_string(),
            Box::new(|config| {
                DefaultPipelinePart::new(
                    NAME.to_string(),
                    config,
                    Box::new(|context, _| {
                        let log = Self::get_context_value(context, &context.types().event_log())?;
                        let patterns = Self::get_context_value(context, &context.types().patterns())?;
                        let hashes = log.to_hashes_event_log::<NameEventHasher>();
                        let repeat_sets = build_repeat_sets(&hashes, patterns);

                        let activity_level = Self::get_context_value(context, &context.types().activity_level())?;
                        let tree =
                            build_repeat_set_tree_from_repeats(&hashes, &repeat_sets, *activity_level, |sub_array| {
                                create_activity_name(log, sub_array)
                            });

                        context.put_concrete(&context.types().activities().key().clone(), tree);
                        Ok(())
                    }),
                )
            }),
        )
    }

    fn discover_activities_instances() {}
}

unsafe impl Sync for PipelineParts {}

unsafe impl Send for PipelineParts {}
