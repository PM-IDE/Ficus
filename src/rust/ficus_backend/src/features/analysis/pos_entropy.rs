use std::collections::{HashMap, HashSet};

use crate::event_log::core::{
    event::Event,
    event_log::EventLog,
    trace::{Trace, TraceInfo},
};

use super::event_log_info::{EventLogInfo, EventLogInfoCreationDto};

pub fn calculate_pos_entropies<TLog>(log: &TLog, ignored_events: &Option<HashSet<String>>) -> HashMap<String, f64>
where
    TLog: EventLog,
{
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let mut entropies = HashMap::new();
    for event_name in log_info.get_all_event_classes() {
        if let Some(ignored_events) = ignored_events {
            if ignored_events.contains(event_name) {
                continue;
            }
        }

        let entropy = calculate_pos_entropy_for_event(log, event_name, ignored_events);
        entropies.insert(event_name.to_owned(), entropy);
    }

    entropies
}

pub fn calculate_pos_entropy_for_event<TLog>(
    log: &TLog,
    event_name: &String,
    ignored_events: &Option<HashSet<String>>,
) -> f64
where
    TLog: EventLog,
{
    let vector_length = match ignored_events {
        Some(ignored_events) => calculate_vector_length_with_ignored_events(log, ignored_events),
        None => calculate_vector_length(log),
    };

    let mut prob_vector = vec![0f64; vector_length];
    let mut non_empty_traces_count = 0;

    for trace in log.get_traces() {
        let mut index = 0;
        let mut empty_trace = true;
        for event in trace.borrow().get_events() {
            let event = event.borrow();
            let name = event.get_name();

            if let Some(ignored_events_set) = ignored_events {
                if ignored_events_set.contains(name) {
                    continue;
                }
            }

            empty_trace = false;
            if name == event_name {
                prob_vector[index] += 1f64;
            }

            index += 1;
        }

        if !empty_trace {
            non_empty_traces_count += 1;
        }
    }

    for i in 0..prob_vector.len() {
        prob_vector[i] = prob_vector[i] / non_empty_traces_count as f64;
    }

    calculate_pos_entropy(&prob_vector, non_empty_traces_count)
}

fn calculate_vector_length_with_ignored_events<TLog>(log: &TLog, ignored_events: &HashSet<String>) -> usize
where
    TLog: EventLog,
{
    let mut max = 0;

    for trace in log.get_traces() {
        let mut trace = trace.borrow_mut();
        let counts = trace.get_or_create_trace_info().get_events_counts();
        let mut num_of_ignored_events = 0;
        for ignored_event in ignored_events {
            if let Some(count) = counts.get(ignored_event) {
                num_of_ignored_events += *count;
            }
        }

        max = max.max(trace.get_events().len() - num_of_ignored_events);
    }

    max
}

fn calculate_vector_length<TLog>(log: &TLog) -> usize
where
    TLog: EventLog,
{
    log.get_traces()
        .into_iter()
        .map(|trace| trace.borrow().get_events().len())
        .max()
        .unwrap()
}

fn calculate_pos_entropy(probabilities: &Vec<f64>, traces_count: usize) -> f64 {
    let log = (traces_count as f64).log2();
    let mut non_zero_count = 0;

    let sum: f64 = probabilities
        .iter()
        .filter(|p| {
            if **p != 0f64 {
                non_zero_count += 1;
                return true;
            }

            false
        })
        .map(|p| -p.log2() / log)
        .sum();

    sum / non_zero_count as f64
}
