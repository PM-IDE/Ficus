use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::providers::relations_cache::RelationsCaches;
use crate::utils::graph::graph::Graph;
use std::collections::HashMap;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::core::event::event::Event;

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

const PROXIMITY_CORRELATION: &'static str = "ProximityCorrelation";

struct FuzzyMetricsProvider<'a, TLog> where TLog: EventLog {
    log: &'a TLog,
    log_info: &'a EventLogInfo,
    caches: RelationsCaches<f64>
}

impl<'a, TLog> FuzzyMetricsProvider<'a, TLog> where TLog: EventLog {
    pub fn new(log: &'a TLog, log_info: &'a EventLogInfo) -> Self {
        Self { log, log_info, caches: RelationsCaches::new(&[PROXIMITY_CORRELATION]) }
    }

    pub fn unary_frequency_significance(&self, event_class: &String) -> f64 {
        self.log_info.event_count(event_class) as f64
    }

    pub fn binary_frequency_significance(&self, first_class: &String, second_class: &String) -> f64 {
        self.log_info.dfg_info().get_directly_follows_count(first_class, second_class) as f64
    }

    pub fn proximity_correlation(&mut self, first_class: &String, second_class: &String) -> f64 {
        if let Some(value) = self.caches.cache(PROXIMITY_CORRELATION).try_get(first_class, second_class) {
            return *value;
        }

        let mut count = 0;
        let mut result = 0.0;
        for trace in self.log.traces() {
            let trace = trace.borrow();
            let events = trace.events();
            let mut last_seen_first = None;

            for i in 0..events.len() {
                let event = events[i].borrow();
                let name = event.name();

                if name == first_class {
                    last_seen_first = Some(i.clone());
                    continue;
                }

                if name == second_class {
                    if let Some(first_index) = last_seen_first {
                        let second_stamp = event.timestamp();
                        let first_event = events.get(first_index).unwrap();
                        let first_event = first_event.borrow();
                        let first_stamp = first_event.timestamp();

                        result += second_stamp.signed_duration_since(*first_stamp).num_milliseconds() as f64;
                        count += 1;
                        last_seen_first = None;
                    }
                }
            }
        }

        result = result / (count as f64);

        self.caches.cache_mut(PROXIMITY_CORRELATION).put(first_class, second_class, result.clone());
        result
    }
}
