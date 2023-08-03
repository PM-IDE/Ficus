use std::hash::{Hash, Hasher};

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

impl Hash for PipelineType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub struct Types {
    event_log: PipelineType,
    activities: PipelineType,
    repeat_sets: PipelineType,
    trace_activities: PipelineType,
    patterns: PipelineType,
    petri_net: PipelineType,
    event_class_tree: PipelineType,
    activities_to_logs: PipelineType,
    activity_name: PipelineType,
}

impl Types {
    pub fn new() -> Self {
        Self {
            event_log: PipelineType::new("event_log"),
            activities: PipelineType::new("activities"),
            repeat_sets: PipelineType::new("repeat_sets"),
            trace_activities: PipelineType::new("trace_activities"),
            patterns: PipelineType::new("patterns"),
            petri_net: PipelineType::new("petri_net"),
            event_class_tree: PipelineType::new("event_class_tree"),
            activities_to_logs: PipelineType::new("activities_to_logs"),
            activity_name: PipelineType::new("activity_name"),
        }
    }

    pub fn event_log(&self) -> &PipelineType {
        &self.event_log
    }

    pub fn activities(&self) -> &PipelineType {
        &self.activities
    }

    pub fn repeat_sets(&self) -> &PipelineType {
        &self.repeat_sets
    }

    pub fn trace_activities(&self) -> &PipelineType {
        &self.trace_activities
    }

    pub fn patterns(&self) -> &PipelineType {
        &self.patterns
    }

    pub fn petri_net(&self) -> &PipelineType {
        &self.petri_net
    }

    pub fn event_class_tree(&self) -> &PipelineType {
        &self.event_class_tree
    }

    pub fn activities_to_logs(&self) -> &PipelineType {
        &self.activities_to_logs
    }

    pub fn activity_name(&self) -> &PipelineType {
        &self.activity_name
    }
}
