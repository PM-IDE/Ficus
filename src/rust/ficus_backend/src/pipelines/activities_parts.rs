use crate::event_log::core::event::event::Event;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::event_log_info::count_events;
use crate::features::analysis::patterns::activity_instances;
use crate::features::analysis::patterns::activity_instances::{substitute_underlying_events, UNDEF_ACTIVITY_NAME};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::{
    event_log::{
        core::event_log::EventLog,
        xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
    },
    features::analysis::patterns::{
        activity_instances::{
            add_unattached_activities, count_underlying_events, create_activity_name,
            create_log_from_unattached_events, create_new_log_from_activities_instances, extract_activities_instances,
            ActivityInTraceInfo, AdjustingMode, SubTraceKind, UndefActivityHandlingStrategy,
        },
        repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets},
    },
    utils::user_data::user_data::{UserData, UserDataImpl},
};
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

use super::{
    aliases::TracesActivities,
    context::PipelineContext,
    errors::pipeline_errors::PipelinePartExecutionError,
    keys::context_keys::ContextKeys,
    pipelines::{DefaultPipelinePart, PipelinePart, PipelinePartFactory},
};

pub enum UndefActivityHandlingStrategyDto {
    DontInsert,
    InsertAsSingleEvent,
    InsertAllEvents,
}

impl FromStr for UndefActivityHandlingStrategyDto {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DontInsert" => Ok(UndefActivityHandlingStrategyDto::DontInsert),
            "InsertAsSingleEvent" => Ok(UndefActivityHandlingStrategyDto::InsertAsSingleEvent),
            "InsertAllEvents" => Ok(UndefActivityHandlingStrategyDto::InsertAllEvents),
            _ => Err(()),
        }
    }
}

