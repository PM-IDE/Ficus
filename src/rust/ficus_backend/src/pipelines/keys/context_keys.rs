use std::{any::Any, borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

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
};

use super::context_key::{ContextKey, DefaultContextKey};

pub struct ContextKeys {
    pub(super) keys: HashMap<Cow<'static, str>, Box<dyn Any>>,
}

unsafe impl Sync for ContextKeys {}
unsafe impl Send for ContextKeys {}

impl ContextKeys {
    pub fn find_key(&self, name: &str) -> Option<&Box<dyn ContextKey>> {
        match self.keys.get(name) {
            Some(key) => Some(key.downcast_ref::<Box<dyn ContextKey>>().unwrap()),
            None => None,
        }
    }

    pub fn find_concrete_key<T: 'static>(&self, name: &str) -> Option<&Box<DefaultContextKey<T>>> {
        match self.keys.get(name) {
            Some(key) => Some(key.downcast_ref::<Box<DefaultContextKey<T>>>().unwrap()),
            None => None,
        }
    }

    pub fn path(&self) -> &Box<DefaultContextKey<String>> {
        self.find_concrete_key::<String>(Self::PATH).unwrap()
    }

    pub fn event_log(&self) -> &Box<DefaultContextKey<XesEventLogImpl>> {
        self.find_concrete_key::<XesEventLogImpl>(Self::EVENT_LOG).unwrap()
    }

    pub fn activities(&self) -> &Box<DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>>> {
        self.find_concrete_key::<Activities>(Self::ACTIVITIES).unwrap()
    }

    pub fn repeat_sets(&self) -> &Box<DefaultContextKey<Vec<SubArrayWithTraceIndex>>> {
        self.find_concrete_key::<RepeatSets>(Self::REPEAT_SETS).unwrap()
    }

    pub fn trace_activities(&self) -> &Box<DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>>> {
        self.find_concrete_key::<TracesActivities>(Self::TRACE_ACTIVITIES)
            .unwrap()
    }

    pub fn patterns(&self) -> &Box<DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>>> {
        self.find_concrete_key::<Patterns>(Self::PATTERNS).unwrap()
    }

    pub fn petri_net(&self) -> &Box<DefaultContextKey<PetriNet>> {
        self.find_concrete_key::<PetriNet>(Self::PETRI_NET).unwrap()
    }

    pub fn activities_to_logs(&self) -> &Box<DefaultContextKey<HashMap<String, XesEventLogImpl>>> {
        self.find_concrete_key::<ActivitiesToLogs>(Self::ACTIVITIES_TO_LOGS)
            .unwrap()
    }

    pub fn activity_name(&self) -> &Box<DefaultContextKey<String>> {
        self.find_concrete_key::<String>(Self::ACTIVITY_NAME).unwrap()
    }

    pub fn hashes_event_log(&self) -> &Box<DefaultContextKey<Vec<Vec<u64>>>> {
        self.find_concrete_key::<Vec<Vec<u64>>>(Self::HASHES_EVENT_LOG).unwrap()
    }

    pub fn names_event_log(&self) -> &Box<DefaultContextKey<Vec<Vec<String>>>> {
        self.find_concrete_key::<Vec<Vec<String>>>(Self::NAMES_EVENT_LOG)
            .unwrap()
    }
}
