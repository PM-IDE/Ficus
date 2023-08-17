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
};

use super::{
    context_key::{ContextKey, DefaultContextKey},
    context_keys::ContextKeys,
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
    pub const HASHES_EVENT_LOG: &str = "hashes_event_log";
    pub const NAMES_EVENT_LOG: &str = "names_event_log";

    pub fn new() -> Self {
        let mut concrete_keys: HashMap<Cow<'static, str>, Box<dyn Any>> = HashMap::new();
        let mut context_keys: HashMap<Cow<'static, str>, Box<dyn ContextKey>> = HashMap::new();

        Self::insert_path(&mut concrete_keys, &mut context_keys);
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
        let key = Box::new(DefaultContextKey::<String>::new(Self::PATH));

        context_keys.insert(Cow::Borrowed(&Self::PATH), key.clone());
        concrete_keys.insert(Cow::Borrowed(&Self::PATH), key.clone());
    }

    fn insert_event_log(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<XesEventLogImpl>::new(Self::EVENT_LOG));

        context_keys.insert(Cow::Borrowed(&Self::EVENT_LOG), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::EVENT_LOG), key);
    }

    fn insert_activities(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<Activities>::new(Self::ACTIVITIES));

        context_keys.insert(Cow::Borrowed(&Self::ACTIVITIES), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::ACTIVITIES), key);
    }

    fn insert_repeat_sets(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<RepeatSets>::new(Self::REPEAT_SETS));

        context_keys.insert(Cow::Borrowed(&Self::REPEAT_SETS), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::REPEAT_SETS), key);
    }

    fn insert_trace_activities(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<TracesActivities>::new(Self::TRACE_ACTIVITIES));

        context_keys.insert(Cow::Borrowed(&Self::TRACE_ACTIVITIES), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::TRACE_ACTIVITIES), key);
    }

    fn insert_patterns(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<Patterns>::new(Self::PATTERNS));

        context_keys.insert(Cow::Borrowed(&Self::PATTERNS), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::PATTERNS), key);
    }

    fn insert_petri_net(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<PetriNet>::new(Self::PETRI_NET));

        context_keys.insert(Cow::Borrowed(&Self::PETRI_NET), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::PETRI_NET), key);
    }

    fn insert_activities_to_logs(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<ActivitiesToLogs>::new(Self::ACTIVITIES_TO_LOGS));

        context_keys.insert(Cow::Borrowed(&Self::ACTIVITIES_TO_LOGS), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::ACTIVITIES_TO_LOGS), key);
    }

    fn insert_activity_name(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = Box::new(DefaultContextKey::<String>::new(Self::ACTIVITY_NAME));

        context_keys.insert(Cow::Borrowed(&Self::ACTIVITY_NAME), key.clone());
        concrete_keys.insert(Cow::Borrowed(Self::ACTIVITY_NAME), key);
    }

    fn insert_hashes_event_log(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = DefaultContextKey::<Vec<Vec<u64>>>::new_with_factory(
            Self::HASHES_EVENT_LOG.to_string(),
            Rc::new(Box::new(|pipeline_context, keys| {
                match pipeline_context.get_concrete(keys.event_log()) {
                    None => None,
                    Some(log) => Some(log.to_hashes_event_log::<NameEventHasher>()),
                }
            })),
        );

        context_keys.insert(Cow::Borrowed(&Self::HASHES_EVENT_LOG), Box::new(key.clone()));
        concrete_keys.insert(Cow::Borrowed(Self::HASHES_EVENT_LOG), Box::new(key.clone()));
    }

    fn insert_names_event_log(
        concrete_keys: &mut HashMap<Cow<'static, str>, Box<dyn Any>>,
        context_keys: &mut HashMap<Cow<'static, str>, Box<dyn ContextKey>>,
    ) {
        let key = DefaultContextKey::<Vec<Vec<String>>>::new_with_factory(
            Self::NAMES_EVENT_LOG.to_string(),
            Rc::new(Box::new(|pipeline_context, keys| {
                match pipeline_context.get_concrete(keys.event_log()) {
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

        context_keys.insert(Cow::Borrowed(&Self::NAMES_EVENT_LOG), Box::new(key.clone()));
        concrete_keys.insert(Cow::Borrowed(Self::NAMES_EVENT_LOG), Box::new(key.clone()));
    }
}