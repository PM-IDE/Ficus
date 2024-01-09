use std::{cell::RefCell, rc::Rc};

use linfa::{traits::{Fit, Predict}, metrics::SilhouetteScore};
use linfa_clustering::KMeans;

use crate::{utils::dataset::dataset::LabeledDataset, event_log::core::event_log::EventLog, pipelines::aliases::TracesActivities, features::analysis::patterns::repeat_sets::ActivityNode};

use super::common::{MyDataset, ClusteredDataset, transform_to_ficus_dataset, CosineDistance, create_dataset, merge_activities};

pub fn clusterize_activities_k_means(
    log: &impl EventLog,
    traces_activities: &mut TracesActivities,
    activity_level: usize,
    clusters_count: usize,
    iterations_count: usize,
    tolerance: f64,
    class_extractor: Option<String>,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(log, traces_activities, activity_level, class_extractor) {
        let model = create_k_means_model(clusters_count, iterations_count as u64, tolerance, &dataset);

        let clustered_dataset = model.predict(dataset.clone());
        merge_activities(
            log,
            traces_activities,
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
    processed: &Vec<(Rc<RefCell<ActivityNode>>, Vec<u64>)>, 
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

pub fn clusterize_activities_k_means_grid_search(
    log: &impl EventLog,
    traces_activities: &mut TracesActivities,
    activity_level: usize,
    iterations_count: usize,
    tolerance: f64,
    class_extractor: Option<String>,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(log, traces_activities, activity_level, class_extractor) {
        let mut best_metric = -1f64;
        let mut best_labels = None;

        for clusters_count in 2..processed.len() {
            let model = create_k_means_model(clusters_count, iterations_count as u64, tolerance, &dataset);

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
                log,
                traces_activities,
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