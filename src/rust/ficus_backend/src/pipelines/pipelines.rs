use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use chrono::{DateTime, Duration, Utc};
use prost::bytes::BufMut;
use regex::Regex;

use crate::{
    event_log::{
        core::{
            event::{
                event::Event,
                event_hasher::{NameEventHasher, RegexEventHasher},
            },
            event_log::EventLog,
            trace::trace::Trace,
        },
        xes::{
            reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_log,
            xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl,
        },
    },
    features::{
        analysis::{
            event_log_info::{EventLogInfo, EventLogInfoCreationDto},
            patterns::{
                activity_instances::{
                    add_unattached_activities, count_underlying_events, create_activity_name,
                    create_new_log_from_activities_instances, extract_activities_instances, ActivityInTraceInfo,
                    SubTraceKind, UndefActivityHandlingStrategy, UNDEF_ACTIVITY_NAME,
                },
                contexts::PatternsDiscoveryStrategy,
                repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets},
                repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
                tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
            },
        },
        mutations::{
            filtering::{filter_log_by_name, filter_log_by_regex},
            split::get_traces_groups_indices,
        },
    },
    pipelines::errors::pipeline_errors::{MissingContextError, RawPartExecutionError},
    utils::{
        colors::{Color, ColoredRectangle, ColorsHolder},
        user_data::{
            keys::Key,
            user_data::{UserData, UserDataImpl},
        },
    },
};

