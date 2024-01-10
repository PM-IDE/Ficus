use std::str::FromStr;

use crate::{event_log::core::event_log::EventLog, pipelines::aliases::TracesActivities};

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

pub struct ClusteringCommonParams<'a, TLog>
where
    TLog: EventLog,
{
    pub log: &'a TLog,
    pub traces_activities: &'a mut TracesActivities,
    pub activity_level: usize,
    pub tolerance: f64,
    pub class_extractor: Option<String>,
    pub activities_repr_source: ActivityRepresentationSource,
}
