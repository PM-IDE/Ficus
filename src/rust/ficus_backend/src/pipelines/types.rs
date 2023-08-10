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
    utils::user_data::Key,
};

pub trait ContextKey: Any {}

pub struct DefaultContextKey<T>
where
    T: 'static,
{
    key: Key<T>,
}

impl<T> ContextKey for DefaultContextKey<T> {}

impl<T> DefaultContextKey<T>
where
    T: 'static,
{
    pub fn new(type_name: &str) -> Self {
        Self {
            key: Key::new(type_name.to_owned()),
        }
    }

    pub fn key(&self) -> &Key<T> {
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
    keys: HashMap<String, Box<dyn Any>>,
}

unsafe impl Sync for ContextKeys {}
unsafe impl Send for ContextKeys {}

impl ContextKeys {
    pub const PATH: &str = "path";
    pub const EVENT_LOG: &str = "event_log";
    pub const ACTIVITIES: &str = "activities";
    pub const REPEAT_SETS: &str = "repeat_sets";
    pub const TRACE_ACTIVITIES: &str = "trace_activities";
    pub const PATTERNS: &str = "patterns";
    pub const PETRI_NET: &str = "petri_net";
    pub const ACTIVITIES_TO_LOGS: &str = "activities_to_logs";
    pub const ACTIVITY_NAME: &str = "activity_name";


    pub fn new() -> Self {
        let mut keys: HashMap<String, Box<dyn Any>> = HashMap::new();
        keys.insert(
            Self::PATH.to_string(),
            Box::new(DefaultContextKey::<String>::new(Self::PATH)),
        );

        keys.insert(
            Self::EVENT_LOG.to_string(),
            Box::new(DefaultContextKey::<XesEventLogImpl>::new(Self::EVENT_LOG)),
        );

        keys.insert(
            Self::ACTIVITIES.to_string(),
            Box::new(DefaultContextKey::<Vec<Rc<RefCell<ActivityNode>>>>::new(
                Self::ACTIVITIES,
            )),
        );

        keys.insert(
            Self::REPEAT_SETS.to_string(),
            Box::new(DefaultContextKey::<Vec<SubArrayWithTraceIndex>>::new(Self::REPEAT_SETS)),
        );

        keys.insert(
            Self::TRACE_ACTIVITIES.to_string(),
            Box::new(DefaultContextKey::<Vec<Vec<ActivityInTraceInfo>>>::new(
                Self::TRACE_ACTIVITIES,
            )),
        );

        keys.insert(
            Self::PATTERNS.to_string(),
            Box::new(DefaultContextKey::<Vec<Vec<SubArrayInTraceInfo>>>::new(Self::PATTERNS)),
        );

        keys.insert(
            Self::PETRI_NET.to_string(),
            Box::new(DefaultContextKey::<PetriNet>::new(Self::PETRI_NET)),
        );

        keys.insert(
            Self::ACTIVITIES_TO_LOGS.to_string(),
            Box::new(DefaultContextKey::<HashMap<String, XesEventLogImpl>>::new(
                Self::ACTIVITIES_TO_LOGS,
            )),
        );

        keys.insert(
            Self::ACTIVITY_NAME.to_string(),
            Box::new(DefaultContextKey::<String>::new(Self::ACTIVITY_NAME)),
        );

        Self {
            keys: HashMap::from_iter(keys),
        }
    }

    pub fn find_key<T: 'static>(&self, name: &String) -> Option<&Box<T>> {
        match self.keys.get(name) {
            Some(key) => Some(key.downcast_ref::<Box<T>>().unwrap()),
            None => None,
        }
    }

    pub fn path(&self) -> &Box<DefaultContextKey<String>> {
        self.find_key::<DefaultContextKey<String>>(&Self::PATH.to_string())
            .unwrap()
    }

    pub fn event_log(&self) -> &Box<DefaultContextKey<XesEventLogImpl>> {
        self.find_key::<DefaultContextKey<XesEventLogImpl>>(&Self::EVENT_LOG.to_string())
            .unwrap()
    }

    pub fn activities(&self) -> &Box<DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>>> {
        self.find_key::<DefaultContextKey<Vec<Rc<RefCell<ActivityNode>>>>>(&Self::ACTIVITIES.to_string())
            .unwrap()
    }

    pub fn repeat_sets(&self) -> &Box<DefaultContextKey<Vec<SubArrayWithTraceIndex>>> {
        self.find_key::<DefaultContextKey<Vec<SubArrayWithTraceIndex>>>(&Self::REPEAT_SETS.to_string())
            .unwrap()
    }

    pub fn trace_activities(&self) -> &Box<DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>>> {
        self.find_key::<DefaultContextKey<Vec<Vec<ActivityInTraceInfo>>>>(&Self::TRACE_ACTIVITIES.to_string())
            .unwrap()
    }

    pub fn patterns(&self) -> &Box<DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>>> {
        self.find_key::<DefaultContextKey<Vec<Vec<SubArrayInTraceInfo>>>>(&Self::PATTERNS.to_string())
            .unwrap()
    }

    pub fn petri_net(&self) -> &Box<DefaultContextKey<PetriNet>> {
        self.find_key::<DefaultContextKey<PetriNet>>(&Self::PETRI_NET.to_string())
            .unwrap()
    }

    pub fn activities_to_logs(&self) -> &Box<DefaultContextKey<HashMap<String, XesEventLogImpl>>> {
        self.find_key::<DefaultContextKey<HashMap<String, XesEventLogImpl>>>(&Self::ACTIVITIES_TO_LOGS.to_string())
            .unwrap()
    }

    pub fn activity_name(&self) -> &Box<DefaultContextKey<String>> {
        self.find_key::<DefaultContextKey<String>>(&Self::ACTIVITY_NAME.to_string())
            .unwrap()
    }
}
