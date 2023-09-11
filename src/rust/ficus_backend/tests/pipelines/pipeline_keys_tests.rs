use std::sync::Arc;

use ficus_backend::{
    event_log::{core::event_log::EventLog, xes::xes_event_log::XesEventLogImpl},
    features::{
        analysis::patterns::{
            activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy, entry_points::PatternsKind,
        },
        discovery::petri_net::PetriNet,
    },
    pipelines::{
        aliases::{Activities, ActivitiesToLogs, ColorsEventLog, Patterns, RepeatSets, TracesActivities},
        context::PipelineContext,
        keys::context_keys::ContextKeys,
        pipelines::Pipeline,
    },
    utils::{
        colors::ColorsHolder,
        user_data::{keys::Key, user_data::UserData},
    },
    vecs,
};

#[test]
fn test_event_log_key() {
    execute_test(|keys, context| {
        let log_key = keys.event_log();
        let log = XesEventLogImpl::empty();

        assert!(context.get_concrete(log_key.key()).is_none());

        context.put_concrete(log_key.key(), log);

        assert!(context.get_concrete(log_key.key()).is_some())
    })
}

fn execute_test(test: impl Fn(&ContextKeys, &mut PipelineContext) -> ()) {
    let keys = Arc::new(Box::new(ContextKeys::new()));
    let mut context = PipelineContext::empty();

    test(&keys, &mut context);
}

#[test]
#[rustfmt::skip]
fn test_event_log_all_concrete_keys() {
    execute_test(|keys, _| {
        assert!(keys.find_concrete_key::<XesEventLogImpl>(ContextKeys::EVENT_LOG).is_some());
        assert!(keys.find_concrete_key::<Activities>(ContextKeys::ACTIVITIES).is_some());
        assert!(keys.find_concrete_key::<ActivitiesToLogs>(ContextKeys::ACTIVITIES_TO_LOGS).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::ACTIVITY_NAME).is_some());
        assert!(keys.find_concrete_key::<Patterns>(ContextKeys::PATTERNS).is_some());
        assert!(keys.find_concrete_key::<Vec<Vec<u64>>>(ContextKeys::HASHES_EVENT_LOG).is_some());
        assert!(keys.find_concrete_key::<Vec<Vec<String>>>(ContextKeys::NAMES_EVENT_LOG).is_some());
        assert!(keys.find_concrete_key::<PetriNet>(ContextKeys::PETRI_NET).is_some());
        assert!(keys.find_concrete_key::<RepeatSets>(ContextKeys::REPEAT_SETS).is_some());
        assert!(keys.find_concrete_key::<TracesActivities>(ContextKeys::TRACE_ACTIVITIES).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::PATH).is_some());
        assert!(keys.find_concrete_key::<ColorsEventLog>(ContextKeys::COLORS_EVENT_LOG).is_some());
        assert!(keys.find_concrete_key::<ColorsHolder>(ContextKeys::COLORS_HOLDER).is_some());
        assert!(keys.find_concrete_key::<usize>(ContextKeys::UNDERLYING_EVENTS_COUNT).is_some());
        assert!(keys.find_concrete_key::<u32>(ContextKeys::EVENTS_COUNT).is_some());
        assert!(keys.find_concrete_key::<Vec<String>>(ContextKeys::EVENT_CLASSES_REGEXES).is_some());
        assert!(keys.find_concrete_key::<AdjustingMode>(ContextKeys::ADJUSTING_MODE).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::EVENT_CLASS_REGEX).is_some());
        assert!(keys.find_concrete_key::<PatternsKind>(ContextKeys::PATTERNS_KIND).is_some());
        assert!(keys.find_concrete_key::<Pipeline>(ContextKeys::PIPELINE).is_some());

        assert!(keys.find_concrete_key::<u32>(ContextKeys::TANDEM_ARRAY_LENGTH).is_some());
        assert!(keys.find_concrete_key::<u32>(ContextKeys::ACTIVITY_LEVEL).is_some());
        assert!(keys.find_concrete_key::<bool>(ContextKeys::NARROW_ACTIVITIES).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::EVENT_NAME).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::REGEX).is_some());
        assert!(keys.find_concrete_key::<PatternsDiscoveryStrategy>(ContextKeys::PATTERNS_DISCOVERY_STRATEGY).is_some());
    })
}

#[rustfmt::skip]
fn get_all_keys_names() -> Vec<String> {
    vecs![
        "path",
        "tandem_array_length",
        "activity_level",
        "narrow_activities",
        "event_name",
        "regex",
        "patterns_discovery_strategy",
        "output_string",
        "event_log_info",
        "underlying_events_count",
        "events_count",
        "event_classes_regexes",
        "adjusting_mode",
        "event_class_regex",
        "patterns_kind",
        "pipeline",

        "event_log",
        "activities",
        "repeat_sets",
        "trace_activities",
        "patterns",
        "petri_net",
        "activities_to_logs",
        "activity_name",
        "hashes_event_log",
        "names_event_log",
        "colors_event_log",
        "colors_holder"
    ]
}

