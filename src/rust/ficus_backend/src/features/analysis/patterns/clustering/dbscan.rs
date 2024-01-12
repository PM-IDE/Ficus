use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use linfa_nn::KdTree;

use crate::{event_log::core::event_log::EventLog, utils::dataset::dataset::LabeledDataset};

use super::{
    common::{create_dataset, transform_to_ficus_dataset, create_colors_vector},
    merging::merge_activities,
    params::{ClusteringCommonParams, DistanceWrapper},
};

pub fn clusterize_activities_dbscan<TLog: EventLog>(
    params: &mut ClusteringCommonParams<TLog>,
    min_points: usize,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset(&params.vis_params) {
        let clusters = Dbscan::params_with(min_points, DistanceWrapper::new(params.distance), KdTree)
            .tolerance(params.tolerance)
            .transform(dataset.records())
            .unwrap();

        merge_activities(
            params.vis_params.log,
            params.vis_params.traces_activities,
            &processed.iter().map(|x| x.0.clone()).collect(),
            &clusters,
        );

        let ficus_dataset = transform_to_ficus_dataset(&dataset, &processed, classes_names);
        let labels = clusters
            .into_raw_vec()
            .iter()
            .map(|x| if x.is_none() { 0 } else { x.unwrap() + 1 })
            .collect();

        let colors = create_colors_vector(&labels, params.vis_params.colors_holder);
        Some(LabeledDataset::new(ficus_dataset, labels, colors))
    } else {
        None
    }
}
