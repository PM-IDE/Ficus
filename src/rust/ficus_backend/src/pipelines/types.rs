use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

pub struct PipelineType {
    name: String,
}

impl PipelineType {
    pub fn new(type_name: &str) -> Self {
        Self {
            name: type_name.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for PipelineType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PipelineType {}

impl Hash for PipelineType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub struct Types {
    path: Rc<Box<PipelineType>>,
    event_log: Rc<Box<PipelineType>>,
    activities: Rc<Box<PipelineType>>,
    repeat_sets: Rc<Box<PipelineType>>,
    trace_activities: Rc<Box<PipelineType>>,
    patterns: Rc<Box<PipelineType>>,
    petri_net: Rc<Box<PipelineType>>,
    event_class_tree: Rc<Box<PipelineType>>,
    activities_to_logs: Rc<Box<PipelineType>>,
    activity_name: Rc<Box<PipelineType>>,
}

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
            event_class_tree: Self::allocate_type(PipelineType::new("event_class_tree")),
            activities_to_logs: Self::allocate_type(PipelineType::new("activities_to_logs")),
            activity_name: Self::allocate_type(PipelineType::new("activity_name")),
        }
    }

    fn allocate_type(pipeline_type: PipelineType) -> Rc<Box<PipelineType>> {
        Rc::new(Box::new(pipeline_type))
    }

    pub fn path(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.path)
    }

    pub fn event_log(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.event_log)
    }

    pub fn activities(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.activities)
    }

    pub fn repeat_sets(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.repeat_sets)
    }

    pub fn trace_activities(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.trace_activities)
    }

    pub fn patterns(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.patterns)
    }

    pub fn petri_net(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.petri_net)
    }

    pub fn event_class_tree(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.event_class_tree)
    }

    pub fn activities_to_logs(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.activities_to_logs)
    }

    pub fn activity_name(&self) -> Rc<Box<PipelineType>> {
        Rc::clone(&self.activity_name)
    }
}
