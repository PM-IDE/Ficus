use std::{
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

pub struct PipelineType<T> {
    key: Key<T>,
}

impl<T> PipelineType<T>
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

impl<T> PartialEq for PipelineType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for PipelineType<T> {}

impl<T> Hash for PipelineType<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

pub struct Types {
    path: Rc<Box<PipelineType<String>>>,
    event_log: Rc<Box<PipelineType<XesEventLogImpl>>>,
    activities: Rc<Box<PipelineType<Vec<Rc<RefCell<ActivityNode>>>>>>,
    repeat_sets: Rc<Box<PipelineType<Vec<SubArrayWithTraceIndex>>>>,
    trace_activities: Rc<Box<PipelineType<Vec<Vec<ActivityInTraceInfo>>>>>,
    patterns: Rc<Box<PipelineType<Vec<Vec<SubArrayInTraceInfo>>>>>,
    petri_net: Rc<Box<PipelineType<PetriNet>>>,
    activities_to_logs: Rc<Box<PipelineType<HashMap<String, XesEventLogImpl>>>>,
    activity_name: Rc<Box<PipelineType<String>>>,
}

unsafe impl Sync for Types {}
unsafe impl Send for Types {}

impl Types {
    pub fn new() -> Self {
        Self {
            path: Self::allocate_type(PipelineType::new("path")),
            event_log: Self::allocate_type(PipelineType::new("event_log")),
            activities: Self::allocate_type(PipelineType::new("activities")),
            repeat_sets: Self::allocate_type(PipelineType::new("repeat_sets")),
            trace_activities: Self::allocate_type(PipelineType::new("trace_activities")),
            patterns: Self::allocate_type(PipelineType::new("patterns")),
            petri_net: Self::allocate_type(PipelineType::new("petri_net")),
            activities_to_logs: Self::allocate_type(PipelineType::new("activities_to_logs")),
            activity_name: Self::allocate_type(PipelineType::new("activity_name")),
        }
    }

    fn allocate_type<T>(pipeline_type: PipelineType<T>) -> Rc<Box<PipelineType<T>>> {
        Rc::new(Box::new(pipeline_type))
    }

    pub fn path(&self) -> Rc<Box<PipelineType<String>>> {
        Rc::clone(&self.path)
    }

    pub fn event_log(&self) -> Rc<Box<PipelineType<XesEventLogImpl>>> {
        Rc::clone(&self.event_log)
    }

    pub fn activities(&self) -> Rc<Box<PipelineType<Vec<Rc<RefCell<ActivityNode>>>>>> {
        Rc::clone(&self.activities)
    }

    pub fn repeat_sets(&self) -> Rc<Box<PipelineType<Vec<SubArrayWithTraceIndex>>>> {
        Rc::clone(&self.repeat_sets)
    }

    pub fn trace_activities(&self) -> Rc<Box<PipelineType<Vec<Vec<ActivityInTraceInfo>>>>> {
        Rc::clone(&self.trace_activities)
    }

    pub fn patterns(&self) -> Rc<Box<PipelineType<Vec<Vec<SubArrayInTraceInfo>>>>> {
        Rc::clone(&self.patterns)
    }

    pub fn petri_net(&self) -> Rc<Box<PipelineType<PetriNet>>> {
        Rc::clone(&self.petri_net)
    }

    pub fn activities_to_logs(&self) -> Rc<Box<PipelineType<HashMap<String, XesEventLogImpl>>>> {
        Rc::clone(&self.activities_to_logs)
    }

    pub fn activity_name(&self) -> Rc<Box<PipelineType<String>>> {
        Rc::clone(&self.activity_name)
    }
}
