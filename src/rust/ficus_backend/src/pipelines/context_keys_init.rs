use std::{any::Any, collections::HashMap};

use crate::{event_log::xes::xes_event_log::XesEventLogImpl, features::discovery::petri_net::PetriNet};

use super::{
    aliases::{Activities, ActivitiesToLogs, Patterns, RepeatSets, TracesActivities},
    context_keys::{ContextKeys, DefaultContextKey},
};

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

        Self::insert_path(&mut keys);
        Self::insert_event_log(&mut keys);
        Self::insert_activities(&mut keys);
        Self::insert_repeat_sets(&mut keys);
        Self::insert_trace_activities(&mut keys);
        Self::insert_patterns(&mut keys);
        Self::insert_petri_net(&mut keys);
        Self::insert_activities_to_logs(&mut keys);
        Self::insert_activity_name(&mut keys);

        Self {
            keys: HashMap::from_iter(keys),
        }
    }

    fn insert_path(map: &mut HashMap<String, Box<dyn Any>>) {
        map.insert(
            Self::PATH.to_string(),
            Box::new(DefaultContextKey::<String>::new(Self::PATH)),
        );
    }

    fn insert_event_log(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<XesEventLogImpl>::new(Self::EVENT_LOG));
        map.insert(Self::EVENT_LOG.to_string(), key);
    }

    fn insert_activities(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<Activities>::new(Self::ACTIVITIES));
        map.insert(Self::ACTIVITIES.to_string(), key);
    }

    fn insert_repeat_sets(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<RepeatSets>::new(Self::REPEAT_SETS));
        map.insert(Self::REPEAT_SETS.to_string(), key);
    }

    fn insert_trace_activities(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<TracesActivities>::new(Self::TRACE_ACTIVITIES));
        map.insert(Self::TRACE_ACTIVITIES.to_string(), key);
    }

    fn insert_patterns(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<Patterns>::new(Self::PATTERNS));
        map.insert(Self::PATTERNS.to_string(), key);
    }

    fn insert_petri_net(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<PetriNet>::new(Self::PETRI_NET));
        map.insert(Self::PETRI_NET.to_string(), key);
    }

    fn insert_activities_to_logs(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<ActivitiesToLogs>::new(Self::ACTIVITIES_TO_LOGS));
        map.insert(Self::ACTIVITIES_TO_LOGS.to_string(), key);
    }

    fn insert_activity_name(map: &mut HashMap<String, Box<dyn Any>>) {
        let key = Box::new(DefaultContextKey::<String>::new(Self::ACTIVITY_NAME));
        map.insert(Self::ACTIVITY_NAME.to_string(), key);
    }
}
