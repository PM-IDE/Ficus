use crate::{event_log::core::event_log::EventLog, pipelines::aliases::TracesActivities};

pub struct ClusteringCommonParams<'a, TLog>
where
    TLog: EventLog,
{
    pub log: &'a TLog,
    pub traces_activities: &'a mut TracesActivities,
    pub activity_level: usize,
    pub tolerance: f64,
    pub class_extractor: Option<String>,
    pub obtain_repr_from_traces: bool,
}
