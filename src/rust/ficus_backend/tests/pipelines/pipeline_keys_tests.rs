use std::sync::Arc;

use ficus_backend::{
    event_log::{core::event_log::EventLog, xes::xes_event_log::XesEventLogImpl},
    features::{analysis::patterns::contexts::PatternsDiscoveryStrategy, discovery::petri_net::PetriNet},
    pipelines::{
        aliases::{Activities, ActivitiesToLogs, ColorsEventLog, Patterns, RepeatSets, TracesActivities},
        context::PipelineContext,
        keys::context_keys::ContextKeys,
    },
    utils::{
        colors::ColorsHolder,
        user_data::{keys::Key, user_data::UserData},
    },
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

        assert!(keys.find_concrete_key::<u32>(ContextKeys::TANDEM_ARRAY_LENGTH).is_some());
        assert!(keys.find_concrete_key::<u32>(ContextKeys::ACTIVITY_LEVEL).is_some());
        assert!(keys.find_concrete_key::<bool>(ContextKeys::NARROW_ACTIVITIES).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::EVENT_NAME).is_some());
        assert!(keys.find_concrete_key::<String>(ContextKeys::REGEX).is_some());
        assert!(keys.find_concrete_key::<PatternsDiscoveryStrategy>(ContextKeys::PATTERNS_DISCOVERY_STRATEGY).is_some());
    })
}

#[test]
#[rustfmt::skip]
fn test_event_log_all_keys() {
    execute_test(|keys, _| {
        assert!(keys.find_key(ContextKeys::EVENT_LOG).is_some());
        assert!(keys.find_key(ContextKeys::ACTIVITIES).is_some());
        assert!(keys.find_key(ContextKeys::ACTIVITIES_TO_LOGS).is_some());
        assert!(keys.find_key(ContextKeys::ACTIVITY_NAME).is_some());
        assert!(keys.find_key(ContextKeys::PATTERNS).is_some());
        assert!(keys.find_key(ContextKeys::HASHES_EVENT_LOG).is_some());
        assert!(keys.find_key(ContextKeys::NAMES_EVENT_LOG).is_some());
        assert!(keys.find_key(ContextKeys::PETRI_NET).is_some());
        assert!(keys.find_key(ContextKeys::REPEAT_SETS).is_some());
        assert!(keys.find_key(ContextKeys::TRACE_ACTIVITIES).is_some());
        assert!(keys.find_key(ContextKeys::PATH).is_some());
        assert!(keys.find_key(ContextKeys::TANDEM_ARRAY_LENGTH).is_some());
        assert!(keys.find_key(ContextKeys::ACTIVITY_LEVEL).is_some());
        assert!(keys.find_key(ContextKeys::NARROW_ACTIVITIES).is_some());
        assert!(keys.find_key(ContextKeys::EVENT_NAME).is_some());
        assert!(keys.find_key(ContextKeys::REGEX).is_some());
        assert!(keys.find_key(ContextKeys::COLORS_EVENT_LOG).is_some());
        assert!(keys.find_key(ContextKeys::NAMES_EVENT_LOG).is_some());
        assert!(keys.find_key(ContextKeys::PATTERNS_DISCOVERY_STRATEGY).is_some());
        assert!(keys.find_key(ContextKeys::UNDERLYING_EVENTS_COUNT).is_some());
        assert!(keys.find_key(ContextKeys::EVENTS_COUNT).is_some());
    })
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
    })
}
