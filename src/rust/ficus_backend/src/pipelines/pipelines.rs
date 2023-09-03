use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use regex::Regex;

use crate::{
    event_log::{
        core::{
            event::{event::Event, event_hasher::NameEventHasher},
            event_log::EventLog,
            trace::trace::Trace,
        },
        xes::{
            reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_log,
            xes_event::XesEventImpl,
        },
    },
    features::{
        analysis::patterns::{
            activity_instances::{
                create_activity_name, create_new_log_from_activities_instances, extract_activities_instances,
                UndefActivityHandlingStrategy,
            },
            contexts::PatternsDiscoveryStrategy,
            repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets},
            repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
            tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
        },
        mutations::{
            filtering::{filter_log_by_name, filter_log_by_regex},
            split::get_traces_groups_indices,
        },
    },
    pipelines::errors::pipeline_errors::{MissingContextError, RawPartExecutionError},
    utils::{
        colors::Color,
        user_data::{
            keys::Key,
            user_data::{UserData, UserDataImpl},
        },
    },
};

use super::{
    aliases::NamesToColors,
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
        for part in &self.parts {
            part.execute(context, keys)?;
        }

        Ok(())
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

type PipelinePartFactory = Box<dyn Fn(Box<UserDataImpl>) -> DefaultPipelinePart>;

pub struct PipelineParts {
    names_to_parts: HashMap<String, PipelinePartFactory>,
}

impl PipelineParts {
    pub fn find_part(&self, name: &str) -> Option<&PipelinePartFactory> {
        self.names_to_parts.get(name)
    }
}

impl PipelineParts {
    pub const READ_LOG_FROM_XES: &str = "ReadLogFromXes";
    pub const WRITE_LOG_TO_XES: &str = "WriteLogToXes";
    pub const FIND_PRIMITIVE_TANDEM_ARRAYS: &str = "FindPrimitiveTandemArrays";
    pub const FIND_MAXIMAL_TANDEM_ARRAYS: &str = "FindMaximalTandemArrays";
    pub const FIND_MAXIMAL_REPEATS: &str = "FindMaximalRepeats";
    pub const FIND_SUPER_MAXIMAL_REPEATS: &str = "FindSuperMaximalRepeats";
    pub const FIND_NEAR_SUPER_MAXIMAL_REPEATS: &str = "FindNearSuperMaximalRepeats";
    pub const DISCOVER_ACTIVITIES: &str = "DiscoverActivities";
    pub const DISCOVER_ACTIVITIES_INSTANCES: &str = "DiscoverActivitiesInstances";
    pub const CREATE_LOG_FROM_ACTIVITIES: &str = "CreateLogFromActivities";
    pub const FILTER_EVENTS_BY_NAME: &str = "FilterEventsByName";
    pub const FILTER_EVENTS_BY_REGEX: &str = "FilterEventsByRegex";
    pub const FILTER_LOG_BY_VARIANTS: &str = "FilterLogByVariants";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_NAME: &str = "DrawPlacementOfEventByName";
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
            Self::discover_activities(),
            Self::discover_activities_instances(),
            Self::create_log_from_activities(),
            Self::filter_log_by_event_name(),
            Self::filter_log_by_regex(),
            Self::filter_log_by_variants(),
            Self::draw_placements_of_event_by_name(),
        ];

        let mut names_to_parts = HashMap::new();
        for part in parts {
            names_to_parts.insert((&part.0).to_owned(), part.1);
        }

        Self { names_to_parts }
    }

    fn read_log_from_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_XES, &|context, keys, _| {
            let path = Self::get_context_value(context, keys.path())?;
            let log = read_event_log(path);
            if log.is_none() {
                let message = format!("Failed to read event log from {}", path.as_str());
                return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
            }

            context.put_concrete(keys.event_log().key(), log.unwrap());
            Ok(())
        })
    }

    fn create_pipeline_part(
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

    fn get_context_value<'a, T>(
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

    fn get_context_value_mut<'a, T>(
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

    fn write_log_to_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_LOG_TO_XES, &|context, keys, _| {
            let path = Self::get_context_value(context, &keys.path())?;
            match write_log(&context.get_concrete(&keys.event_log().key()).unwrap(), path) {
                Ok(()) => Ok(()),
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(
                    err.to_string(),
                ))),
            }
        })
    }

    fn find_primitive_tandem_arrays() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_PRIMITIVE_TANDEM_ARRAYS, &|context, keys, config| {
            Self::find_tandem_arrays_and_put_to_context(context, keys, &config, find_primitive_tandem_arrays)
        })
    }

    fn find_maximal_tandem_arrays() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_MAXIMAL_TANDEM_ARRAYS, &|context, keys, config| {
            Self::find_tandem_arrays_and_put_to_context(context, keys, &config, find_maximal_tandem_arrays)
        })
    }

    fn find_tandem_arrays_and_put_to_context(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        part_config: &UserDataImpl,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, usize) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_context_value(context, keys.event_log())?;
        let array_length = part_config.get_concrete(keys.tandem_array_length().key()).unwrap();

        let arrays = patterns_finder(&log.to_hashes_event_log::<NameEventHasher>(), *array_length as usize);
        context.put_concrete(keys.patterns().key(), arrays);
        Ok(())
    }

    fn find_repeats_and_put_to_context(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_context_value(context, keys.event_log())?;
        let strategy = PatternsDiscoveryStrategy::FromAllTraces;
        let arrays = patterns_finder(&log.to_hashes_event_log::<NameEventHasher>(), &strategy);

        context.put_concrete(keys.patterns().key(), arrays);

        Ok(())
    }

    fn find_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_MAXIMAL_REPEATS, &|context, keys, _| {
            Self::find_repeats_and_put_to_context(context, keys, find_maximal_repeats)
        })
    }

    fn find_super_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_SUPER_MAXIMAL_REPEATS, &|context, keys, _| {
            Self::find_repeats_and_put_to_context(context, keys, find_super_maximal_repeats)
        })
    }

    fn find_near_super_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_NEAR_SUPER_MAXIMAL_REPEATS, &|context, keys, _| {
            Self::find_repeats_and_put_to_context(context, keys, find_near_super_maximal_repeats)
        })
    }

    fn discover_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES, &|context, keys, config| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let patterns = Self::get_context_value(context, keys.patterns())?;
            let hashes = log.to_hashes_event_log::<NameEventHasher>();
            let repeat_sets = build_repeat_sets(&hashes, patterns);

            let activity_level = Self::get_context_value(config, keys.activity_level())?;
            let tree =
                build_repeat_set_tree_from_repeats(&hashes, &repeat_sets, *activity_level as usize, |sub_array| {
                    create_activity_name(log, sub_array)
                });

            context.put_concrete(&keys.activities().key(), tree);
            Ok(())
        })
    }

    fn discover_activities_instances() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_INSTANCES, &|context, keys, config| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let mut tree = Self::get_context_value_mut(context, keys.activities())?;
            let narrow = Self::get_context_value(config, keys.narrow_activities())?;

            let hashes = log.to_hashes_event_log::<NameEventHasher>();
            let instances = extract_activities_instances(&hashes, &mut tree, *narrow);

            context.put_concrete(&keys.trace_activities().key(), instances);
            Ok(())
        })
    }

    fn create_log_from_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::CREATE_LOG_FROM_ACTIVITIES, &|context, keys, config| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let instances = Self::get_context_value(context, keys.trace_activities())?;
            let log = create_new_log_from_activities_instances(
                log,
                instances,
                &UndefActivityHandlingStrategy::InsertAllEvents,
                &|info| {
                    Rc::new(RefCell::new(XesEventImpl::new_min_date(
                        info.node.borrow().name.clone(),
                    )))
                },
            );

            context.put_concrete(keys.event_log().key(), log);

            Ok(())
        })
    }

    fn filter_log_by_event_name() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_EVENTS_BY_NAME, &|context, keys, config| {
            let log = Self::get_context_value_mut(context, keys.event_log())?;
            let event_name = Self::get_context_value(config, keys.event_name())?;
            filter_log_by_name(log, &event_name);

            Ok(())
        })
    }

    fn filter_log_by_regex() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_EVENTS_BY_REGEX, &|context, keys, config| {
            let log = Self::get_context_value_mut(context, keys.event_log())?;
            let regex = Self::get_context_value(config, keys.regex())?;

            match Regex::new(&regex) {
                Ok(regex) => {
                    filter_log_by_regex(log, &regex);
                    Ok(())
                }
                Err(_) => {
                    let error = format!("Failed to parse regex {}", regex);
                    Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(error)))
                }
            }
        })
    }

    fn filter_log_by_variants() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_LOG_BY_VARIANTS, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let groups_indices: HashSet<usize> = get_traces_groups_indices(log)
                .into_iter()
                .map(|group| *(group.first().unwrap()))
                .collect();

            let log = Self::get_context_value_mut(context, keys.event_log())?;
            log.filter_traces(&|_, index| groups_indices.contains(&index));

            Ok(())
        })
    }

    fn draw_placements_of_event_by_name() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_PLACEMENT_OF_EVENT_BY_NAME, &|context, keys, config| {
            let names_to_colors_key = keys.names_to_colors().key();
            if context.get_concrete(names_to_colors_key).is_none() {
                context.put_concrete(names_to_colors_key, HashMap::new())
            }

            let log = Self::get_context_value(context, keys.event_log())?;
            let event_name = Self::get_context_value(config, keys.event_name())?;

            let names_to_colors = Self::get_context_value_mut(context, keys.names_to_colors())
                .ok()
                .unwrap();
            let used_colors = names_to_colors
                .values()
                .into_iter()
                .map(|c| *c)
                .collect::<HashSet<Color>>();

            let mut colors_log = vec![];
            for trace in log.get_traces() {
                let mut colors_trace = vec![];
                for event in trace.borrow().get_events() {
                    let event = event.borrow();
                    let name = event.get_name();
                    if name == event_name {
                        if !names_to_colors.contains_key(name) {
                            let new_color = Color::random(Some(&used_colors));
                            names_to_colors.insert(name.to_owned(), new_color);
                        }

                        colors_trace.push(names_to_colors[name].clone());
                    } else {
                        colors_trace.push(Color::black());
                    }
                }

                colors_log.push(colors_trace);
            }

            context.put_concrete(keys.colors_event_log().key(), colors_log);

            Ok(())
        })
    }
}

unsafe impl Sync for PipelineParts {}

unsafe impl Send for PipelineParts {}
