use std::sync::Arc;

use ficus_backend::{
    event_log::{core::event_log::EventLog, xes::xes_event_log::XesEventLogImpl},
    features::{
        analysis::{
            event_log_info::EventLogInfo,
            patterns::{
                activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy, entry_points::PatternsKind,
            },
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
        let mut count = 0usize;

        assert_existence::<String>(keys, ContextKeys::PATH, &mut count);
        assert_existence::<u32>(keys, ContextKeys::TANDEM_ARRAY_LENGTH, &mut count);
        assert_existence::<u32>(keys, ContextKeys::ACTIVITY_LEVEL, &mut count);
        assert_existence::<bool>(keys, ContextKeys::NARROW_ACTIVITIES, &mut count);
        assert_existence::<String>(keys, ContextKeys::EVENT_NAME, &mut count);
        assert_existence::<String>(keys, ContextKeys::REGEX, &mut count);
        assert_existence::<PatternsDiscoveryStrategy>(keys, ContextKeys::PATTERNS_DISCOVERY_STRATEGY, &mut count);
        assert_existence::<String>(keys, ContextKeys::OUTPUT_STRING, &mut count);
        assert_existence::<EventLogInfo>(keys, ContextKeys::EVENT_LOG_INFO, &mut count);
        assert_existence::<usize>(keys, ContextKeys::UNDERLYING_EVENTS_COUNT, &mut count);
        assert_existence::<u32>(keys, ContextKeys::EVENTS_COUNT, &mut count);
        assert_existence::<Vec<String>>(keys, ContextKeys::EVENT_CLASSES_REGEXES, &mut count);
        assert_existence::<AdjustingMode>(keys, ContextKeys::ADJUSTING_MODE, &mut count);
        assert_existence::<String>(keys, ContextKeys::EVENT_CLASS_REGEX, &mut count);
        assert_existence::<PatternsKind>(keys, ContextKeys::PATTERNS_KIND, &mut count);
        assert_existence::<Pipeline>(keys, ContextKeys::PIPELINE, &mut count);

        assert_existence::<XesEventLogImpl>(keys, ContextKeys::EVENT_LOG, &mut count);
        assert_existence::<Activities>(keys, ContextKeys::ACTIVITIES, &mut count);
        assert_existence::<ActivitiesToLogs>(keys, ContextKeys::ACTIVITIES_TO_LOGS, &mut count);
        assert_existence::<String>(keys, ContextKeys::ACTIVITY_NAME, &mut count);
        assert_existence::<Patterns>(keys, ContextKeys::PATTERNS, &mut count);
        assert_existence::<Vec<Vec<u64>>>(keys, ContextKeys::HASHES_EVENT_LOG, &mut count);
        assert_existence::<Vec<Vec<String>>>(keys, ContextKeys::NAMES_EVENT_LOG, &mut count);
        assert_existence::<PetriNet>(keys, ContextKeys::PETRI_NET, &mut count);
        assert_existence::<RepeatSets>(keys, ContextKeys::REPEAT_SETS, &mut count);
        assert_existence::<TracesActivities>(keys, ContextKeys::TRACE_ACTIVITIES, &mut count);
        assert_existence::<ColorsEventLog>(keys, ContextKeys::COLORS_EVENT_LOG, &mut count);
        assert_existence::<ColorsHolder>(keys, ContextKeys::COLORS_HOLDER, &mut count);

        assert_eq!(count, get_all_keys_names().len())
    })
}

fn assert_existence<T: 'static>(keys: &ContextKeys, name: &str, count: &mut usize) {
    assert!(keys.find_concrete_key::<T>(name).is_some());
    *count += 1;
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
        let mut count = 0;

        assert_keys_equivalence::<String>(keys, ContextKeys::PATH, &mut count);        
        assert_keys_equivalence::<u32>(keys, ContextKeys::TANDEM_ARRAY_LENGTH, &mut count);        
        assert_keys_equivalence::<u32>(keys, ContextKeys::ACTIVITY_LEVEL, &mut count);        
        assert_keys_equivalence::<bool>(keys, ContextKeys::NARROW_ACTIVITIES, &mut count);        
        assert_keys_equivalence::<String>(keys, ContextKeys::EVENT_NAME, &mut count);        
        assert_keys_equivalence::<String>(keys, ContextKeys::REGEX, &mut count);        
        assert_keys_equivalence::<ColorsEventLog>(keys, ContextKeys::COLORS_EVENT_LOG, &mut count);        
        assert_keys_equivalence::<ColorsHolder>(keys, ContextKeys::COLORS_HOLDER, &mut count);        
        assert_keys_equivalence::<PatternsDiscoveryStrategy>(keys, ContextKeys::PATTERNS_DISCOVERY_STRATEGY, &mut count);       
        assert_keys_equivalence::<String>(keys, ContextKeys::OUTPUT_STRING, &mut count);         
        assert_keys_equivalence::<EventLogInfo>(keys, ContextKeys::EVENT_LOG_INFO, &mut count);
        assert_keys_equivalence::<usize>(keys, ContextKeys::UNDERLYING_EVENTS_COUNT, &mut count);        
        assert_keys_equivalence::<u32>(keys, ContextKeys::EVENTS_COUNT, &mut count);        
        assert_keys_equivalence::<Vec<String>>(keys, ContextKeys::EVENT_CLASSES_REGEXES, &mut count);        
        assert_keys_equivalence::<AdjustingMode>(keys, ContextKeys::ADJUSTING_MODE, &mut count);        
        assert_keys_equivalence::<String>(keys, ContextKeys::EVENT_CLASS_REGEX, &mut count);        
        assert_keys_equivalence::<PatternsKind>(keys, ContextKeys::PATTERNS_KIND, &mut count);        
        assert_keys_equivalence::<Pipeline>(keys, ContextKeys::PIPELINE, &mut count);

        assert_keys_equivalence::<XesEventLogImpl>(keys, ContextKeys::EVENT_LOG, &mut count);
        assert_keys_equivalence::<Activities>(keys, ContextKeys::ACTIVITIES, &mut count);
        assert_keys_equivalence::<ActivitiesToLogs>(keys, ContextKeys::ACTIVITIES_TO_LOGS, &mut count);        
        assert_keys_equivalence::<String>(keys, ContextKeys::ACTIVITY_NAME, &mut count);        
        assert_keys_equivalence::<Patterns>(keys, ContextKeys::PATTERNS, &mut count);        
        assert_keys_equivalence::<Vec<Vec<u64>>>(keys, ContextKeys::HASHES_EVENT_LOG, &mut count);        
        assert_keys_equivalence::<Vec<Vec<String>>>(keys, ContextKeys::NAMES_EVENT_LOG, &mut count);        
        assert_keys_equivalence::<PetriNet>(keys, ContextKeys::PETRI_NET, &mut count);        
        assert_keys_equivalence::<RepeatSets>(keys, ContextKeys::REPEAT_SETS, &mut count);        
        assert_keys_equivalence::<TracesActivities>(keys, ContextKeys::TRACE_ACTIVITIES, &mut count);        
        

        assert_eq!(count, get_all_keys_names().len())
    })
}

fn assert_keys_equivalence<T: 'static>(keys: &ContextKeys, name: &str, count: &mut usize) {
    assert!(keys.find_key(name).unwrap().key().id() == keys.find_concrete_key::<T>(name).unwrap().key().id());
    *count += 1;
}
