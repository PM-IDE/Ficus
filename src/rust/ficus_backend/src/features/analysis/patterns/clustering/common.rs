use std::{rc::Rc, cell::RefCell, collections::{HashSet, HashMap}};

use linfa::DatasetBase;
use linfa_nn::distance::Distance;
use ndarray::{ArrayBase, OwnedRepr, Dim, Array1, Array2, Dimension, ArrayView};

use crate::{features::analysis::patterns::{repeat_sets::ActivityNode, activity_instances::ActivityInTraceInfo}, event_log::core::{event_log::EventLog, event::{event_hasher::RegexEventHasher, event::Event}, trace::trace::Trace}, pipelines::aliases::TracesActivities, utils::dataset::dataset::FicusDataset};

pub(super) type ActivityNodeWithCoords = Vec<(Rc<RefCell<ActivityNode>>, Vec<u64>)>;
pub(super) type MyDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, Array1<()>>;
pub(super) type ClusteredDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<usize>, Dim<[usize; 1]>>>;

pub(super) fn create_dataset(
    log: &impl EventLog,
    traces_activities: &TracesActivities,
    activity_level: usize,
    class_extractor: Option<String>,
) -> Option<(MyDataset, ActivityNodeWithCoords, Vec<String>)> {
    let mut all_event_classes = HashSet::new();
    let mut processed = HashMap::new();
    let regex_hasher = match class_extractor.as_ref() {
        Some(class_extractor) => Some(RegexEventHasher::new(class_extractor).ok().unwrap()),
        None => None,
    };

    for trace_activities in traces_activities.iter() {
        for activity in trace_activities {
            if processed.contains_key(&activity.node.borrow().name) {
                continue;
            }

            if activity.node.borrow().level != activity_level {
                continue;
            }

            let activity_event_classes = if let Some(regex_hasher) = regex_hasher.as_ref() {
                if let Some(repeat_set) = activity.node.borrow().repeat_set.as_ref() {
                    let trace = log.traces().get(repeat_set.trace_index).unwrap();
                    let trace = trace.borrow();
                    let events = trace.events();
                    let array = &repeat_set.sub_array;

                    let mut abstracted_event_classes = HashSet::new();
                    for event in &events[array.start_index..(array.start_index + array.length)] {
                        abstracted_event_classes.insert(regex_hasher.hash_name(event.borrow().name()));
                    }

                    let abstracted_event_classes = abstracted_event_classes.into_iter().collect::<Vec<u64>>();
                    for class in &abstracted_event_classes {
                        all_event_classes.insert(*class);
                    }

                    abstracted_event_classes
                } else {
                    panic!();
                }
            } else {
                for event_class in &activity.node.borrow().event_classes {
                    all_event_classes.insert(event_class.to_owned());
                }

                activity.node.borrow().event_classes.iter().map(|x| *x).collect()
            };

            processed.insert(
                activity.node.borrow().name.to_owned(),
                (activity.node.clone(), activity_event_classes),
            );
        }
    }

    let all_event_classes = all_event_classes.into_iter().collect::<Vec<u64>>();
    let mut processed = processed.iter().map(|x| x.1.clone()).collect::<ActivityNodeWithCoords>();
    processed.sort_by(|first, second| first.0.borrow().name.cmp(&second.0.borrow().name));

    let mut vector = vec![];
    for activity in &processed {
        for i in 0..all_event_classes.len() {
            vector.push(if activity.1.contains(&all_event_classes[i]) { 1.0 } else { 0.0 });
        }
    }

    let shape = (processed.len(), all_event_classes.len());

    let array = match Array2::from_shape_vec(shape, vector) {
        Ok(score) => score,
        Err(_) => return None,
    };

    Some((DatasetBase::from(array), processed, all_event_classes.iter().map(|x| x.to_string()).collect()))
}

pub(super) fn merge_activities(
    log: &impl EventLog,
    traces_activities: &mut TracesActivities,
    processed: &Vec<Rc<RefCell<ActivityNode>>>,
    labels: &Array1<Option<usize>>,
) {
    let mut activity_names_to_clusters = HashMap::new();
    let mut clusters_to_activities: HashMap<usize, Vec<Rc<RefCell<ActivityNode>>>> = HashMap::new();

    for (activity, label) in processed.iter().zip(labels.iter()) {
        if let Some(label) = label {
            activity_names_to_clusters.insert(activity.borrow().name.to_owned(), *label);

            if let Some(cluster_activities) = clusters_to_activities.get_mut(label) {
                cluster_activities.push(activity.clone());
            } else {
                clusters_to_activities.insert(*label, vec![activity.clone()]);
            }
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
            name: new_activity_name,
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
                    length: current_activity_in_trace.length,
                };
            }
        }
    }
}

#[derive(Clone)]
pub(super) struct CosineDistance {}

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

pub fn create_traces_activities_dataset(
    log: &impl EventLog,
    traces_activities: &mut TracesActivities,
    activity_level: usize,
    class_extractor: Option<String>
) -> Option<FicusDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(log, traces_activities, activity_level, class_extractor) {
        Some(transform_to_ficus_dataset(&dataset, &processed, classes_names))
    } else {
        None
    }
}

pub(super) fn transform_to_ficus_dataset(
    dataset: &MyDataset, 
    processed: &Vec<(Rc<RefCell<ActivityNode>>, Vec<u64>)>,
    classes_names: Vec<String>
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

    let row_names = processed.iter().map(|x| x.0.borrow().name.to_owned()).collect();
    FicusDataset::new(matrix, classes_names, row_names)
}