impl PipelineParts {
    pub(super) fn discover_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES, &|context, keys, config| {
            let activity_level = Self::get_user_data(config, keys.activity_level())?;
            Self::do_discover_activities(context, keys, *activity_level)
        })
    }

    pub(super) fn do_discover_activities(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        activity_level: u32,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;
        let patterns = Self::get_user_data(context, keys.patterns())?;
        let hashed_log = Self::get_user_data(context, keys.hashes_event_log())?;
        let repeat_sets = build_repeat_sets(&hashed_log, patterns);

        let tree =
            build_repeat_set_tree_from_repeats(&hashed_log, &repeat_sets, activity_level as usize, |sub_array| {
                create_activity_name(log, sub_array)
            });

        context.put_concrete(&keys.activities().key(), tree);
        Ok(())
    }

    pub(super) fn discover_activities_instances() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_INSTANCES, &|context, keys, config| {
            Self::do_discover_activities_instances(context, keys, config)?;
            Ok(())
        })
    }

    pub(super) fn do_discover_activities_instances(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
    ) -> Result<(), PipelinePartExecutionError> {
        let mut tree = Self::get_user_data_mut(context, keys.activities())?;
        let narrow = Self::get_user_data(config, keys.narrow_activities())?;
        let hashed_log = Self::get_user_data(context, keys.hashes_event_log())?;
        let min_events_in_activity = *Self::get_user_data(config, keys.min_activity_length())?;
        let activity_filter_kind = Self::get_user_data(config, keys.activity_filter_kind())?;

        let instances = extract_activities_instances(
            &hashed_log,
            &mut tree,
            narrow,
            min_events_in_activity as usize,
            activity_filter_kind,
        );

        context.put_concrete(&keys.trace_activities().key(), instances);
        Ok(())
    }

    pub(super) fn create_log_from_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::CREATE_LOG_FROM_ACTIVITIES, &|context, keys, config| {
            Self::do_create_log_from_activities(context, keys, config)?;
            Ok(())
        })
    }

    pub(super) fn do_create_log_from_activities(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;
        let instances = Self::get_user_data(context, keys.trace_activities())?;
        let undef_activity_strat = Self::get_user_data(config, keys.undef_activity_handling_strategy())?;

        let strategy = match undef_activity_strat {
            UndefActivityHandlingStrategyDto::DontInsert => UndefActivityHandlingStrategy::DontInsert,
            UndefActivityHandlingStrategyDto::InsertAsSingleEvent => {
                UndefActivityHandlingStrategy::InsertAsSingleEvent(Box::new(|| {
                    Rc::new(RefCell::new(XesEventImpl::new_min_date(UNDEF_ACTIVITY_NAME.to_owned())))
                }))
            }
            UndefActivityHandlingStrategyDto::InsertAllEvents => UndefActivityHandlingStrategy::InsertAllEvents,
        };

        let log = create_new_log_from_activities_instances(log, instances, &strategy, &|info| {
            Rc::new(RefCell::new(XesEventImpl::new_min_date(
                info.node.borrow().name.clone(),
            )))
        });

        context.put_concrete(keys.event_log().key(), log);
        Ok(())
    }

    pub(super) fn discover_activities_instances_for_several_levels() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL, &|context, keys, config| {
            let event_classes = Self::get_user_data(config, keys.event_classes_regexes())?;
            let initial_activity_level = *Self::get_user_data(config, keys.activity_level())?;
            let patterns_kind = Self::get_user_data(config, keys.patterns_kind())?;
            let adjusting_mode = Self::get_user_data(config, keys.adjusting_mode())?;
            let patterns_discovery_strategy = Self::get_user_data(config, keys.patterns_discovery_strategy())?;
            let narrow_activities = Self::get_user_data(config, keys.narrow_activities())?;
            let events_count = Self::get_user_data(config, keys.events_count())?;
            let min_events_in_activity = Self::get_user_data(config, keys.min_activity_length())?;
            let activity_filter_kind = Self::get_user_data(config, keys.activity_filter_kind())?;

            let mut index = 0;
            for event_class_regex in event_classes.into_iter().rev() {
                let mut config = UserDataImpl::new();
                config.put_concrete(keys.patterns_kind().key(), *patterns_kind);
                config.put_concrete(keys.event_class_regex().key(), event_class_regex.to_owned());
                config.put_concrete(keys.adjusting_mode().key(), *adjusting_mode);
                config.put_concrete(keys.activity_level().key(), initial_activity_level + index);
                config.put_concrete(keys.patterns_discovery_strategy().key(), *patterns_discovery_strategy);
                config.put_concrete(keys.narrow_activities().key(), *narrow_activities);
                config.put_concrete(keys.events_count().key(), *events_count);
                config.put_concrete(keys.min_activity_length().key(), *min_events_in_activity);
                config.put_concrete(keys.activity_filter_kind().key(), *activity_filter_kind);

                Self::adjust_with_activities_from_unattached_events(context, keys, &config)?;

                index += 1;
            }

            Ok(())
        })
    }

    pub(super) fn adjust_with_activities_from_unattached_events(
        old_context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
    ) -> Result<(), PipelinePartExecutionError> {
        if Self::get_user_data(old_context, keys.activities()).is_err() {
            old_context.put_concrete(keys.activities().key(), vec![])
        }

        let adjusting_mode = *Self::get_user_data(config, keys.adjusting_mode())?;
        let log = Self::get_user_data(old_context, keys.event_log())?;

        let mut new_context = PipelineContext::empty_from(&old_context);

        if adjusting_mode == AdjustingMode::FromUnattachedSubTraces {
            match Self::get_user_data(old_context, keys.trace_activities()) {
                Ok(activities) => new_context.put_concrete(
                    keys.event_log().key(),
                    create_log_from_unattached_events(log, activities),
                ),
                Err(_) => {}
            }
        } else {
            new_context.put_concrete(keys.event_log().key(), log.clone());
        }

        Self::find_patterns(&mut new_context, keys, config)?;

        let old_activities = Self::get_user_data_mut(old_context, keys.activities())?;
        let new_activities = Self::get_user_data(&new_context, keys.activities())?;
        for new_activity in new_activities {
            old_activities.push(new_activity.clone());
        }

        old_context
            .pipeline_parts()
            .unwrap()
            .create_add_unattached_events_part(config.clone())
            .execute(old_context, keys)?;

        Ok(())
    }

    pub(super) fn discover_activities_in_unattached_subtraces() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(
            Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES,
            &|context, keys, config| {
                let log = Self::get_user_data(context, keys.event_log())?;
                let mut existing_activities = &Self::create_empty_activities(log);

                if let Ok(activities) = Self::get_user_data(context, keys.trace_activities()) {
                    existing_activities = activities;
                }

                let activities = Self::get_user_data_mut(context, keys.activities())?;

                let narrow_kind = Self::get_user_data(config, keys.narrow_activities())?;
                let hashed_log = Self::create_hashed_event_log(config, keys, log);
                let min_events_count = *Self::get_user_data(config, keys.events_count())? as usize;
                let min_events_in_activity = *Self::get_user_data(config, keys.min_activity_length())? as usize;
                let activity_filter_kind = Self::get_user_data(config, keys.activity_filter_kind())?;

                let new_activities = add_unattached_activities(
                    &hashed_log,
                    activities,
                    existing_activities,
                    min_events_count,
                    narrow_kind,
                    min_events_in_activity,
                    activity_filter_kind,
                );

                context.put_concrete(keys.trace_activities().key(), new_activities);

                Ok(())
            },
        )
    }

    pub(super) fn create_add_unattached_events_part(&self, config: UserDataImpl) -> DefaultPipelinePart {
        let name = Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES;
        let add_unattached_events_factory = self.find_part(name).unwrap();

        add_unattached_events_factory(Box::new(config))
    }

    pub(super) fn create_empty_activities(log: &XesEventLogImpl) -> TracesActivities {
        let mut activities = vec![];
        for _ in log.traces() {
            activities.push(vec![]);
        }

        return activities;
    }

    pub(super) fn clear_activities_related_stuff() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::CLEAR_ACTIVITIES, &|context, keys, _| {
            Self::do_clear_activities_related_stuff(context, keys);
            Ok(())
        })
    }

    pub(super) fn do_clear_activities_related_stuff(context: &mut PipelineContext, keys: &ContextKeys) {
        context.remove_concrete(keys.activities().key());
        context.remove_concrete(keys.trace_activities().key());
        context.remove_concrete(keys.patterns().key());
        context.remove_concrete(keys.repeat_sets().key());
    }

    pub(super) fn get_number_of_underlying_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_UNDERLYING_EVENTS_COUNT, &|context, keys, _| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let count = count_underlying_events(log);
            context.log(format!("Number of underlying events: {}", &count))?;

            context.put_concrete(keys.underlying_events_count().key(), count);
            Ok(())
        })
    }

    pub(super) fn execute_with_activities_instances(
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

    pub(super) fn discover_activities_until_no_more() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_UNTIL_NO_MORE, &|context, keys, config| {
            let mut activity_level = *Self::get_user_data(config, keys.activity_level())?;

            loop {
                let log = Self::get_user_data(context, keys.event_log())?;
                let events_count = count_events(log);

                Self::do_clear_activities_related_stuff(context, keys);
                Self::find_patterns(context, keys, config)?;
                Self::do_discover_activities(context, keys, activity_level)?;
                Self::do_discover_activities_instances(context, keys, config)?;

                let activities_instances = Self::get_user_data(context, keys.trace_activities())?;
                context.log(format!(
                    "Discovered {} activities instances",
                    activities_instances.iter().map(|t| t.len()).sum::<usize>()
                ))?;

                Self::do_create_log_from_activities(context, keys, config)?;

                let new_events_count = count_events(Self::get_user_data(context, keys.event_log())?);
                if new_events_count == events_count {
                    return Ok(());
                }

                activity_level += 1;
            }
        })
    }

    pub(super) fn execute_with_each_activity_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::EXECUTE_WITH_EACH_ACTIVITY_LOG, &|context, keys, config| {
            let activity_level = *Self::get_user_data(config, keys.activity_level())?;
            let log = Self::get_user_data(context, keys.event_log())?;
            let activities = Self::get_user_data(context, keys.trace_activities())?;
            let pipeline = Self::get_user_data(config, keys.pipeline())?;

            let activities_to_logs =
                activity_instances::create_logs_for_activities(log, activities, activity_level as usize);

            for (_, activity_log) in activities_to_logs {
                let mut temp_context = PipelineContext::empty_from(context);
                temp_context.put_concrete(keys.event_log().key(), activity_log.borrow().clone());

                pipeline.execute(&mut temp_context, keys)?;
            }

            Ok(())
        })
    }

    pub(super) fn substitute_underlying_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::SUBSTITUTE_UNDERLYING_EVENTS, &|context, keys, _| {
            let log = Self::get_user_data_mut(context, keys.event_log())?;
            let mut new_log = XesEventLogImpl::empty();

            for trace in log.traces() {
                let mut new_trace = XesTraceImpl::empty();
                for event in trace.borrow().events() {
                    substitute_underlying_events::<XesEventLogImpl>(event, &mut new_trace);
                }

                new_log.push(Rc::new(RefCell::new(new_trace)));
            }

            context.put_concrete(keys.event_log().key(), new_log);
            Ok(())
        })
    }

    pub(super) fn apply_class_extractor() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::APPLY_CLASS_EXTRACTOR, &|context, keys, config| {
            let log = Self::get_user_data_mut(context, keys.event_log())?;

            let event_class_regex = Self::get_user_data(config, keys.event_class_regex())?;
            let event_class_regex = Self::try_parse_regex(event_class_regex)?;

            let filter_regex = Self::get_user_data(config, keys.regex())?;
            let filter_regex = Self::try_parse_regex(filter_regex)?;

            for trace in log.traces() {
                for event in trace.borrow().events() {
                    if !filter_regex.is_match(event.borrow().name()) {
                        continue;
                    }

                    let borrowed_event = event.borrow();
                    let found_match = event_class_regex.find(borrowed_event.name());
                    if found_match.is_none() {
                        continue;
                    }

                    let found_match = found_match.unwrap();
                    let start = found_match.start();
                    let end = found_match.end();
                    drop(found_match);
                    drop(borrowed_event);

                    if start == 0 {
                        let new_name = event.borrow().name()[start..end].to_owned();
                        event.borrow_mut().set_name(new_name);
                    }
                }
            }

            Ok(())
        })
    }
}
