use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::pipelines::activities_parts::{ActivitiesLogsSourceDto, UndefActivityHandlingStrategyDto};
use crate::pipelines::patterns_parts::PatternsKindDto;
use crate::{
    event_log::xes::xes_event_log::XesEventLogImpl,
    features::{
        analysis::{
            event_log_info::EventLogInfo,
            patterns::{
                activity_instances::{ActivityInTraceInfo, AdjustingMode},
                contexts::PatternsDiscoveryStrategy,
                repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
                tandem_arrays::SubArrayInTraceInfo,
            },
        },
        discovery::petri_net::petri_net::PetriNet,
    },
    pipelines::{aliases::*, pipelines::Pipeline},
    utils::colors::ColorsHolder,
};

use super::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys_init::{ConcreteKeysStorage, ContextKeysStorage},
};

pub struct ContextKeys {
    pub(super) concrete_keys: ConcreteKeysStorage,
    pub(super) context_keys: ContextKeysStorage,
}

unsafe impl Sync for ContextKeys {}
unsafe impl Send for ContextKeys {}

impl ContextKeys {
    pub fn len(&self) -> usize {
        self.concrete_keys.len()
    }

    pub fn find_key(&self, name: &str) -> Option<&Box<dyn ContextKey>> {
        self.context_keys.get(name)
    }

