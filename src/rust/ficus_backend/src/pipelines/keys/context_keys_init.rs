use std::{any::Any, borrow::Cow, collections::HashMap, rc::Rc};

use crate::{
    event_log::{
        core::{
            event::{event::Event, event_hasher::NameEventHasher},
            event_log::EventLog,
            trace::trace::Trace,
        },
        xes::xes_event_log::XesEventLogImpl,
    },
    features::discovery::petri_net::PetriNet,
    pipelines::aliases::*,
    utils::user_data::user_data::UserData,
};

use super::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys::ContextKeys,
};

impl ContextKeys {
    pub const PATH: &str = "path";
    pub const TANDEM_ARRAY_LENGTH: &str = "tandem_array_length";
    pub const ACTIVITY_LEVEL: &str = "activity_level";
    pub const NARROW_ACTIVITIES: &str = "narrow_activities";

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

    pub fn new() -> Self {
        let mut concrete_keys: HashMap<Cow<'static, str>, Box<dyn Any>> = HashMap::new();
        let mut context_keys: HashMap<Cow<'static, str>, Box<dyn ContextKey>> = HashMap::new();

        Self::insert_path(&mut concrete_keys, &mut context_keys);
        Self::insert_tandem_arrays_length(&mut concrete_keys, &mut context_keys);
        Self::insert_activity_level(&mut concrete_keys, &mut context_keys);
        Self::insert_narrow_activities(&mut concrete_keys, &mut context_keys);

        Self::insert_event_log(&mut concrete_keys, &mut context_keys);
        Self::insert_activities(&mut concrete_keys, &mut context_keys);
        Self::insert_repeat_sets(&mut concrete_keys, &mut context_keys);
        Self::insert_trace_activities(&mut concrete_keys, &mut context_keys);
        Self::insert_patterns(&mut concrete_keys, &mut context_keys);
        Self::insert_petri_net(&mut concrete_keys, &mut context_keys);
        Self::insert_activities_to_logs(&mut concrete_keys, &mut context_keys);
        Self::insert_activity_name(&mut concrete_keys, &mut context_keys);

        Self::insert_hashes_event_log(&mut concrete_keys, &mut context_keys);
        Self::insert_names_event_log(&mut concrete_keys, &mut context_keys);

        Self {
            concrete_keys,
            context_keys,
        }
    }

    fn insert_path(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<String>(concrete_keys, context_keys, Self::PATH);
    }

    fn insert_key<T: 'static>(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
        name: &'static str,
    ) {
        let key = Box::new(DefaultContextKey::<T>::new(name));
        Self::insert_key_to_map(concrete_keys, context_keys, key, name);
    }

    fn insert_key_to_map<T: 'static>(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
        key: Box<DefaultContextKey<T>>,
        name: &'static str,
    ) {
        context_keys.insert(Cow::Borrowed(name), key.clone());
        concrete_keys.insert(Cow::Borrowed(name), key.clone());
    }

    fn insert_tandem_arrays_length(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<u32>(concrete_keys, context_keys, Self::TANDEM_ARRAY_LENGTH);
    }

    fn insert_activity_level(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<usize>(concrete_keys, context_keys, Self::ACTIVITY_LEVEL);
    }

    fn insert_narrow_activities(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<bool>(concrete_keys, context_keys, Self::NARROW_ACTIVITIES);
    }

    fn insert_event_log(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<XesEventLogImpl>(concrete_keys, context_keys, Self::EVENT_LOG);
    }

    fn insert_activities(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<Activities>(concrete_keys, context_keys, Self::ACTIVITIES);
    }

    fn insert_repeat_sets(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<RepeatSets>(concrete_keys, context_keys, Self::REPEAT_SETS);
    }

    fn insert_trace_activities(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<TracesActivities>(concrete_keys, context_keys, Self::TRACE_ACTIVITIES);
    }

    fn insert_patterns(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<Patterns>(concrete_keys, context_keys, Self::PATTERNS);
    }

    fn insert_petri_net(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<PetriNet>(concrete_keys, context_keys, Self::PETRI_NET);
    }

    fn insert_activities_to_logs(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<ActivitiesToLogs>(concrete_keys, context_keys, Self::ACTIVITIES_TO_LOGS);
    }

    fn insert_activity_name(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        Self::insert_key::<String>(concrete_keys, context_keys, Self::ACTIVITY_NAME);
    }

    fn insert_hashes_event_log(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = DefaultContextKey::<Vec<Vec<u64>>>::new_with_factory(
            Self::HASHES_EVENT_LOG.to_string(),
            Rc::new(Box::new(|pipeline_context, keys| {
                match pipeline_context.get_concrete(keys.event_log().key()) {
                    None => None,
                    Some(log) => Some(log.to_hashes_event_log::<NameEventHasher>()),
                }
            })),
        );

        Self::insert_key_to_map(concrete_keys, context_keys, Box::new(key), Self::HASHES_EVENT_LOG);
    }

    fn insert_names_event_log(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = DefaultContextKey::<Vec<Vec<String>>>::new_with_factory(
            Self::NAMES_EVENT_LOG.to_string(),
            Rc::new(Box::new(|pipeline_context, keys| {
                match pipeline_context.get_concrete(keys.event_log().key()) {
                    None => None,
                    Some(log) => {
                        let mut result = vec![];
                        for trace in log.get_traces() {
                            let mut vec = vec![];
                            for event in trace.borrow().get_events() {
                                vec.push(event.borrow().get_name().to_string());
                            }

                            result.push(vec);
                        }

                        Some(result)
                    }
                }
            })),
        );

        Self::insert_key_to_map(concrete_keys, context_keys, Box::new(key), Self::NAMES_EVENT_LOG);
    }
}
