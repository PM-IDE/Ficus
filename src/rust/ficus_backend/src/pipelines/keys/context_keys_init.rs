use std::{any::Any, borrow::Cow, collections::HashMap};

use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::{
    event_log::xes::xes_event_log::XesEventLogImpl,
    features::{
        analysis::{
            event_log_info::EventLogInfo,
            patterns::{
                activity_instances::AdjustingMode, contexts::PatternsDiscoveryStrategy,
            },
        },
        discovery::petri_net::PetriNet,
    },
    pipelines::{aliases::*, pipelines::Pipeline},
    utils::colors::ColorsHolder,
};

use super::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys::ContextKeys,
};

pub(super) type ConcreteKeysStorage = HashMap<Cow<'static, str>, Box<dyn Any>>;
pub(super) type ContextKeysStorage = HashMap<Cow<'static, str>, Box<dyn ContextKey>>;

struct ContextKeysInitContext {
    concrete_keys: ConcreteKeysStorage,
    context_keys: ContextKeysStorage,
}

impl ContextKeysInitContext {
    fn empty() -> Self {
        Self {
            concrete_keys: ConcreteKeysStorage::new(),
            context_keys: ContextKeysStorage::new(),
        }
    }

    fn insert<T>(&mut self, name: &'static str, key: &Box<DefaultContextKey<T>>) {
        self.insert_concrete(name, key.clone());
        self.insert_context(name, key.clone());
    }

    fn insert_concrete<T>(&mut self, name: &'static str, key: Box<DefaultContextKey<T>>) {
        let prev = self.context_keys.insert(Cow::Borrowed(name), key.clone());
        assert!(prev.is_none());
    }

    fn insert_context<T>(&mut self, name: &'static str, key: Box<DefaultContextKey<T>>) {
        let prev = self.concrete_keys.insert(Cow::Borrowed(name), key.clone());
        assert!(prev.is_none());
    }

    fn deconstruct(self) -> (ConcreteKeysStorage, ContextKeysStorage) {
        (self.concrete_keys, self.context_keys)
    }
}

impl ContextKeys {
    pub const PATH: &str = "path";
    pub const TANDEM_ARRAY_LENGTH: &str = "tandem_array_length";
    pub const ACTIVITY_LEVEL: &str = "activity_level";
    pub const NARROW_ACTIVITIES: &str = "narrow_activities";
    pub const EVENT_NAME: &str = "event_name";
    pub const REGEX: &str = "regex";
    pub const PATTERNS_DISCOVERY_STRATEGY: &str = "patterns_discovery_strategy";
    pub const OUTPUT_STRING: &str = "output_string";
    pub const EVENT_LOG_INFO: &str = "event_log_info";
    pub const UNDERLYING_EVENTS_COUNT: &str = "underlying_events_count";
    pub const EVENTS_COUNT: &str = "events_count";
    pub const EVENT_CLASSES_REGEXES: &str = "event_classes_regexes";
    pub const ADJUSTING_MODE: &str = "adjusting_mode";
    pub const EVENT_CLASS_REGEX: &str = "event_class_regex";
    pub const PATTERNS_KIND: &str = "patterns_kind";
    pub const PIPELINE: &str = "pipeline";

    pub const EVENT_LOG: &str = "event_log";
    pub const ACTIVITIES: &str = "activities";
    pub const REPEAT_SETS: &str = "repeat_sets";
    pub const TRACE_ACTIVITIES: &str = "trace_activities";
    pub const PATTERNS: &str = "patterns";
    pub const PETRI_NET: &str = "petri_net";
    pub const ACTIVITIES_TO_LOGS: &str = "activities_to_logs";
    pub const ACTIVITY_NAME: &str = "activity_name";
    pub const HASHES_EVENT_LOG: &str = "hashes_event_log";
    pub const NAMES_EVENT_LOG: &str = "names_event_log";
    pub const COLORS_EVENT_LOG: &str = "colors_event_log";
    pub const COLORS_HOLDER: &str = "colors_holder";