    pub fn find_concrete_key<T: 'static>(&self, name: &str) -> Option<&DefaultContextKey<T>> {
        match self.concrete_keys.get(name) {
            Some(key) => Some(key.downcast_ref::<DefaultContextKey<T>>().unwrap()),
            None => None,
        }
    }

    pub fn path(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::PATH).unwrap()
    }

    pub fn is_path(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.path())
    }

    fn are_keys_equal(first: &dyn ContextKey, second: &dyn ContextKey) -> bool {
        first.key().id() == second.key().id()
    }

    pub fn event_log(&self) -> &DefaultContextKey<XesEventLogImpl> {
        self.find_concrete_key::<XesEventLogImpl>(Self::EVENT_LOG).unwrap()
    }

    pub fn is_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.event_log())
    }

    pub fn activities(&self) -> &DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>> {
        self.find_concrete_key::<Activities>(Self::ACTIVITIES).unwrap()
    }

    pub fn is_activities(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activities())
    }

    pub fn repeat_sets(&self) -> &DefaultContextKey<Vec<SubArrayWithTraceIndex>> {
        self.find_concrete_key::<RepeatSets>(Self::REPEAT_SETS).unwrap()
    }

    pub fn is_repeat_sets(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.repeat_sets())
    }

    pub fn trace_activities(&self) -> &DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>> {
        self.find_concrete_key::<TracesActivities>(Self::TRACE_ACTIVITIES)
            .unwrap()
    }

    pub fn is_trace_activities(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.trace_activities())
    }

    pub fn patterns(&self) -> &DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>> {
        self.find_concrete_key::<Patterns>(Self::PATTERNS).unwrap()
    }

    pub fn is_patterns(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.patterns())
    }

    pub fn petri_net(&self) -> &DefaultContextKey<DefaultPetriNet> {
        self.find_concrete_key::<DefaultPetriNet>(Self::PETRI_NET).unwrap()
    }

    pub fn is_petri_net(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.petri_net())
    }

    pub fn activities_to_logs(&self) -> &DefaultContextKey<HashMap<String, XesEventLogImpl>> {
        self.find_concrete_key::<ActivitiesToLogs>(Self::ACTIVITIES_TO_LOGS)
            .unwrap()
    }

    pub fn is_activities_to_logs(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activities_to_logs())
    }

    pub fn activity_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::ACTIVITY_NAME).unwrap()
    }

    pub fn is_activity_name(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activity_name())
    }

    pub fn hashes_event_log(&self) -> &DefaultContextKey<Vec<Vec<u64>>> {
        self.find_concrete_key::<Vec<Vec<u64>>>(Self::HASHES_EVENT_LOG).unwrap()
    }

    pub fn is_hashes_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.hashes_event_log())
    }

    pub fn names_event_log(&self) -> &DefaultContextKey<Vec<Vec<String>>> {
        self.find_concrete_key::<Vec<Vec<String>>>(Self::NAMES_EVENT_LOG)
            .unwrap()
    }

    pub fn is_names_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.names_event_log())
    }

    pub fn tandem_array_length(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::TANDEM_ARRAY_LENGTH).unwrap()
    }

    pub fn is_tandem_array_length(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.tandem_array_length())
    }

    pub fn activity_level(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::ACTIVITY_LEVEL).unwrap()
    }

    pub fn is_activity_level(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.activity_level())
    }

    pub fn narrow_activities(&self) -> &DefaultContextKey<ActivityNarrowingKind> {
        self.find_concrete_key::<ActivityNarrowingKind>(Self::NARROW_ACTIVITIES)
            .unwrap()
    }

    pub fn is_narrow_activities(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.narrow_activities())
    }

    pub fn event_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::EVENT_NAME).unwrap()
    }

    pub fn is_event_name(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.event_name())
    }

    pub fn regex(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::REGEX).unwrap()
    }

    pub fn is_regex(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.regex())
    }

    pub fn colors_event_log(&self) -> &DefaultContextKey<ColorsEventLog> {
        self.find_concrete_key::<ColorsEventLog>(Self::COLORS_EVENT_LOG)
            .unwrap()
    }

    pub fn is_colors_event_log(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.colors_event_log())
    }

    pub fn colors_holder(&self) -> &DefaultContextKey<ColorsHolder> {
        self.find_concrete_key::<ColorsHolder>(Self::COLORS_HOLDER).unwrap()
    }

    pub fn is_colors_holder(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.colors_holder())
    }

    pub fn patterns_discovery_strategy(&self) -> &DefaultContextKey<PatternsDiscoveryStrategy> {
        self.find_concrete_key::<PatternsDiscoveryStrategy>(Self::PATTERNS_DISCOVERY_STRATEGY)
            .unwrap()
    }

    pub fn is_patterns_discovery_strategy(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(key, self.patterns_discovery_strategy())
    }

    pub fn output_string(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::OUTPUT_STRING).unwrap()
    }

    pub fn is_output_string(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.output_string(), key)
    }

    pub fn event_log_info(&self) -> &DefaultContextKey<EventLogInfo> {
        self.find_concrete_key::<EventLogInfo>(Self::EVENT_LOG_INFO).unwrap()
    }

    pub fn is_event_log_info(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_log_info(), key)
    }

    pub fn underlying_events_count(&self) -> &DefaultContextKey<usize> {
        self.find_concrete_key::<usize>(Self::UNDERLYING_EVENTS_COUNT).unwrap()
    }

    pub fn is_underlying_events_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.underlying_events_count(), key)
    }

    pub fn events_count(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::EVENTS_COUNT).unwrap()
    }

    pub fn is_events_count(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.events_count(), key)
    }

    pub fn event_classes_regexes(&self) -> &DefaultContextKey<Vec<String>> {
        self.find_concrete_key::<Vec<String>>(Self::EVENT_CLASSES_REGEXES)
            .unwrap()
    }

    pub fn is_event_classes_regexes(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_classes_regexes(), key)
    }

    pub fn adjusting_mode(&self) -> &DefaultContextKey<AdjustingMode> {
        self.find_concrete_key::<AdjustingMode>(Self::ADJUSTING_MODE).unwrap()
    }

    pub fn is_adjusting_mode(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.adjusting_mode(), key)
    }

    pub fn event_class_regex(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::EVENT_CLASS_REGEX).unwrap()
    }

    pub fn is_vent_class_regex(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.event_class_regex(), key)
    }

    pub fn patterns_kind(&self) -> &DefaultContextKey<PatternsKindDto> {
        self.find_concrete_key::<PatternsKindDto>(Self::PATTERNS_KIND).unwrap()
    }

    pub fn is_patterns_kind(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.patterns_kind(), key)
    }

    pub fn pipeline(&self) -> &DefaultContextKey<Pipeline> {
        self.find_concrete_key::<Pipeline>(Self::PIPELINE).unwrap()
    }

    pub fn is_pipeline(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.pipeline(), key)
    }

    pub fn min_activity_length(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::MIN_ACTIVITY_LENGTH).unwrap()
    }

    pub fn is_min_activity_length(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.min_activity_length(), key)
    }

    pub fn undef_activity_handling_strategy(&self) -> &DefaultContextKey<UndefActivityHandlingStrategyDto> {
        self.find_concrete_key::<UndefActivityHandlingStrategyDto>(Self::UNDEF_ACTIVITY_HANDLING_STRATEGY)
            .unwrap()
    }

    pub fn is_undef_activity_handling_strategy(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.undef_activity_handling_strategy(), key)
    }

    pub fn activity_filter_kind(&self) -> &DefaultContextKey<ActivityInTraceFilterKind> {
        self.find_concrete_key::<ActivityInTraceFilterKind>(Self::ACTIVITY_IN_TRACE_FILTER_KIND)
            .unwrap()
    }

    pub fn is_activity_filter_kind(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.activity_filter_kind(), key)
    }

    pub fn activities_logs_source(&self) -> &DefaultContextKey<ActivitiesLogsSourceDto> {
        self.find_concrete_key::<ActivitiesLogsSourceDto>(Self::ACTIVITIES_LOGS_SOURCE)
            .unwrap()
    }

    pub fn is_activities_logs_source(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.activities_logs_source(), key)
    }

    pub fn pnml_use_names_as_ids(&self) -> &DefaultContextKey<bool> {
        self.find_concrete_key::<bool>(Self::PNML_USE_NAMES_AS_IDS).unwrap()
    }

    pub fn is_pnml_use_names_as_ids(&self, key: &dyn ContextKey) -> bool {
        Self::are_keys_equal(self.pnml_use_names_as_ids(), key)
    }
}
