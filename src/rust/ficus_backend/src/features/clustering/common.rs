use std::str::FromStr;

use linfa::DatasetBase;
use linfa_nn::distance::{L1Dist, L2Dist, Distance};
use ndarray::{ArrayBase, OwnedRepr, Dim, Array1, Dimension, ArrayView};

use crate::{event_log::core::event_log::EventLog, utils::{colors::{ColorsHolder, Color}, dataset::dataset::FicusDataset}};

use super::activities::distance::{CosineDistance, LevenshteinDistance};


pub(super) type MyDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, Array1<()>>;
pub(super) type ClusteredDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<usize>, Dim<[usize; 1]>>>;


#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FicusDistance {
    Cosine,
    L1,
    L2,
    Levenshtein
}

impl FromStr for FicusDistance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cosine" => Ok(Self::Cosine),
            "L1" => Ok(Self::L1),
            "L2" => Ok(Self::L2),
            "Levenshtein" => Ok(Self::Levenshtein),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub enum DistanceWrapper {
    Cosine(CosineDistance),
    L1(L1Dist),
    L2(L2Dist),
    Levenshtein(LevenshteinDistance)
}

impl DistanceWrapper {
    pub fn new(ficus_distance: FicusDistance) -> DistanceWrapper {
        match ficus_distance {
            FicusDistance::Cosine => DistanceWrapper::Cosine(CosineDistance {}),
            FicusDistance::L1 => DistanceWrapper::L1(L1Dist {}),
            FicusDistance::L2 => DistanceWrapper::L2(L2Dist {}),
            FicusDistance::Levenshtein => DistanceWrapper::Levenshtein(LevenshteinDistance {}),
        }
    }
}

impl Distance<f64> for DistanceWrapper {
    fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
        match self {
            DistanceWrapper::Cosine(d) => d.distance(a, b),
            DistanceWrapper::L1(d) => d.distance(a, b),
            DistanceWrapper::L2(d) => d.distance(a, b),
            DistanceWrapper::Levenshtein(d) => d.distance(a, b),
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

pub struct CommonVisualizationParams<'a, TLog> where TLog: EventLog {
    pub log: &'a TLog,
    pub colors_holder: &'a mut ColorsHolder
}

pub fn transform_to_ficus_dataset(
    dataset: &MyDataset,
    processed: Vec<String>,
    classes_names: Vec<String>,
) -> FicusDataset {
    let rows_count = dataset.records().shape()[0];
    let cols_count = dataset.records().shape()[1];

    let mut matrix = vec![];
    for i in 0..rows_count {
        let mut vec = vec![];
        for j in 0..cols_count {
            vec.push(*dataset.records.get([i, j]).unwrap());
        }

        matrix.push(vec);
    }

    FicusDataset::new(matrix, classes_names, processed)
}

pub(super) fn create_colors_vector(labels: &Vec<usize>, colors_holder: &mut ColorsHolder) -> Vec<Color> {
    labels.iter().map(|x| colors_holder.get_or_create(&create_cluster_name(*x))).collect()
}

pub fn scale_raw_dataset_min_max(vector: &mut Vec<f64>, objects_count: usize, features_count: usize) {
    for i in 0..features_count {
        let mut max = f64::MIN;
        let mut min = f64::MAX;

        for j in 0..objects_count {
            let index = i + j * features_count;
            max = max.max(vector[index]);
            min = min.min(vector[index]);
        }

        for j in 0..objects_count {
            let index = i + j * features_count;
            vector[index] = (vector[index] - min) / (max - min);
        }
    }
}

pub fn create_cluster_name(cluster_index: usize) -> String {
    format!("CLUSTER_{}", cluster_index)
}
