use std::{cell::RefCell, collections::HashMap, rc::Rc};

use linfa::{
    metrics::SilhouetteScore,
    traits::{Fit, Predict},
};
use linfa_clustering::KMeans;

use crate::{
    event_log::core::event_log::EventLog, features::analysis::patterns::repeat_sets::ActivityNode, pipelines::aliases::TracesActivities,
    utils::dataset::dataset::LabeledDataset,
};

use super::{
    common::{create_dataset, merge_activities, transform_to_ficus_dataset, ClusteredDataset, CosineDistance, MyDataset},
    params::ClusteringCommonParams,
};

pub fn clusterize_activities_k_means<TLog: EventLog>(
    params: &mut ClusteringCommonParams<TLog>,
    clusters_count: usize,
    iterations_count: usize,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(params) {
        let model = create_k_means_model(clusters_count, iterations_count as u64, params.tolerance, &dataset);

        let clustered_dataset = model.predict(dataset.clone());
        merge_activities(
            params.log,
            params.traces_activities,
            &processed.iter().map(|x| x.0.clone()).collect(),
            &clustered_dataset.targets.map(|x| Some(*x)),
        );

        Some(create_labeled_dataset_from_k_means(&dataset, &clustered_dataset, &processed, classes_names))
    } else {
        None
    }
}

fn create_labeled_dataset_from_k_means(
    dataset: &MyDataset,
    clustered_dataset: &ClusteredDataset,
    processed: &Vec<(Rc<RefCell<ActivityNode>>, HashMap<u64, usize>)>,
    classes_names: Vec<String>,
) -> LabeledDataset {
    let ficus_dataset = transform_to_ficus_dataset(dataset, processed, classes_names);
    LabeledDataset::new(ficus_dataset, clustered_dataset.targets.clone().into_raw_vec())
}

fn create_k_means_model(clusters_count: usize, iterations_count: u64, tolerance: f64, dataset: &MyDataset) -> KMeans<f64, CosineDistance> {
    KMeans::params_with(clusters_count, rand::thread_rng(), CosineDistance {})
        .max_n_iterations(iterations_count)
        .tolerance(tolerance)
        .fit(&dataset)
        .expect("KMeans fitted")
}

pub fn clusterize_activities_k_means_grid_search<TLog: EventLog>(
    params: &mut ClusteringCommonParams<TLog>,
    iterations_count: usize,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(params) {
        let mut best_metric = -1f64;
        let mut best_labels = None;

        for clusters_count in 2..processed.len() {
            let model = create_k_means_model(clusters_count, iterations_count as u64, params.tolerance, &dataset);

            let clustered_dataset = model.predict(dataset.clone());
            let score = match clustered_dataset.silhouette_score() {
                Ok(score) => score,
                Err(_) => return None,
            };

            if score > best_metric {
                best_labels = Some(clustered_dataset.targets);
                best_metric = score;
            }
        }

        if let Some(best_labels) = best_labels.as_ref() {
            merge_activities(
                params.log,
                params.traces_activities,
                &processed.iter().map(|x| x.0.clone()).collect(),
                &best_labels.map(|x| Some(*x)),
            );

            let ficus_dataset = transform_to_ficus_dataset(&dataset, &processed, classes_names);
            Some(LabeledDataset::new(ficus_dataset, best_labels.clone().into_raw_vec()))
        } else {
            None
        }
    } else {
        None
    }
}