#[test]
fn test_event_log_all_keys() {
    execute_test(|keys, _| {
        for key_name in get_all_keys_names() {
            assert!(keys.find_key(&key_name).is_some());
        }
    })
}

#[test]
fn test_keys_count() {
    execute_test(|keys, _| assert_eq!(keys.len(), get_all_keys_names().len()))
}

#[test]
#[rustfmt::skip]
fn test_equivalence_of_keys() {
    execute_test(|keys, _| {
        assert!(keys.find_key(ContextKeys::EVENT_LOG).unwrap().key().id() == keys.find_concrete_key::<XesEventLogImpl>(ContextKeys::EVENT_LOG).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::ACTIVITIES).unwrap().key().id() == keys.find_concrete_key::<Activities>(ContextKeys::ACTIVITIES).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::ACTIVITIES_TO_LOGS).unwrap().key().id() == keys.find_concrete_key::<ActivitiesToLogs>(ContextKeys::ACTIVITIES_TO_LOGS).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::ACTIVITY_NAME).unwrap().key().id() == keys.find_concrete_key::<String>(ContextKeys::ACTIVITY_NAME).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::PATTERNS).unwrap().key().id() == keys.find_concrete_key::<Patterns>(ContextKeys::PATTERNS).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::HASHES_EVENT_LOG).unwrap().key().id() == keys.find_concrete_key::<Vec<Vec<u64>>>(ContextKeys::HASHES_EVENT_LOG).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::NAMES_EVENT_LOG).unwrap().key().id() == keys.find_concrete_key::<Vec<Vec<String>>>(ContextKeys::NAMES_EVENT_LOG).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::PETRI_NET).unwrap().key().id() == keys.find_concrete_key::<PetriNet>(ContextKeys::PETRI_NET).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::REPEAT_SETS).unwrap().key().id() == keys.find_concrete_key::<RepeatSets>(ContextKeys::REPEAT_SETS).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::TRACE_ACTIVITIES).unwrap().key().id() == keys.find_concrete_key::<TracesActivities>(ContextKeys::TRACE_ACTIVITIES).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::PATH).unwrap().key().id() == keys.find_concrete_key::<String>(ContextKeys::PATH).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::TANDEM_ARRAY_LENGTH).unwrap().key().id() == keys.find_concrete_key::<u32>(ContextKeys::TANDEM_ARRAY_LENGTH).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::ACTIVITY_LEVEL).unwrap().key().id() == keys.find_concrete_key::<u32>(ContextKeys::ACTIVITY_LEVEL).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::NARROW_ACTIVITIES).unwrap().key().id() == keys.find_concrete_key::<bool>(ContextKeys::NARROW_ACTIVITIES).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::EVENT_NAME).unwrap().key().id() == keys.find_concrete_key::<String>(ContextKeys::EVENT_NAME).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::REGEX).unwrap().key().id() == keys.find_concrete_key::<String>(ContextKeys::REGEX).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::COLORS_EVENT_LOG).unwrap().key().id() == keys.find_concrete_key::<ColorsEventLog>(ContextKeys::COLORS_EVENT_LOG).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::COLORS_HOLDER).unwrap().key().id() == keys.find_concrete_key::<ColorsHolder>(ContextKeys::COLORS_HOLDER).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::PATTERNS_DISCOVERY_STRATEGY).unwrap().key().id() == keys.find_concrete_key::<PatternsDiscoveryStrategy>(ContextKeys::PATTERNS_DISCOVERY_STRATEGY).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::UNDERLYING_EVENTS_COUNT).unwrap().key().id() == keys.find_concrete_key::<usize>(ContextKeys::UNDERLYING_EVENTS_COUNT).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::EVENTS_COUNT).unwrap().key().id() == keys.find_concrete_key::<u32>(ContextKeys::EVENTS_COUNT).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::EVENT_CLASSES_REGEXES).unwrap().key().id() == keys.find_concrete_key::<Vec<String>>(ContextKeys::EVENT_CLASSES_REGEXES).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::ADJUSTING_MODE).unwrap().key().id() == keys.find_concrete_key::<AdjustingMode>(ContextKeys::ADJUSTING_MODE).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::EVENT_CLASS_REGEX).unwrap().key().id() == keys.find_concrete_key::<String>(ContextKeys::EVENT_CLASS_REGEX).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::PATTERNS_KIND).unwrap().key().id() == keys.find_concrete_key::<PatternsKind>(ContextKeys::PATTERNS_KIND).unwrap().key().id());
        assert!(keys.find_key(ContextKeys::PIPELINE).unwrap().key().id() == keys.find_concrete_key::<Pipeline>(ContextKeys::PIPELINE).unwrap().key().id());
    })
}