    pub fn new() -> Self {
        let mut context = ContextKeysInitContext::empty();

        Self::insert_path(&mut context);
        Self::insert_tandem_arrays_length(&mut context);
        Self::insert_activity_level(&mut context);
        Self::insert_narrow_activities(&mut context);
        Self::insert_event_name(&mut context);
        Self::insert_regex(&mut context);
        Self::insert_patterns_discovery_strategy(&mut context);
        Self::insert_output_string(&mut context);
        Self::insert_event_log_info(&mut context);
        Self::insert_underlying_events_count(&mut context);
        Self::insert_events_count(&mut context);
        Self::insert_event_classes_regexes(&mut context);
        Self::insert_adjusting_mode(&mut context);
        Self::insert_event_class_regex(&mut context);
        Self::insert_patterns_kind(&mut context);
        Self::insert_pipeline(&mut context);

        Self::insert_event_log(&mut context);
        Self::insert_activities(&mut context);
        Self::insert_repeat_sets(&mut context);
        Self::insert_trace_activities(&mut context);
        Self::insert_patterns(&mut context);
        Self::insert_petri_net(&mut context);
        Self::insert_activities_to_logs(&mut context);
        Self::insert_activity_name(&mut context);

        Self::insert_hashes_event_log(&mut context);
        Self::insert_names_event_log(&mut context);
        Self::insert_colors_event_log(&mut context);
        Self::insert_colors_holder(&mut context);

        let (concrete_keys, context_keys) = context.deconstruct();

        Self {
            concrete_keys,
            context_keys,
        }
    }

    fn insert_path(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::PATH);
    }

    fn insert_key<T: 'static>(context: &mut ContextKeysInitContext, name: &'static str) {
        let key = Box::new(DefaultContextKey::<T>::new(name));
        Self::insert_key_to_map(context, key, name);
    }

    fn insert_key_to_map<T: 'static>(
        context: &mut ContextKeysInitContext,
        key: Box<DefaultContextKey<T>>,
        name: &'static str,
    ) {
        context.insert(name, &key);
    }

    fn insert_tandem_arrays_length(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::TANDEM_ARRAY_LENGTH);
    }

    fn insert_activity_level(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::ACTIVITY_LEVEL);
    }

    fn insert_narrow_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<bool>(context, Self::NARROW_ACTIVITIES);
    }

    fn insert_event_name(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::EVENT_NAME);
    }

    fn insert_regex(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::REGEX);
    }

    fn insert_patterns_discovery_strategy(context: &mut ContextKeysInitContext) {
        Self::insert_key::<PatternsDiscoveryStrategy>(context, Self::PATTERNS_DISCOVERY_STRATEGY);
    }

    fn insert_output_string(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::OUTPUT_STRING);
    }

    fn insert_event_log_info(context: &mut ContextKeysInitContext) {
        Self::insert_key::<EventLogInfo>(context, Self::EVENT_LOG_INFO);
    }

    fn insert_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<XesEventLogImpl>(context, Self::EVENT_LOG);
    }

    fn insert_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Activities>(context, Self::ACTIVITIES);
    }

    fn insert_repeat_sets(context: &mut ContextKeysInitContext) {
        Self::insert_key::<RepeatSets>(context, Self::REPEAT_SETS);
    }

    fn insert_trace_activities(context: &mut ContextKeysInitContext) {
        Self::insert_key::<TracesActivities>(context, Self::TRACE_ACTIVITIES);
    }

    fn insert_patterns(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Patterns>(context, Self::PATTERNS);
    }

    fn insert_petri_net(context: &mut ContextKeysInitContext) {
        Self::insert_key::<PetriNet>(context, Self::PETRI_NET);
    }

    fn insert_activities_to_logs(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ActivitiesToLogs>(context, Self::ACTIVITIES_TO_LOGS);
    }

    fn insert_activity_name(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::ACTIVITY_NAME);
    }

    fn insert_hashes_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Vec<Vec<u64>>>(context, Self::HASHES_EVENT_LOG);
    }

    fn insert_names_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Vec<Vec<String>>>(context, Self::NAMES_EVENT_LOG);
    }

    fn insert_colors_event_log(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ColorsEventLog>(context, Self::COLORS_EVENT_LOG);
    }

    fn insert_colors_holder(context: &mut ContextKeysInitContext) {
        Self::insert_key::<ColorsHolder>(context, Self::COLORS_HOLDER);
    }

    fn insert_underlying_events_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<usize>(context, Self::UNDERLYING_EVENTS_COUNT);
    }

    fn insert_events_count(context: &mut ContextKeysInitContext) {
        Self::insert_key::<u32>(context, Self::EVENTS_COUNT);
    }

    fn insert_event_classes_regexes(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Vec<String>>(context, Self::EVENT_CLASSES_REGEXES);
    }

    fn insert_adjusting_mode(context: &mut ContextKeysInitContext) {
        Self::insert_key::<AdjustingMode>(context, Self::ADJUSTING_MODE)
    }

    fn insert_event_class_regex(context: &mut ContextKeysInitContext) {
        Self::insert_key::<String>(context, Self::EVENT_CLASS_REGEX)
    }

    fn insert_patterns_kind(context: &mut ContextKeysInitContext) {
        Self::insert_key::<PatternsKindDto>(context, Self::PATTERNS_KIND)
    }

    fn insert_pipeline(context: &mut ContextKeysInitContext) {
        Self::insert_key::<Pipeline>(context, Self::PIPELINE)
    }
}