use super::{
    aliases::TracesActivities,
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
    pub const DRAW_PLACEMENT_OF_EVENT_BY_REGEX: &str = "DrawPlacementOfEventsByRegex";
    pub const DRAW_FULL_ACTIVITIES_DIAGRAM: &str = "DrawFullActivitiesDiagram";
    pub const DRAW_SHORT_ACTIVITIES_DIAGRAM: &str = "DrawShortActivitiesDiagram";
    pub const GET_EVENT_LOG_INFO: &str = "GetEventLogInfo";
    pub const CLEAR_ACTIVITIES: &str = "ClearActivities";
    pub const GET_UNDERLYING_EVENTS_COUNT: &str = "GetUnderlyingEventsCount";
    pub const FILTER_TRACES_BY_EVENTS_COUNT: &str = "FilterTracesByEventsCount";
    pub const TRACES_DIVERSITY_DIAGRAM: &str = "TracesDiversityDiagram";
    pub const GET_NAMES_EVENT_LOG: &str = "GetNamesEventLog";
    pub const GET_HASHES_EVENT_LOG: &str = "GetHashesEventLog";
    pub const USE_NAMES_EVENT_LOG: &str = "UseNamesEventLog";
    pub const DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL: &str = "DiscoverActivitiesForSeveralLevels";
    pub const DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES: &str = "DiscoverActivitiesInUnattachedSubTraces";
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

    fn read_log_from_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_XES, &|context, keys, _| {
            let path = Self::get_context_value(context, keys.path())?;
            context.log(format!("Reading event log from {}", &path))?;

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
        config: &UserDataImpl,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, usize) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_context_value(context, keys.event_log())?;
        let array_length = *config.get_concrete(keys.tandem_array_length().key()).unwrap() as usize;

        let hashed_log = Self::create_hashed_event_log(config, keys, log);

        let arrays = patterns_finder(&hashed_log, array_length);

        context.put_concrete(keys.hashes_event_log().key(), hashed_log);
        context.put_concrete(keys.patterns().key(), arrays);

        Ok(())
    }

    fn find_repeats_and_put_to_context(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_context_value(context, keys.event_log())?;
        let strategy = Self::get_context_value(config, keys.patterns_discovery_strategy())?;

        let hashed_log = Self::create_hashed_event_log(config, keys, log);

        let repeats = patterns_finder(&hashed_log, &strategy);

        context.put_concrete(keys.hashes_event_log().key(), hashed_log);
        context.put_concrete(keys.patterns().key(), repeats);

        Ok(())
    }

    fn create_hashed_event_log(config: &UserDataImpl, keys: &ContextKeys, log: &XesEventLogImpl) -> Vec<Vec<u64>> {
        match Self::get_context_value(config, keys.event_class_regex()) {
            Ok(regex) => {
                let hasher = RegexEventHasher::new(regex).ok().unwrap();
                log.to_hashes_event_log(&hasher)
            }
            Err(_) => log.to_hashes_event_log(&NameEventHasher::new()),
        }
    }

    fn find_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_MAXIMAL_REPEATS, &|context, keys, config| {
            Self::find_repeats_and_put_to_context(context, keys, config, find_maximal_repeats)
        })
    }

    fn find_super_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_SUPER_MAXIMAL_REPEATS, &|context, keys, config| {
            Self::find_repeats_and_put_to_context(context, keys, config, find_super_maximal_repeats)
        })
    }

    fn find_near_super_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_NEAR_SUPER_MAXIMAL_REPEATS, &|context, keys, config| {
            Self::find_repeats_and_put_to_context(context, keys, config, find_near_super_maximal_repeats)
        })
    }

    fn discover_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES, &|context, keys, config| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let patterns = Self::get_context_value(context, keys.patterns())?;
            let hashed_log = Self::get_context_value(context, keys.hashes_event_log())?;
            let repeat_sets = build_repeat_sets(&hashed_log, patterns);

            let activity_level = Self::get_context_value(config, keys.activity_level())?;
            let tree =
                build_repeat_set_tree_from_repeats(&hashed_log, &repeat_sets, *activity_level as usize, |sub_array| {
                    create_activity_name(log, sub_array)
                });

            context.put_concrete(&keys.activities().key(), tree);
            Ok(())
        })
    }

    fn discover_activities_instances() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_INSTANCES, &|context, keys, config| {
            let mut tree = Self::get_context_value_mut(context, keys.activities())?;
            let narrow = Self::get_context_value(config, keys.narrow_activities())?;
            let hashed_log = Self::get_context_value(context, keys.hashes_event_log())?;

            let instances = extract_activities_instances(&hashed_log, &mut tree, *narrow);

            context.put_concrete(&keys.trace_activities().key(), instances);
            Ok(())
        })
    }

    fn create_log_from_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::CREATE_LOG_FROM_ACTIVITIES, &|context, keys, _| {
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
            let event_name = Self::get_context_value(config, keys.event_name())?;
            Self::draw_events_placement(context, keys, &|event| event.get_name() == event_name)
        })
    }

    fn draw_events_placement(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        selector: &impl Fn(&XesEventImpl) -> bool,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_context_value(context, keys.event_log())?;
        let colors_holder =
            Self::get_context_value_mut(context, keys.colors_holder()).expect("Default value should be initialized");

        let mut colors_log = vec![];
        for trace in log.get_traces() {
            let mut colors_trace = vec![];
            let mut index = 0usize;
            for event in trace.borrow().get_events() {
                let event = event.borrow();
                let name = event.get_name();
                if selector(&event) {
                    let color = colors_holder.get_or_create(name.as_str());
                    colors_trace.push(ColoredRectangle::square(color, index, name.to_owned()));
                } else {
                    colors_trace.push(ColoredRectangle::square(
                        Color::black(),
                        index,
                        UNDEF_ACTIVITY_NAME.to_owned(),
                    ));
                }

                index += 1;
            }

            colors_log.push(colors_trace);
        }

        context.put_concrete(keys.colors_event_log().key(), colors_log);
        Ok(())
    }

    fn draw_events_placements_by_regex() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_PLACEMENT_OF_EVENT_BY_REGEX, &|context, keys, config| {
            let regex = Self::get_context_value(config, keys.regex())?;
            let regex = Regex::new(regex).ok().unwrap();
            Self::draw_events_placement(context, keys, &|event| regex.is_match(event.get_name()))
        })
    }

    fn draw_full_activities_diagram() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_FULL_ACTIVITIES_DIAGRAM, &|context, keys, _| {
            let traces_activities = Self::get_context_value(context, keys.trace_activities())?;
            let log = Self::get_context_value(context, keys.event_log())?;
            let colors_holder = Self::get_context_value_mut(context, keys.colors_holder())?;

            let mut colors_log = vec![];
            for (activities, trace) in traces_activities.into_iter().zip(log.get_traces().into_iter()) {
                let mut colors_trace = vec![];

                Self::execute_with_activities_instances(
                    activities,
                    trace.borrow().get_events().len(),
                    &mut |sub_trace| match sub_trace {
                        SubTraceKind::Attached(activity) => {
                            let color = colors_holder.get_or_create(&activity.node.borrow().name);
                            let name = activity.node.borrow().name.to_owned();
                            colors_trace.push(ColoredRectangle::new(color, activity.start_pos, activity.length, name));
                        }
                        SubTraceKind::Unattached(start_pos, length) => {
                            colors_trace.push(ColoredRectangle::new(
                                Color::black(),
                                start_pos,
                                length,
                                UNDEF_ACTIVITY_NAME.to_string(),
                            ));
                        }
                    },
                )?;

                colors_log.push(colors_trace);
            }

            context.put_concrete(keys.colors_event_log().key(), colors_log);

            Ok(())
        })
    }

    fn execute_with_activities_instances(
        activities: &Vec<ActivityInTraceInfo>,
        trace_len: usize,
        handler: &mut impl FnMut(SubTraceKind) -> (),
    ) -> Result<(), PipelinePartExecutionError> {
        let mut index = 0usize;
        for activity in activities {
            if activity.start_pos > index {
                handler(SubTraceKind::Unattached(index, activity.start_pos - index));
            }

            handler(SubTraceKind::Attached(&activity));
            index = activity.start_pos + activity.length;
        }

        if index < trace_len {
            handler(SubTraceKind::Unattached(index, trace_len - index));
        }

        Ok(())
    }

    fn draw_short_activities_diagram() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_SHORT_ACTIVITIES_DIAGRAM, &|context, keys, _| {
            let traces_activities = Self::get_context_value(context, keys.trace_activities())?;
            let log = Self::get_context_value(context, keys.event_log())?;
            let colors_holder = Self::get_context_value_mut(context, keys.colors_holder())?;

            let mut colors_log = vec![];
            for (activities, trace) in traces_activities.into_iter().zip(log.get_traces().into_iter()) {
                let mut colors_trace = vec![];
                let mut index = 0;
                Self::execute_with_activities_instances(
                    activities,
                    trace.borrow().get_events().len(),
                    &mut |sub_trace| {
                        match sub_trace {
                            SubTraceKind::Attached(activity) => {
                                let color = colors_holder.get_or_create(&activity.node.borrow().name);
                                let name = activity.node.borrow().name.to_owned();
                                colors_trace.push(ColoredRectangle::new(color, index, 1, name));
                            }
                            SubTraceKind::Unattached(_, _) => {
                                colors_trace.push(ColoredRectangle::new(
                                    Color::black(),
                                    index,
                                    1,
                                    UNDEF_ACTIVITY_NAME.to_owned(),
                                ));
                            }
                        }

                        index += 1;
                    },
                )?;

                colors_log.push(colors_trace);
            }

            context.put_concrete(keys.colors_event_log().key(), colors_log);

            Ok(())
        })
    }

    fn get_event_log_info() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_EVENT_LOG_INFO, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
            context.put_concrete(keys.event_log_info().key(), log_info);

            Ok(())
        })
    }

    fn clear_activities_related_stuff() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::CLEAR_ACTIVITIES, &|context, keys, _| {
            context.remove_concrete(keys.activities().key());
            context.remove_concrete(keys.trace_activities().key());
            context.remove_concrete(keys.patterns().key());
            context.remove_concrete(keys.repeat_sets().key());

            Ok(())
        })
    }

    fn get_number_of_underlying_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_UNDERLYING_EVENTS_COUNT, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;
            context.put_concrete(keys.underlying_events_count().key(), count_underlying_events(log));
            Ok(())
        })
    }

    fn filter_traces_by_count() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_TRACES_BY_EVENTS_COUNT, &|context, keys, config| {
            let log = Self::get_context_value_mut(context, keys.event_log())?;
            let min_events_count = *Self::get_context_value(config, keys.events_count())? as usize;
            log.filter_traces(&|trace, _| trace.get_events().len() < min_events_count);

            Ok(())
        })
    }

    fn traces_diversity_diagram() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::TRACES_DIVERSITY_DIAGRAM, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let colors_holder = context
                .get_concrete_mut(keys.colors_holder().key())
                .expect("Should be initialized");

            let mut result = vec![];
            for trace in log.get_traces() {
                let mut vec = vec![];
                let mut index = 0usize;
                for event in trace.borrow().get_events() {
                    let event = event.borrow();
                    let name = event.get_name();
                    let color = colors_holder.get_or_create(name.as_str());

                    vec.push(ColoredRectangle::square(color, index, name.to_owned()));
                    index += 1;
                }

                result.push(vec);
            }

            context.put_concrete(keys.colors_event_log().key(), result);

            Ok(())
        })
    }

    fn get_hashes_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_HASHES_EVENT_LOG, &|context, keys, config| {
            let log = Self::get_context_value(context, keys.event_log())?;
            let hashes_event_log = Self::create_hashed_event_log(config, keys, log);

            context.put_concrete(keys.hashes_event_log().key(), hashes_event_log);

            Ok(())
        })
    }

    fn get_names_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_NAMES_EVENT_LOG, &|context, keys, _| {
            let log = Self::get_context_value(context, keys.event_log())?;

            let mut result = vec![];
            for trace in log.get_traces() {
                let mut vec = vec![];
                for event in trace.borrow().get_events() {
                    vec.push(event.borrow().get_name().to_string());
                }

                result.push(vec);
            }

            context.put_concrete(keys.names_event_log().key(), result);

            Ok(())
        })
    }

    fn use_names_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::USE_NAMES_EVENT_LOG, &|context, keys, _| {
            let names_log = Self::get_context_value(context, keys.names_event_log())?;
            let mut log = XesEventLogImpl::empty();
            for names_trace in names_log {
                let mut trace = XesTraceImpl::empty();
                let mut date = DateTime::<Utc>::MIN_UTC;

                for name in names_trace {
                    let event = XesEventImpl::new_with_date(name.clone(), date.clone());
                    trace.push(Rc::new(RefCell::new(event)));
                    date = date + Duration::seconds(1);
                }

                log.push(Rc::new(RefCell::new(trace)));
            }

            context.put_concrete::<XesEventLogImpl>(keys.event_log().key(), log);

            Ok(())
        })
    }

    fn discover_activities_instances_for_several_levels() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL, &|context, keys, config| {
            let event_classes = Self::get_context_value(config, keys.event_classes_regexes())?;

            let name = Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES;

            for event_class_regex in event_classes.into_iter().rev() {
                let add_unattached_events_factory = context.pipeline_parts().unwrap().find_part(name).unwrap();
                let mut new_config = config.clone();
                new_config.put_concrete(keys.event_class_regex().key(), event_class_regex.clone());

                let part = add_unattached_events_factory(Box::new(new_config));

                part.execute(context, keys)?;
            }

            Ok(())
        })
    }

    fn discover_activities_in_unattached_subtraces() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(
            Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES,
            &|context, keys, config| {
                let log = Self::get_context_value(context, keys.event_log())?;
                let mut existing_activities = &Self::create_empty_activities(log);

                if let Ok(activities) = Self::get_context_value(context, keys.trace_activities()) {
                    existing_activities = activities;
                }

                let narrow_activities = *Self::get_context_value(context, keys.narrow_activities())?;
                let hashed_log = Self::create_hashed_event_log(config, keys, log);
                let activities = Self::get_context_value_mut(context, keys.activities())?;
                let min_events_count = *Self::get_context_value(config, keys.events_count())? as usize;

                let new_activities = add_unattached_activities(
                    &hashed_log,
                    activities,
                    existing_activities,
                    min_events_count,
                    narrow_activities,
                );

                context.put_concrete(keys.trace_activities().key(), new_activities);

                Ok(())
            },
        )
    }

    fn create_empty_activities(log: &XesEventLogImpl) -> TracesActivities {
        let mut activities = vec![];
        for _ in log.get_traces() {
            activities.push(vec![]);
        }

        return activities;
    }
}

unsafe impl Sync for PipelineParts {}

unsafe impl Send for PipelineParts {}
