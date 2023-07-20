use std::collections::HashMap;

use crate::event_log::core::{event::Event, event_log::EventLog, trace::Trace};

use super::event_log_info::{EventLogInfo, EventLogInfoCreationDto};

pub fn calculate_pos_entropies<TLog>(log: &TLog) -> HashMap<String, f64>
where
    TLog: EventLog,
{
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let mut entropies = HashMap::new();
    for event_name in log_info.get_all_event_classes() {
        entropies.insert(event_name.to_owned(), calculate_pos_entropy_for_event(log, event_name));
    }

    entropies
}

pub fn calculate_pos_entropy_for_event<TLog>(log: &TLog, event_name: &String) -> f64
where
    TLog: EventLog,
{
    let vector_length = calculate_vector_length(log);
    let mut prob_vector = vec![0f64; vector_length];
    let mut non_empty_traces_count = 0;

    for trace in log.get_traces() {
        let mut index = 0;
        let mut empty_trace = true;
        for event in trace.borrow().get_events() {
            empty_trace = false;
            if event.borrow().get_name() == event_name {
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
