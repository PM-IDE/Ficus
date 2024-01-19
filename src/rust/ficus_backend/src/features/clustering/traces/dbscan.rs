use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use linfa::{traits::Transformer, DatasetBase};
use linfa_clustering::Dbscan;
use linfa_nn::KdTree;
use ndarray::Array2;

use crate::{
    event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    features::{clustering::common::{
        create_colors_vector, scale_raw_dataset_min_max, transform_to_ficus_dataset, DistanceWrapper, FicusDistance, MyDataset,
    }, analysis::patterns::activity_instances::{create_vector_of_underlying_events, create_vector_of_immediate_underlying_events}},
    utils::dataset::dataset::LabeledDataset,
};

use super::traces_params::{TracesClusteringParams, TracesRepresentationSource};

pub fn clusterize_log_by_traces_dbscan<TLog: EventLog>(
    params: &mut TracesClusteringParams<TLog>,
    min_points: usize,
) -> Option<(Vec<TLog>, LabeledDataset)> {
    if let Some((dataset, objects, features)) = create_traces_dataset(params.vis_params.log, &params.distance, &params.repr_source) {
        let clusters = Dbscan::params_with(min_points, DistanceWrapper::new(params.distance), KdTree)
            .tolerance(params.tolerance)
            .transform(dataset.records())
            .unwrap();

        let ficus_dataset = transform_to_ficus_dataset(&dataset, objects, features);

        let labels = clusters
            .into_raw_vec()
            .iter()
            .map(|x| if x.is_none() { 0 } else { x.unwrap() + 1 })
            .collect();

        let mut new_logs: HashMap<usize, TLog> = HashMap::new();
        for (trace, label) in params.vis_params.log.traces().iter().zip(&labels) {
            let trace_copy = trace.borrow().clone();
            if let Some(cluster_log) = new_logs.get_mut(label) {
                cluster_log.push(Rc::new(RefCell::new(trace_copy)));
            } else {
                let mut cluster_log = TLog::empty();
                cluster_log.push(Rc::new(RefCell::new(trace_copy)));

                new_logs.insert(label.to_owned(), cluster_log);
            }
        }

        let new_logs = new_logs.into_iter().map(|x| x.1).collect();
        let colors = create_colors_vector(&labels, &mut params.vis_params.colors_holder);

        Some((new_logs, LabeledDataset::new(ficus_dataset, labels, colors)))
    } else {
        None
    }
}

fn create_traces_dataset<TLog: EventLog>(
    log: &TLog, 
    distance: &FicusDistance,
    trace_repr_source: &TracesRepresentationSource
) -> Option<(MyDataset, Vec<String>, Vec<String>)> {
    match distance {
        FicusDistance::Cosine | FicusDistance::L1 | FicusDistance::L2 => create_traces_dataset_default(log, trace_repr_source),
        FicusDistance::Levenshtein => create_traces_dataset_levenshtein(log, trace_repr_source),
    }
}

fn create_traces_dataset_default<TLog: EventLog>(
    log: &TLog,
    trace_repr_source: &TracesRepresentationSource,
) -> Option<(MyDataset, Vec<String>, Vec<String>)> {
    create_traces_dataset_default_internal(log, |trace| create_trace_representation::<TLog>(trace, trace_repr_source))
}

fn create_trace_representation<TLog: EventLog>(
    trace: &TLog::TTrace, 
    trace_repr_source: &TracesRepresentationSource
) -> Vec<Rc<RefCell<TLog::TEvent>>> {
    match trace_repr_source {
        TracesRepresentationSource::Events => trace.events().clone(),
        TracesRepresentationSource::UnderlyingEvents => {
            let mut events = vec![];
            for event in trace.events() {
                for event in create_vector_of_immediate_underlying_events::<TLog>(event) {
                    events.push(event);
                }
            }

            events
        },
        TracesRepresentationSource::DeepestUnderlyingEvents => {
            let mut events = vec![];
            for event in trace.events() {
                for underlying_event in create_vector_of_underlying_events::<TLog>(event) {
                    events.push(underlying_event);
                }
            }

            events
        }
    }
}

fn create_traces_dataset_default_internal<TLog: EventLog>(
    log: &TLog,
    trace_repr_creator: impl Fn(&TLog::TTrace) -> Vec<Rc<RefCell<TLog::TEvent>>>,
) -> Option<(MyDataset, Vec<String>, Vec<String>)> {
    let mut processed_traces = vec![];
    for trace in log.traces() {
        let trace = trace.borrow();
        processed_traces.push(trace_repr_creator(&trace));
    }

    let mut all_event_classes = HashSet::new();
    for trace in &processed_traces {
        for event in trace {
            all_event_classes.insert(event.borrow().name().to_owned());
        }
    }

    let mut all_event_classes = all_event_classes.into_iter().collect::<Vec<String>>();
    all_event_classes.sort();

    let mut raw_dataset = vec![];
    for trace in &processed_traces {
        let mut events_counts: HashMap<String, usize> = HashMap::new();

        for event in trace {
            let event = event.borrow();
            *events_counts.entry(event.name().to_owned()).or_default() += 1;
        }

        for class in &all_event_classes {
            raw_dataset.push(if let Some(count) = events_counts.get(class) { *count } else { 0 } as f64);
        }
    }

    scale_raw_dataset_min_max(&mut raw_dataset, processed_traces.len(), all_event_classes.len());

    let shape = (processed_traces.len(), all_event_classes.len());
    let array = match Array2::from_shape_vec(shape, raw_dataset) {
        Ok(score) => score,
        Err(_) => return None,
    };

    Some((
        DatasetBase::from(array),
        (0..processed_traces.len()).into_iter().map(|x| format!("Trace_{}", x)).collect(),
        all_event_classes,
    ))
}

fn create_traces_dataset_levenshtein<TLog: EventLog>(
    log: &TLog,
    trace_repr_source: &TracesRepresentationSource
) -> Option<(MyDataset, Vec<String>, Vec<String>)> {
    create_traces_dataset_levenshtein_internal(log, |trace| create_trace_representation::<TLog>(trace, trace_repr_source))
}

fn create_traces_dataset_levenshtein_internal<TLog: EventLog>(
    log: &TLog,
    trace_repr_creator: impl Fn(&TLog::TTrace) -> Vec<Rc<RefCell<TLog::TEvent>>>,
) -> Option<(MyDataset, Vec<String>, Vec<String>)> {
    let mut processed_traces = vec![];
    for trace in log.traces() {
        let trace = trace.borrow();
        processed_traces.push(trace_repr_creator(&trace));
    }

    let mut all_event_classes = HashMap::new();
    let mut max_length = 0;
    for trace in &processed_traces {
        max_length = max_length.max(trace.len() + 1);

        for event in trace {
            if !all_event_classes.contains_key(event.borrow().name()) {
                all_event_classes.insert(event.borrow().name().to_owned(), all_event_classes.len());
            }
        }
    }

    let mut raw_dataset = vec![];
    for trace in &processed_traces {
        for event in trace {
            raw_dataset.push(*all_event_classes.get(event.borrow().name()).expect("Should be there") as f64);
        }

        for _ in trace.len()..max_length {
            raw_dataset.push(0f64);
        }
    }

    let shape = (processed_traces.len(), max_length);
    let array = match Array2::from_shape_vec(shape, raw_dataset) {
        Ok(score) => score,
        Err(_) => return None,
    };

    Some((
        DatasetBase::from(array),
        (0..processed_traces.len()).into_iter().map(|x| format!("Trace_{}", x)).collect(),
        (0..max_length).into_iter().map(|x| format!("Symbol_{}", x)).collect(),
    ))
}
