use std::collections::HashMap;

use crate::event_log::core::event_log::EventLog;

use super::{petri_net::DefaultPetriNet, replay::replay_petri_net};

pub fn annotate_with_counts(log: &impl EventLog, net: &DefaultPetriNet) -> Option<HashMap<u64, usize>> {
    let replay_states = replay_petri_net(log, net);
    if replay_states.is_none() {
        return None;
    }

    let mut fired_arcs = HashMap::new();
    for state in replay_states.as_ref().unwrap() {
        for fired_transition in state.fired_transitions() {
            let transition = net.transition(fired_transition);
            for incoming_arc in transition.incoming_arcs() {
                handle_arc(&mut fired_arcs, incoming_arc.id());
            }

            for outgoing_arc in transition.outgoing_arcs() {
                handle_arc(&mut fired_arcs, outgoing_arc.id());
            }
        }
    }

    Some(fired_arcs)
}

fn handle_arc(fired_arcs: &mut HashMap<u64, usize>, arc_id: u64) {
    *fired_arcs.entry(arc_id).or_default() += 1;
}

pub fn annotate_with_frequencies(log: &impl EventLog, net: &DefaultPetriNet) -> Option<HashMap<u64, f64>> {
    let count_annotation = annotate_with_counts(log, net)?;
    let mut freq_annotations = HashMap::new();

    let sum: usize = count_annotation.values().into_iter().sum();
    for (arc_id, count) in count_annotation {
        freq_annotations.insert(arc_id, (count as f64) / sum as f64);
    }

    Some(freq_annotations)
}
