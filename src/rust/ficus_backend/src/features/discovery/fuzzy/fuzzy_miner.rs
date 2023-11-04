use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::providers::relations_cache::RelationsCaches;
use crate::utils::graph::graph::Graph;
use std::collections::HashMap;

pub type FuzzyGraph = Graph<String, f64>;

pub fn discover_graph_fuzzy(log: &impl EventLog) -> FuzzyGraph {
    let mut graph = FuzzyGraph::empty();

    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));

    let mut classes_to_ids = HashMap::new();
    for class in info.all_event_classes() {
        let node_id = graph.add_node(Some(class.to_owned()));
        classes_to_ids.insert(class.to_owned(), node_id);
    }

    graph
}

struct FuzzyMetricsProvider<'a> {
    log_info: &'a EventLogInfo,
}

impl<'a> FuzzyMetricsProvider<'a> {
    pub fn new(log_info: &'a EventLogInfo) -> Self {
        Self { log_info }
    }

    pub fn unary_frequency_significance(&self, event_class: &String) -> f64 {
        self.log_info.event_count(event_class) as f64
    }

    pub fn binary_frequency_significance(&self, first_class: &String, second_class: &String) -> f64 {
        self.log_info.dfg_info().get_directly_follows_count(first_class, second_class) as f64
    }
}
