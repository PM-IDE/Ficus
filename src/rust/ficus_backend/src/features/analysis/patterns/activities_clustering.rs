use std::collections::HashSet;

use crate::pipelines::aliases::TracesActivities;

pub fn clusterize_activities(traces_activities: &TracesActivities) {
    let mut all_event_classes = HashSet::new();
    for trace_activities in traces_activities {
        for activity in trace_activities {
            for event_class in &activity.node.borrow().event_classes {
                all_event_classes.insert(event_class.to_owned());
            }
        }
    }

    let all_event_classes = all_event_classes.into_iter().collect::<Vec<u64>>();

    let mut activities_vectors = vec![];
    for trace_activities in traces_activities {
        for activity in trace_activities {
            let mut activity_vector = vec![0;all_event_classes.len()];

            for i in 0..activity_vector.len() {
                if activity.node.borrow().event_classes.contains(&all_event_classes[i]) {
                    activity_vector[i] = 1;
                }
            }

            activities_vectors.push(activity_vector);
        }
    }

    println!("{:?}", &activities_vectors);
}