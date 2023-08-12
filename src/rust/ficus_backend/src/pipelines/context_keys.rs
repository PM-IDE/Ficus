use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::Rc,
};

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
    utils::user_data::{DefaultKey, Key},
};

use super::aliases::{Activities, ActivitiesToLogs, Patterns, RepeatSets, TracesActivities};

pub trait ContextKey {
    fn key(&self) -> &dyn Key;
}

pub struct DefaultContextKey<T>
where
    T: 'static,
{
    key: DefaultKey<T>,
}

impl<T> ContextKey for DefaultContextKey<T> {
    fn key(&self) -> &dyn Key {
        &self.key
    }
}

impl<T> DefaultContextKey<T>
where
    T: 'static,
{
    pub fn new(type_name: &str) -> Self {
        Self {
            key: DefaultKey::new(type_name.to_owned()),
        }
    }

    pub fn key(&self) -> &DefaultKey<T> {
        &self.key
    }
}

impl<T> PartialEq for DefaultContextKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for DefaultContextKey<T> {}

impl<T> Hash for DefaultContextKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

pub struct ContextKeys {
    pub(super) keys: HashMap<String, Box<dyn Any>>,
}

unsafe impl Sync for ContextKeys {}
unsafe impl Send for ContextKeys {}

impl ContextKeys {
    pub fn find_key(&self, name: &String) -> Option<&Box<dyn ContextKey>> {
        match self.keys.get(name) {
            Some(key) => Some(key.downcast_ref::<Box<dyn ContextKey>>().unwrap()),
            None => None,
        }
    }

    pub fn find_concrete_key<T: 'static>(&self, name: &String) -> Option<&Box<DefaultContextKey<T>>> {
        match self.keys.get(name) {
            Some(key) => Some(key.downcast_ref::<Box<DefaultContextKey<T>>>().unwrap()),
            None => None,
        }
    }

    pub fn path(&self) -> &Box<DefaultContextKey<String>> {
        self.find_concrete_key::<String>(&Self::PATH.to_string()).unwrap()
    }

    pub fn event_log(&self) -> &Box<DefaultContextKey<XesEventLogImpl>> {
        self.find_concrete_key::<XesEventLogImpl>(&Self::EVENT_LOG.to_string())
            .unwrap()
    }

    pub fn activities(&self) -> &Box<DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>>> {
        self.find_concrete_key::<Activities>(&Self::ACTIVITIES.to_string())
            .unwrap()
    }

    pub fn repeat_sets(&self) -> &Box<DefaultContextKey<Vec<SubArrayWithTraceIndex>>> {
        self.find_concrete_key::<RepeatSets>(&Self::REPEAT_SETS.to_string())
            .unwrap()
    }

    pub fn trace_activities(&self) -> &Box<DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>>> {
        self.find_concrete_key::<TracesActivities>(&Self::TRACE_ACTIVITIES.to_string())
            .unwrap()
    }

    pub fn patterns(&self) -> &Box<DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>>> {
        self.find_concrete_key::<Patterns>(&Self::PATTERNS.to_string()).unwrap()
    }

    pub fn petri_net(&self) -> &Box<DefaultContextKey<PetriNet>> {
        self.find_concrete_key::<PetriNet>(&Self::PETRI_NET.to_string())
            .unwrap()
    }

    pub fn activities_to_logs(&self) -> &Box<DefaultContextKey<HashMap<String, XesEventLogImpl>>> {
        self.find_concrete_key::<ActivitiesToLogs>(&Self::ACTIVITIES_TO_LOGS.to_string())
            .unwrap()
    }

    pub fn activity_name(&self) -> &Box<DefaultContextKey<String>> {
        self.find_concrete_key::<String>(&Self::ACTIVITY_NAME.to_string())
            .unwrap()
    }
}
