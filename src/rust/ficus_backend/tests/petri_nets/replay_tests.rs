use ficus_backend::{features::{
    analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto},
    discovery::{
        alpha::{alpha::discover_petri_net_alpha, providers::alpha_provider::DefaultAlphaRelationsProvider},
        petri_net::{
            annotations::{annotate_with_counts, annotate_with_frequencies},
            replay::replay_petri_net, petri_net::DefaultPetriNet,
        },
    },
}, event_log::core::event_log::EventLog, vecs};

use crate::test_core::simple_events_logs_provider::create_simple_event_log;

#[test]
pub fn test_simple_replay() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    let expected_transitions = vec![
        Some(vecs!["A", "B", "C"]),
        Some(vecs!["A", "B", "C"]),
    ];

    execute_test_with_replay(&petri_net, &log, expected_transitions);
}

fn execute_test_with_replay(net: &DefaultPetriNet, log: &impl EventLog, expected_transitions: Vec<Option<Vec<String>>>) {
    let replay_states = replay_petri_net(log, net).unwrap();
    if replay_states.len() != expected_transitions.len() {
        panic!();
    }

    for (replay_state, expected_transitions) in replay_states.iter().zip(expected_transitions.iter()) {
        if replay_state.is_none() && expected_transitions.is_none() {
            continue;
        }

        if !(replay_state.is_some() && expected_transitions.is_some()) {
            panic!();
        }

        let expected = expected_transitions.as_ref().unwrap();
        let state = replay_state.as_ref().unwrap();

        let replayed_transitions: Vec<String> = state.fired_transitions().iter().map(|id| net.transition(id).name().to_owned()).collect();
        assert_eq!(&replayed_transitions, expected);
    }
}

#[test]
pub fn test_simple_count_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    println!("{:?}", annotate_with_counts(&log, &petri_net, true));
}

#[test]
pub fn test_simple_frequency_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    println!("{:?}", annotate_with_frequencies(&log, &petri_net, true));
}
