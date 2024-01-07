use linfa::prelude::Predict;

use std::{collections::{HashSet, HashMap}, rc::Rc, cell::RefCell};

use linfa::{traits::Fit, DatasetBase};
use linfa_clustering::KMeans;
use linfa_nn::distance::LInfDist;
use ndarray::Array2;
use crate::{pipelines::aliases::TracesActivities, features::analysis::patterns::repeat_sets::ActivityNode};

pub fn clusterize_activities(traces_activities: &TracesActivities) {
    let mut all_event_classes = HashSet::new();
    let mut processed = HashMap::new();
    for trace_activities in traces_activities {
        for activity in trace_activities {
            if processed.contains_key(&activity.node.borrow().name) {
                continue;
            }

            for event_class in &activity.node.borrow().event_classes {
                all_event_classes.insert(event_class.to_owned());
            }

            processed.insert(activity.node.borrow().name.to_owned(), activity.node.clone());
        }
    }

    let all_event_classes = all_event_classes.into_iter().collect::<Vec<u64>>();
    let processed = processed.iter().map(|x| x.1.clone()).collect::<Vec<Rc<RefCell<ActivityNode>>>>();

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

    println!("{:?}, {}", &shape, vector.len());
    let array = Array2::from_shape_vec(shape, vector).ok().unwrap();
    let dataset = DatasetBase::from(array);

    let model = KMeans::params_with(processed.len() / 4, rand::thread_rng(), LInfDist)
        .max_n_iterations(10)
        .tolerance(1e-5)
        .fit(&dataset)
        .expect("KMeans fitted");

    let dataset = model.predict(dataset);
    let DatasetBase {
        records, targets, ..
    } = dataset;

    println!("{:?}", targets);
}