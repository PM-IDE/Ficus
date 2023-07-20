use std::collections::HashMap;

use crate::event_log::core::event_log::EventLog;

use super::{event_log_info::EventLogInfo, constants::{FAKE_EVENT_START_NAME, FAKE_EVENT_END_NAME}};

pub fn calculate_default_dfg_entropy<TLog>(log: &TLog) -> HashMap<String, f32> where TLog: EventLog {
    let dfr_calculator = |first: &String, second: &String, log_info: &EventLogInfo| {
        let dfg = log_info.get_dfg_info();
        let dfr = dfg.get_directly_follows_count(&(first.to_owned(), second.to_owned()));
        let first_count = log_info.get_event_count(first);
        dfr as f32 / first_count as f32
    };

    let dpr_calculator = |first: &String, second: &String, log_info: &EventLogInfo| {
        let dfg = log_info.get_dfg_info();
        let dfr = dfg.get_directly_follows_count(&(second.to_owned(), first.to_owned()));
        let first_count = log_info.get_event_count(first);
        dfr as f32 / first_count as f32
    };

    calculate_dfg_entropy(log, dfr_calculator, dpr_calculator)
}

fn calculate_dfg_entropy<TLog, TDfrEntropyCalculator, TDprEntropyCalculator>(
    log: &TLog,
    dfr_calculator: TDfrEntropyCalculator,
    dpr_calculator: TDprEntropyCalculator,
) -> HashMap<String, f32>
where 
    TLog: EventLog,
    TDfrEntropyCalculator: Fn(&String, &String, &EventLogInfo) -> f32,
    TDprEntropyCalculator: Fn(&String, &String, &EventLogInfo) -> f32,
{
    let log_info = EventLogInfo::create_from(log, true);
    let mut entropy = HashMap::new();
    let events_names: Vec<&String> = log_info.get_all_event_classes();
    
    let mut dfr_events_names = events_names.clone();
    let fake_end = FAKE_EVENT_END_NAME.to_string();
    dfr_events_names.push(&fake_end);
    
    let mut dpr_events_names = events_names.clone();
    let fake_start = FAKE_EVENT_START_NAME.to_string();
    dpr_events_names.push(&fake_start);

    for event_name in events_names {
        let dfr_vector: Vec<f32> = dfr_events_names
            .iter()
            .map(|current_name| dfr_calculator(event_name, current_name, &log_info))
            .collect();

        let dpr_vector: Vec<f32> = dpr_events_names
            .iter()
            .map(|current_name| dpr_calculator(event_name, current_name, &log_info))
            .collect();

        let event_entropy = calculate_entropy(&dfr_vector) + calculate_entropy(&dpr_vector);
        entropy.insert(event_name.to_string(), event_entropy);
    }

    entropy
}

fn calculate_entropy(values: &Vec<f32>) -> f32 {
    let mut entropy = 0f32;
    for value in values {
        if *value != 0f32 {
            entropy -= value * value.log2();
        }
    }

    entropy
}
