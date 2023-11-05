use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::providers::relations_cache::RelationsCaches;
use crate::utils::graph::graph::Graph;
use std::collections::HashMap;

pub type FuzzyGraph = Graph<String, f64>;

pub fn discover_graph_fuzzy(
    log: &impl EventLog,
    unary_frequency_threshold: f64,
    binary_frequency_significance_threshold: f64,
    preserve_threshold: f64,
    ratio_threshold: f64,
) -> FuzzyGraph {
    let mut graph = FuzzyGraph::empty();

    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let mut provider = FuzzyMetricsProvider::new(log, &info);

    let mut classes_to_ids = HashMap::new();
    for class in info.all_event_classes() {
        if provider.unary_frequency_significance(class) > unary_frequency_threshold {
            let node_id = graph.add_node(Some(class.to_owned()));
            classes_to_ids.insert(class.to_owned(), node_id);
        }
    }

    for first_class in classes_to_ids.keys() {
        for second_class in classes_to_ids.keys() {
            if provider.binary_frequency_significance(first_class, second_class) > binary_frequency_significance_threshold {
                let first_id = classes_to_ids.get(first_class).unwrap();
                let second_id = classes_to_ids.get(second_class).unwrap();
                graph.connect_nodes(first_id, second_id, None);
            }
        }
    }

    for first_name in classes_to_ids.keys() {
        for second_name in classes_to_ids.keys() {
            let first_id = classes_to_ids.get(first_name).unwrap();
            let second_id = classes_to_ids.get(second_name).unwrap();

            if are_nodes_bi_connected(&graph, first_id, second_id) {
                let first_second_sig = provider.relative_significance(first_name, second_name, &graph);
                let second_first_sig = provider.relative_significance(second_name, first_name, &graph);

                if first_second_sig < preserve_threshold || second_first_sig < preserve_threshold {
                    let offset = (first_second_sig - second_first_sig).abs();

                    if offset > ratio_threshold {
                        if first_second_sig < second_first_sig {
                            graph.disconnect_nodes(first_id, second_id);
                        } else {
                            graph.disconnect_nodes(second_id, first_id);
                        }
                    } else {
                        graph.disconnect_nodes(first_id, second_id);
                        graph.disconnect_nodes(second_id, first_id);
                    }
                }
            }
        }
    }

    graph
}

fn are_nodes_bi_connected(graph: &FuzzyGraph, first_node_id: &u64, second_node_id: &u64) -> bool {
    graph.are_nodes_connected(first_node_id, second_node_id) && graph.are_nodes_connected(second_node_id, first_node_id)
}

const PROXIMITY_CORRELATION: &'static str = "ProximityCorrelation";

struct FuzzyMetricsProvider<'a, TLog>
where
    TLog: EventLog,
{
    log: &'a TLog,
    log_info: &'a EventLogInfo,
    caches: RelationsCaches<f64>,
}

impl<'a, TLog> FuzzyMetricsProvider<'a, TLog>
where
    TLog: EventLog,
{
    pub fn new(log: &'a TLog, log_info: &'a EventLogInfo) -> Self {
        Self {
            log,
            log_info,
            caches: RelationsCaches::new(&[PROXIMITY_CORRELATION]),
        }
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

        self.caches
            .cache_mut(PROXIMITY_CORRELATION)
            .put(first_class, second_class, result.clone());
        result
    }

    pub fn relative_significance(&self, a: &String, b: &String, graph: &FuzzyGraph) -> f64 {
        let a_b_sig = self.binary_frequency_significance(a, b);

        let mut first_sig = 0.5 * a_b_sig;
        let mut second_sig = 0.5 * a_b_sig;
        let mut first_sum = 0.0;
        let mut second_sum = 0.0;

        for node in graph.all_nodes() {
            let name = node.data().unwrap();
            first_sum += self.binary_frequency_significance(a, name);
            second_sum += self.binary_frequency_significance(name, b);
        }

        first_sig /= first_sum;
        second_sig /= second_sum;

        first_sig + second_sig
    }
}