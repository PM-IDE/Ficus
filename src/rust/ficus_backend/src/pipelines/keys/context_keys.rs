use std::{any::Any, borrow::Cow, cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::{
    event_log::xes::xes_event_log::XesEventLogImpl,
    features::{
        analysis::patterns::{
            activity_instances::ActivityInTraceInfo,
            repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
            tandem_arrays::SubArrayInTraceInfo,
        },
        discovery::petri_net::PetriNet,
    },
    pipelines::aliases::*,
    utils::{colors::Color, user_data::keys::Key},
};

use super::context_key::{ContextKey, DefaultContextKey};

pub struct ContextKeys {
    pub(super) concrete_keys: HashMap<Cow<'static, str>, Box<dyn Any>>,
    pub(super) context_keys: HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
}

unsafe impl Sync for ContextKeys {}
unsafe impl Send for ContextKeys {}

impl ContextKeys {
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
        return self.path().key().id() == key.key().id();
    }

    pub fn event_log(&self) -> &DefaultContextKey<XesEventLogImpl> {
        self.find_concrete_key::<XesEventLogImpl>(Self::EVENT_LOG).unwrap()
    }

    pub fn is_event_log(&self, key: &dyn ContextKey) -> bool {
        return self.event_log().key().id() == key.key().id();
    }

    pub fn activities(&self) -> &DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>> {
        self.find_concrete_key::<Activities>(Self::ACTIVITIES).unwrap()
    }

    pub fn is_activities(&self, key: &dyn ContextKey) -> bool {
        return self.activities().key().id() == key.key().id();
    }

    pub fn repeat_sets(&self) -> &DefaultContextKey<Vec<SubArrayWithTraceIndex>> {
        self.find_concrete_key::<RepeatSets>(Self::REPEAT_SETS).unwrap()
    }

    pub fn is_repeat_sets(&self, key: &dyn ContextKey) -> bool {
        return self.repeat_sets().key().id() == key.key().id();
    }

    pub fn trace_activities(&self) -> &DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>> {
        self.find_concrete_key::<TracesActivities>(Self::TRACE_ACTIVITIES)
            .unwrap()
    }

    pub fn is_trace_activities(&self, key: &dyn ContextKey) -> bool {
        return self.trace_activities().key().id() == key.key().id();
    }

    pub fn patterns(&self) -> &DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>> {
        self.find_concrete_key::<Patterns>(Self::PATTERNS).unwrap()
    }

    pub fn is_patterns(&self, key: &dyn ContextKey) -> bool {
        return self.patterns().key().id() == key.key().id();
    }

    pub fn petri_net(&self) -> &DefaultContextKey<PetriNet> {
        self.find_concrete_key::<PetriNet>(Self::PETRI_NET).unwrap()
    }

    pub fn is_petri_net(&self, key: &dyn ContextKey) -> bool {
        return self.petri_net().key().id() == key.key().id();
    }

    pub fn activities_to_logs(&self) -> &DefaultContextKey<HashMap<String, XesEventLogImpl>> {
        self.find_concrete_key::<ActivitiesToLogs>(Self::ACTIVITIES_TO_LOGS)
            .unwrap()
    }

    pub fn is_activities_to_logs(&self, key: &dyn ContextKey) -> bool {
        return self.activities_to_logs().key().id() == key.key().id();
    }

    pub fn activity_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::ACTIVITY_NAME).unwrap()
    }

    pub fn is_activity_name(&self, key: &dyn ContextKey) -> bool {
        return self.activity_name().key().id() == key.key().id();
    }

    pub fn hashes_event_log(&self) -> &DefaultContextKey<Vec<Vec<u64>>> {
        self.find_concrete_key::<Vec<Vec<u64>>>(Self::HASHES_EVENT_LOG).unwrap()
    }

    pub fn is_hashes_event_log(&self, key: &dyn ContextKey) -> bool {
        return self.hashes_event_log().key().id() == key.key().id();
    }

    pub fn names_event_log(&self) -> &DefaultContextKey<Vec<Vec<String>>> {
        self.find_concrete_key::<Vec<Vec<String>>>(Self::NAMES_EVENT_LOG)
            .unwrap()
    }

    pub fn is_names_event_log(&self, key: &dyn ContextKey) -> bool {
        return self.names_event_log().key().id() == key.key().id();
    }

    pub fn tandem_array_length(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::TANDEM_ARRAY_LENGTH).unwrap()
    }

    pub fn is_tandem_array_length(&self, key: &dyn ContextKey) -> bool {
        return self.tandem_array_length().key().id() == key.key().id();
    }

    pub fn activity_level(&self) -> &DefaultContextKey<u32> {
        self.find_concrete_key::<u32>(Self::ACTIVITY_LEVEL).unwrap()
    }

    pub fn is_activity_level(&self, key: &dyn ContextKey) -> bool {
        return self.activity_level().key().id() == key.key().id();
    }

    pub fn narrow_activities(&self) -> &DefaultContextKey<bool> {
        self.find_concrete_key::<bool>(Self::NARROW_ACTIVITIES).unwrap()
    }

    pub fn is_narrow_activities(&self, key: &dyn ContextKey) -> bool {
        return self.narrow_activities().key().id() == key.key().id();
    }

    pub fn event_name(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::EVENT_NAME).unwrap()
    }

    pub fn is_event_name(&self, key: &dyn ContextKey) -> bool {
        return self.event_name().key().id() == key.key().id();
    }

    pub fn regex(&self) -> &DefaultContextKey<String> {
        self.find_concrete_key::<String>(Self::REGEX).unwrap()
    }

    pub fn is_regex(&self, key: &dyn ContextKey) -> bool {
        return self.regex().key().id() == key.key().id();
    }

    pub fn colors_event_log(&self) -> &DefaultContextKey<ColorsEventLog> {
        self.find_concrete_key::<ColorsEventLog>(Self::COLORS_EVENT_LOG)
            .unwrap()
    }

    pub fn is_colors_event_log(&self, key: &dyn ContextKey) -> bool {
        return self.colors_event_log().key().id() == key.key().id();
    }

    pub fn names_to_colors(&self) -> &DefaultContextKey<NamesToColors> {
        self.find_concrete_key::<NamesToColors>(Self::NAMES_TO_COLORS).unwrap()
    }

    pub fn is_names_to_colors(&self, key: &dyn ContextKey) -> bool {
        return self.names_to_colors().key().id() == key.key().id();
    }
}
