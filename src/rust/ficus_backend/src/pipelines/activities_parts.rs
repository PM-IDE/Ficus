use std::{cell::RefCell, rc::Rc};

use crate::features::analysis::event_log_info::count_events;
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

use super::{
    aliases::TracesActivities,
    context::PipelineContext,
    errors::pipeline_errors::PipelinePartExecutionError,
    keys::context_keys::ContextKeys,
    pipelines::{DefaultPipelinePart, PipelinePart, PipelinePartFactory, PipelineParts},
};

impl PipelineParts {
    pub(super) fn discover_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES, &|context, keys, config| {
            let activity_level = Self::get_context_value(config, keys.activity_level())?;
            Self::do_discover_activities(context, keys, *activity_level)
        })
    }

    pub(super) fn do_discover_activities(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        activity_level: u32,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_context_value(context, keys.event_log())?;
        let patterns = Self::get_context_value(context, keys.patterns())?;
        let hashed_log = Self::get_context_value(context, keys.hashes_event_log())?;
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
        let mut tree = Self::get_context_value_mut(context, keys.activities())?;
        let narrow = Self::get_context_value(config, keys.narrow_activities())?;
        let hashed_log = Self::get_context_value(context, keys.hashes_event_log())?;

        let instances = extract_activities_instances(&hashed_log, &mut tree, *narrow);

        context.put_concrete(&keys.trace_activities().key(), instances);
        Ok(())
    }

    pub(super) fn create_log_from_activities() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::CREATE_LOG_FROM_ACTIVITIES, &|context, keys, _| {
            Self::do_create_log_from_activities(context, keys)?;
            Ok(())
        })
    }

    pub(super) fn do_create_log_from_activities(
        context: &mut PipelineContext,
        keys: &ContextKeys,
    ) -> Result<(), PipelinePartExecutionError> {
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
    }

    pub(super) fn discover_activities_instances_for_several_levels() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL, &|context, keys, config| {
            let event_classes = Self::get_context_value(config, keys.event_classes_regexes())?;
            let initial_activity_level = *Self::get_context_value(config, keys.activity_level())?;
            let patterns_kind = Self::get_context_value(config, keys.patterns_kind())?;
            let adjusting_mode = Self::get_context_value(config, keys.adjusting_mode())?;
            let patterns_discovery_strategy = Self::get_context_value(config, keys.patterns_discovery_strategy())?;
            let narrow_activities = Self::get_context_value(config, keys.narrow_activities())?;
            let events_count = Self::get_context_value(config, keys.events_count())?;

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
        if Self::get_context_value(old_context, keys.activities()).is_err() {
            old_context.put_concrete(keys.activities().key(), vec![])
        }

        let adjusting_mode = *Self::get_context_value(config, keys.adjusting_mode())?;
        let log = Self::get_context_value(old_context, keys.event_log())?;

        let mut new_context = PipelineContext::empty_from(&old_context);

        if adjusting_mode == AdjustingMode::FromUnattachedSubTraces {
            match Self::get_context_value(old_context, keys.trace_activities()) {
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

        let old_activities = Self::get_context_value_mut(old_context, keys.activities())?;
        let new_activities = Self::get_context_value(&new_context, keys.activities())?;
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
                let log = Self::get_context_value(context, keys.event_log())?;
                let mut existing_activities = &Self::create_empty_activities(log);

                if let Ok(activities) = Self::get_context_value(context, keys.trace_activities()) {
                    existing_activities = activities;
                }

                let activities = Self::get_context_value_mut(context, keys.activities())?;

                let narrow_activities = *Self::get_context_value(config, keys.narrow_activities())?;
                let hashed_log = Self::create_hashed_event_log(config, keys, log);
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

    pub(super) fn discover_activities_from_pattern_source() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_FROM_PATTERNS, &|context, keys, config| {
            let pipeline = Self::get_context_value(config, keys.pipeline())?;
            pipeline.execute(context, keys)
        })
    }

    pub(super) fn create_add_unattached_events_part(&self, config: UserDataImpl) -> DefaultPipelinePart {
        let name = Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES;
        let add_unattached_events_factory = self.find_part(name).unwrap();

        add_unattached_events_factory(Box::new(config))
    }

    pub(super) fn create_empty_activities(log: &XesEventLogImpl) -> TracesActivities {
        let mut activities = vec![];
        for _ in log.get_traces() {
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
            let log = Self::get_context_value(context, keys.event_log())?;
            context.put_concrete(keys.underlying_events_count().key(), count_underlying_events(log));
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
            let mut activity_level = *Self::get_context_value(config, keys.activity_level())?;

            loop {
                let log = Self::get_context_value(context, keys.event_log())?;
                let events_count = count_events(log);

                Self::do_clear_activities_related_stuff(context, keys);
                Self::find_patterns(context, keys, config)?;
                Self::do_discover_activities(context, keys, activity_level)?;
                Self::do_discover_activities_instances(context, keys, config)?;
                Self::do_create_log_from_activities(context, keys)?;

                let new_events_count = count_events(Self::get_context_value(context, keys.event_log())?);
                if new_events_count == events_count {
                    return Ok(());
                }

                activity_level += 1;
            }
        })
    }
}