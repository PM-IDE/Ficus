use std::{collections::{HashSet, HashMap}, os::raw, rc::Rc, cell::RefCell};

use linfa::{traits::Transformer, DatasetBase};
use linfa_clustering::Dbscan;
use linfa_nn::KdTree;
use ndarray::Array2;

use crate::{event_log::core::{event_log::EventLog, trace::trace::Trace, event::event::Event}, utils::dataset::dataset::LabeledDataset, features::clustering::common::{DistanceWrapper, transform_to_ficus_dataset, create_colors_vector}};

use super::{activities_params::ActivitiesClusteringParams, activities_common::create_dataset, merging::merge_activities};


pub fn clusterize_activities_dbscan<TLog: EventLog>(
    params: &mut ActivitiesClusteringParams<TLog>,
    min_points: usize,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(&params.vis_params) {
        let clusters = Dbscan::params_with(min_points, DistanceWrapper::new(params.distance), KdTree)
            .tolerance(params.tolerance)
            .transform(dataset.records())
            .unwrap();

        merge_activities(
            params.vis_params.common_vis_params.log,
            params.vis_params.traces_activities,
            &processed.iter().map(|x| x.0.clone()).collect(),
            &clusters,
        );

        let ficus_dataset = transform_to_ficus_dataset(
            &dataset, 
            processed.iter().map(|x| x.0.borrow().name.to_owned()).collect(), 
            classes_names
        );
        
        let labels = clusters
            .into_raw_vec()
            .iter()
            .map(|x| if x.is_none() { 0 } else { x.unwrap() + 1 })
            .collect();

        let colors = create_colors_vector(&labels, params.vis_params.common_vis_params.colors_holder);
        Some(LabeledDataset::new(ficus_dataset, labels, colors))
    } else {
        None
    }
}
