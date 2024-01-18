use std::str::FromStr;

use crate::{features::clustering::common::{CommonVisualizationParams, FicusDistance}, event_log::core::event_log::EventLog, pipelines::aliases::TracesActivities};



#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ActivityRepresentationSource {
    EventClasses,
    SubTraces,
    SubTracesUnderlyingEvents,
}

impl FromStr for ActivityRepresentationSource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EventClasses" => Ok(Self::EventClasses),
            "SubTraces" => Ok(Self::SubTraces),
            "SubTracesUnderlyingEvents" => Ok(Self::SubTracesUnderlyingEvents),
            _ => Err(()),
        }
    }
}

pub struct ActivitiesVisualizationParams<'a, TLog> where TLog: EventLog {
    pub common_vis_params: CommonVisualizationParams<'a, TLog>,
    pub traces_activities: &'a mut TracesActivities,
    pub activity_level: usize,
    pub class_extractor: Option<String>,
    pub activities_repr_source: ActivityRepresentationSource,
}

pub struct ActivitiesClusteringParams<'a, TLog>
where
    TLog: EventLog,
{
    pub vis_params: ActivitiesVisualizationParams<'a, TLog>,
    pub tolerance: f64,
    pub distance: FicusDistance,
}
