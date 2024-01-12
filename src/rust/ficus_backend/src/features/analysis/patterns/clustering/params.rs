use std::str::FromStr;

use linfa_nn::distance::{Distance, L1Dist, L2Dist};
use ndarray::{ArrayView, Dimension};

use crate::{event_log::core::event_log::EventLog, pipelines::aliases::TracesActivities};

use super::common::CosineDistance;

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FicusDistance {
    Cosine,
    L1,
    L2,
}

impl FromStr for FicusDistance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cosine" => Ok(Self::Cosine),
            "L1" => Ok(Self::L1),
            "L2" => Ok(Self::L2),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub enum DistanceWrapper {
    Cosine(CosineDistance),
    L1(L1Dist),
    L2(L2Dist),
}

impl DistanceWrapper {
    pub fn new(ficus_distance: FicusDistance) -> DistanceWrapper {
        match ficus_distance {
            FicusDistance::Cosine => DistanceWrapper::Cosine(CosineDistance {}),
            FicusDistance::L1 => DistanceWrapper::L1(L1Dist {}),
            FicusDistance::L2 => DistanceWrapper::L2(L2Dist {}),
        }
    }
}

impl Distance<f64> for DistanceWrapper {
    fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
        match self {
            DistanceWrapper::Cosine(d) => d.distance(a, b),
            DistanceWrapper::L1(d) => d.distance(a, b),
            DistanceWrapper::L2(d) => d.distance(a, b),
        }
    }

    fn rdistance<D: ndarray::prelude::Dimension>(
        &self,
        a: ndarray::prelude::ArrayView<f64, D>,
        b: ndarray::prelude::ArrayView<f64, D>,
    ) -> f64 {
        self.distance(a, b)
    }

    fn rdist_to_dist(&self, rdist: f64) -> f64 {
        rdist
    }

    fn dist_to_rdist(&self, dist: f64) -> f64 {
        dist
    }
}

pub struct ActivitiesVisualizationParams<'a, TLog> where TLog: EventLog {
    pub log: &'a TLog,
    pub traces_activities: &'a mut TracesActivities,
    pub activity_level: usize,
    pub class_extractor: Option<String>,
    pub activities_repr_source: ActivityRepresentationSource,
}

pub struct ClusteringCommonParams<'a, TLog>
where
    TLog: EventLog,
{
    pub vis_params: ActivitiesVisualizationParams<'a, TLog>,
    pub tolerance: f64,
    pub distance: FicusDistance,
}
