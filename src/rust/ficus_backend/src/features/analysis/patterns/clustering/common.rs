use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    vec,
};

use linfa::DatasetBase;
use linfa_nn::distance::Distance;
use ndarray::{Array1, Array2, ArrayBase, ArrayView, Dim, Dimension, OwnedRepr};

use crate::{
    event_log::core::{
        event::{
            event::Event,
            event_hasher::{default_class_extractor_name, RegexEventHasher},
        },
        event_log::EventLog,
        trace::trace::Trace,
    },
    features::analysis::patterns::{
        activity_instances::{create_vector_of_underlying_events, ActivityInTraceInfo},
        repeat_sets::ActivityNode,
    },
    pipelines::aliases::TracesActivities,
    utils::dataset::dataset::FicusDataset,
};

use super::params::{ActivityRepresentationSource, ClusteringCommonParams, ActivitiesVisualizationParams};

pub(super) type ActivityNodeWithCoords = Vec<(Rc<RefCell<ActivityNode>>, HashMap<u64, usize>)>;
pub(super) type MyDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, Array1<()>>;
pub(super) type ClusteredDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<usize>, Dim<[usize; 1]>>>;

pub fn create_dataset<TLog: EventLog>(
    params: &ActivitiesVisualizationParams<TLog>,
) -> Option<(MyDataset, ActivityNodeWithCoords, Vec<String>)> {
    match params.activities_repr_source {
        ActivityRepresentationSource::EventClasses => create_dataset_from_activities_classes(params),
        ActivityRepresentationSource::SubTraces => create_dataset_from_activities_traces(params),
        ActivityRepresentationSource::SubTracesUnderlyingEvents => create_dataset_from_activities_traces_underlying_events(params),
    }
}

pub(super) fn create_dataset_from_activities_traces_underlying_events<TLog: EventLog>(
    params: &ActivitiesVisualizationParams<TLog>,
) -> Option<(MyDataset, ActivityNodeWithCoords, Vec<String>)> {
    create_dataset_internal(
        params.traces_activities,
        params.class_extractor.clone(),
        |traces_activities, regex_hasher, all_event_classes| {
            create_activities_repr_from_subtraces(
                traces_activities,
                regex_hasher,
                all_event_classes,
                params,
                |events, map, all_event_classes| {
                    let mut sub_trace_events = vec![];
                    for event in events {
                        for underlying_event in create_vector_of_underlying_events::<TLog>(event) {
                            sub_trace_events.push(underlying_event);
                        }
                    }

                    update_event_classes::<TLog>(sub_trace_events.as_slice(), regex_hasher, all_event_classes, map)
                },
            )
        },
    )
}

pub(super) fn create_dataset_from_activities_traces<TLog: EventLog>(
    params: &ActivitiesVisualizationParams<TLog>,
) -> Option<(MyDataset, ActivityNodeWithCoords, Vec<String>)> {
    create_dataset_internal(
        params.traces_activities,
        params.class_extractor.clone(),
        |traces_activities, regex_hasher, all_event_classes| {
            create_activities_repr_from_subtraces(
                traces_activities,
                regex_hasher,
                all_event_classes,
                params,
                |events, map, all_event_classes| update_event_classes::<TLog>(events, regex_hasher, all_event_classes, map),
            )
        },
    )
}

fn update_event_classes<TLog: EventLog>(
    events: &[Rc<RefCell<<TLog as EventLog>::TEvent>>],
    regex_hasher: Option<&RegexEventHasher>,
    all_event_classes: &mut HashSet<u64>,
    map: &mut HashMap<u64, usize>,
) {
    for event in events {
        let hash = if let Some(hasher) = regex_hasher {
            hasher.hash_name(event.borrow().name())
        } else {
            default_class_extractor_name(event.borrow().name())
        };

        all_event_classes.insert(hash);
        *map.entry(hash).or_default() += 1;
    }
}

