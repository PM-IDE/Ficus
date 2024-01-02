use ficus_backend::features::{discovery::{alpha::{alpha::discover_petri_net_alpha, providers::alpha_provider::DefaultAlphaRelationsProvider}, petri_net::{replay::replay_petri_net, annotations::{annotate_with_counts, annotate_with_frequencies}}}, analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto}};

use crate::test_core::simple_events_logs_provider::create_simple_event_log;

#[test]
pub fn test_simple_replay() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    println!("{:?}", replay_petri_net(&log, &petri_net));
}

#[test]
pub fn test_simple_count_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    println!("{:?}", annotate_with_counts(&log, &petri_net));
}

#[test]
pub fn test_simple_frequency_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    println!("{:?}", annotate_with_frequencies(&log, &petri_net));
}