use crate::{
    event_log::core::event_log::EventLog,
    features::clustering::common::{CommonVisualizationParams, FicusDistance},
};

pub struct TracesClusteringParams<'a, TLog>
where
    TLog: EventLog,
{
    pub vis_params: CommonVisualizationParams<'a, TLog>,
    pub tolerance: f64,
    pub distance: FicusDistance,
}
