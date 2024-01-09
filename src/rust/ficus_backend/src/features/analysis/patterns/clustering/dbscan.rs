use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use linfa_nn::KdTree;

use crate::{event_log::core::event_log::EventLog, pipelines::aliases::TracesActivities, utils::dataset::dataset::LabeledDataset};

use super::common::{create_dataset_from_activities_classes, merge_activities, transform_to_ficus_dataset, CosineDistance};

pub fn clusterize_activities_dbscan(
    log: &impl EventLog,
    traces_activities: &mut TracesActivities,
    activity_level: usize,
    min_points: usize,
    tolerance: f64,
    class_extractor: Option<String>,
) -> Option<LabeledDataset> {
    if let Some((dataset, processed, classes_names)) = create_dataset_from_activities_classes(log, traces_activities, activity_level, class_extractor) {
        let clusters = Dbscan::params_with(min_points, CosineDistance {}, KdTree)
            .tolerance(tolerance)
            .transform(dataset.records())
            .unwrap();

        merge_activities(log, traces_activities, &processed.iter().map(|x| x.0.clone()).collect(), &clusters);
        let ficus_dataset = transform_to_ficus_dataset(&dataset, &processed, classes_names);
        let labels = clusters
            .into_raw_vec()
            .iter()
            .map(|x| if x.is_none() { 0 } else { x.unwrap() + 1 })
            .collect();

        Some(LabeledDataset::new(ficus_dataset, labels))
    } else {
        None
    }
}
