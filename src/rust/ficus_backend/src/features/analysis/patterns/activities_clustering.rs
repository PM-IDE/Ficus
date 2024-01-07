use linfa::prelude::Predict;

use std::{collections::{HashSet, HashMap}, rc::Rc, cell::RefCell};

use linfa::{traits::Fit, DatasetBase, Dataset};
use linfa_clustering::KMeans;
use linfa::metrics::SilhouetteScore;
use linfa_nn::distance::Distance;
use ndarray::{Array2, ArrayView, Dimension, ArrayBase, OwnedRepr, Array1, Dim};
use crate::{pipelines::aliases::TracesActivities, features::analysis::patterns::repeat_sets::ActivityNode, event_log::core::{event_log::EventLog, trace::trace::Trace, event::event::Event}};

use super::activity_instances::ActivityInTraceInfo;

pub fn clusterize_activities_k_means(
    log: &impl EventLog, 
    traces_activities: &mut TracesActivities,
    activity_level: usize,
    clusters_count: usize,
    iterations_count: usize,
    tolerance: f64,
) {
    if let Some((dataset, processed)) = create_dataset_from_traces_activities(traces_activities, activity_level) {
        let model = create_k_means_model(clusters_count, iterations_count as u64, tolerance, &dataset);

        let clustered_dataset = model.predict(dataset);
        merge_activities(log, traces_activities, &processed, &clustered_dataset.targets);
    }
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
    tolerance: f64
) {
    if let Some((dataset, processed)) = create_dataset_from_traces_activities(traces_activities, activity_level) {
        let mut best_metric = -1f64;
        let mut best_labels = None;

        for clusters_count in 2..processed.len() {
            let model = create_k_means_model(clusters_count, iterations_count as u64, tolerance, &dataset);

            let clustered_dataset = model.predict(dataset.clone());
            let score = match clustered_dataset.silhouette_score() {
                Ok(score) => score,
                Err(_) => return
            };

            if score > best_metric {
                best_labels = Some(clustered_dataset.targets);
                best_metric = score;
            }
        }

        if let Some(best_labels) = best_labels.as_ref() {
            merge_activities(log, traces_activities, &processed, best_labels)
        }
    }
}

type MyDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, Array1<()>>;
fn create_dataset_from_traces_activities(
    traces_activities: &TracesActivities,
    activity_level: usize
) -> Option<(MyDataset, Vec<Rc<RefCell<ActivityNode>>>)> {
    let mut all_event_classes = HashSet::new();
    let mut processed = HashMap::new();

    for trace_activities in traces_activities.iter() {
        for activity in trace_activities {
            if processed.contains_key(&activity.node.borrow().name) {
                continue;
            }

            if activity.node.borrow().level != activity_level {
                continue
            }

            for event_class in &activity.node.borrow().event_classes {
                all_event_classes.insert(event_class.to_owned());
            }

            processed.insert(activity.node.borrow().name.to_owned(), activity.node.clone());
        }
    }

    let all_event_classes = all_event_classes.into_iter().collect::<Vec<u64>>();
    let mut processed = processed.iter().map(|x| x.1.clone()).collect::<Vec<Rc<RefCell<ActivityNode>>>>();
    processed.sort_by(|first, second| first.borrow().name.cmp(&second.borrow().name));

    let mut vector = vec![];
    for activity in &processed {
        for i in 0..all_event_classes.len() {
            vector.push(if activity.borrow().event_classes.contains(&all_event_classes[i]) {
                1.0
            } else {
                0.0
            });
        }
    }

    let shape = (processed.len(), all_event_classes.len());

    let array = match Array2::from_shape_vec(shape, vector) {
        Ok(score) => score,
        Err(_) => return None
    };

    Some((DatasetBase::from(array), processed))
}

fn merge_activities(
    log: &impl EventLog,
    traces_activities: &mut TracesActivities, 
    processed: &Vec<Rc<RefCell<ActivityNode>>>, 
    labels: &Array1<usize>
) {
    let mut activity_names_to_clusters = HashMap::new();
    let mut clusters_to_activities: HashMap<usize, Vec<Rc<RefCell<ActivityNode>>>> = HashMap::new();

    for (activity, label) in processed.iter().zip(labels.iter()) {
        activity_names_to_clusters.insert(activity.borrow().name.to_owned(), *label);

        if let Some(cluster_activities) = clusters_to_activities.get_mut(label) {
            cluster_activities.push(activity.clone());
        } else {
            clusters_to_activities.insert(*label, vec![activity.clone()]);
        }
    }

    let mut new_activity_name_parts = HashSet::new();
    let mut new_cluster_activities = HashMap::new();

    for (cluster, cluster_activities) in &clusters_to_activities {
        if cluster_activities.len() < 2 {
            continue;
        }

        let mut new_event_classes_set = HashSet::new();

        for activity in cluster_activities {
            for event_class in &activity.borrow().event_classes {
                new_event_classes_set.insert(*event_class);
            }

            if let Some(repeat_set) = activity.borrow().repeat_set.as_ref() {
                let trace = log.traces().get(repeat_set.trace_index).unwrap();
                let events = trace.borrow();
                let events = events.events();
                let sub_array = repeat_set.sub_array;
                for event in &events[sub_array.start_index..(sub_array.start_index + sub_array.length)] {
                    new_activity_name_parts.insert(event.borrow().name().to_owned());
                }
            }
        }

        let mut new_activity_name_parts = new_activity_name_parts.iter().map(|x| x.to_owned()).collect::<Vec<String>>();
        new_activity_name_parts.sort_by(|first, second| first.cmp(second));

        let mut new_activity_name = String::new();
        new_activity_name.push_str("CLUSTER_");

        for name in new_activity_name_parts {
            new_activity_name.push_str(name.as_str());
            new_activity_name.push_str("::");
        }

        let new_node = ActivityNode {
            repeat_set: None,
            event_classes: new_event_classes_set,
            children: vec![],
            level: cluster_activities[0].borrow().level,
            name: new_activity_name
        };

        new_cluster_activities.insert(*cluster, Rc::new(RefCell::new(new_node)));
    }

    for trace_activities in traces_activities {
        for i in 0..trace_activities.len() {
            let activity = trace_activities.get(i).unwrap();
            if !activity_names_to_clusters.contains_key(&activity.node.borrow().name) {
                continue;
            }

            let cluster_label = activity_names_to_clusters.get(&activity.node.borrow().name).unwrap();
            if let Some(new_activity_node) = new_cluster_activities.get(cluster_label) {
                let current_activity_in_trace = trace_activities.get(i).unwrap();

                *trace_activities.get_mut(i).unwrap() = ActivityInTraceInfo {
                    node: new_activity_node.clone(),
                    start_pos: current_activity_in_trace.start_pos,
                    length: current_activity_in_trace.length
                };
            }
        }
    }
}

#[derive(Clone)]
struct CosineDistance {}

impl Distance<f64> for CosineDistance {
    fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
        let mut sum = 0.0;
        let mut a_square = 0.0;
        let mut b_square = 0.0;

        for (a, b) in a.iter().zip(b.iter()) {
            sum += a * b;
            a_square += a * a;
            b_square += b * b;
        }

        1.0 - sum / (a_square.sqrt() * b_square.sqrt())
    }
}