fn create_activities_repr_from_subtraces<TLog: EventLog>(
    traces_activities: &TracesActivities,
    regex_hasher: Option<&RegexEventHasher>,
    all_event_classes: &mut HashSet<u64>,
    params: &ActivitiesVisualizationParams<TLog>,
    event_classes_updater: impl Fn(&[Rc<RefCell<TLog::TEvent>>], &mut HashMap<u64, usize>, &mut HashSet<u64>) -> (),
) -> HashMap<String, (Rc<RefCell<ActivityNode>>, HashMap<u64, usize>)> {
    let mut processed = HashMap::new();
    for trace_activities in traces_activities.iter() {
        for activity in trace_activities {
            if processed.contains_key(&activity.node.borrow().name) {
                continue;
            }

            if activity.node.borrow().level != params.activity_level {
                continue;
            }

            let node = activity.node.borrow();
            if !processed.contains_key(&node.name) {
                processed.insert(node.name.to_owned(), (activity.node.clone(), HashMap::new()));
            }

            let map: &mut HashMap<u64, usize> = &mut processed.get_mut(&node.name).unwrap().1;
            if let Some(repeat_set) = node.repeat_set.as_ref() {
                let array = repeat_set.sub_array;
                let trace = params.log.traces().get(repeat_set.trace_index).unwrap();
                let events = trace.borrow();
                let events = events.events();

                let start = array.start_index;
                let end = start + array.length;
                event_classes_updater(&events[start..end], map, all_event_classes);
            }
        }
    }

    processed
        .into_iter()
        .map(|x| (x.0, (x.1 .0, x.1 .1.into_iter().map(|x| (x.0, x.1)).collect())))
        .collect()
}

fn create_dataset_internal(
    traces_activities: &TracesActivities,
    class_extractor: Option<String>,
    activities_repr_fullfiller: impl Fn(
        &Vec<Vec<ActivityInTraceInfo>>,
        Option<&RegexEventHasher>,
        &mut HashSet<u64>,
    ) -> HashMap<String, (Rc<RefCell<ActivityNode>>, HashMap<u64, usize>)>,
) -> Option<(MyDataset, ActivityNodeWithCoords, Vec<String>)> {
    let mut all_event_classes = HashSet::new();
    let regex_hasher = match class_extractor.as_ref() {
        Some(class_extractor) => Some(RegexEventHasher::new(class_extractor).ok().unwrap()),
        None => None,
    };

    let processed = activities_repr_fullfiller(traces_activities, regex_hasher.as_ref(), &mut all_event_classes);

    let mut all_event_classes = all_event_classes.into_iter().collect::<Vec<u64>>();
    all_event_classes.sort();

    let mut processed = processed.iter().map(|x| x.1.clone()).collect::<ActivityNodeWithCoords>();
    processed.sort_by(|first, second| first.0.borrow().name.cmp(&second.0.borrow().name));

    let mut vector = vec![];
    for activity in &processed {
        for i in 0..all_event_classes.len() {
            let count = if let Some(count) = activity.1.get(&all_event_classes[i]) {
                *count
            } else {
                0
            };

            vector.push(count as f64);
        }
    }

    for i in 0..all_event_classes.len() {
        let mut max = f64::MIN;
        let mut min = f64::MAX;

        for j in 0..processed.len() {
            let index = i + j * all_event_classes.len();
            max = max.max(vector[index]);
            min = min.min(vector[index]);
        }

        for j in 0..processed.len() {
            let index = i + j * all_event_classes.len();
            vector[index] = (vector[index] - min) / (max - min);
        }
    }

    let shape = (processed.len(), all_event_classes.len());

    let array = match Array2::from_shape_vec(shape, vector) {
        Ok(score) => score,
        Err(_) => return None,
    };

    Some((
        DatasetBase::from(array),
        processed,
        all_event_classes.iter().map(|x| x.to_string()).collect(),
    ))
}

pub(super) fn create_dataset_from_activities_classes<TLog: EventLog>(
    params: &ActivitiesVisualizationParams<TLog>,
) -> Option<(MyDataset, ActivityNodeWithCoords, Vec<String>)> {
    create_dataset_internal(
        params.traces_activities,
        params.class_extractor.clone(),
        |traces_activities, regex_hasher, all_event_classes| {
            let mut processed = HashMap::new();
            for trace_activities in traces_activities.iter() {
                for activity in trace_activities {
                    if processed.contains_key(&activity.node.borrow().name) {
                        continue;
                    }

                    if activity.node.borrow().level != params.activity_level {
                        continue;
                    }

                    let activity_event_classes = if let Some(regex_hasher) = regex_hasher.as_ref() {
                        if let Some(repeat_set) = activity.node.borrow().repeat_set.as_ref() {
                            let trace = params.log.traces().get(repeat_set.trace_index).unwrap();
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
                        (activity.node.clone(), activity_event_classes.into_iter().map(|x| (x, 1)).collect()),
                    );
                }
            }

            processed
        },
    )
}

#[derive(Clone)]
pub struct CosineDistance {}

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

pub fn transform_to_ficus_dataset(
    dataset: &MyDataset,
    processed: &Vec<(Rc<RefCell<ActivityNode>>, HashMap<u64, usize>)>,
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

    let row_names = processed.iter().map(|x| x.0.borrow().name.to_owned()).collect();
    FicusDataset::new(matrix, classes_names, row_names)
}
