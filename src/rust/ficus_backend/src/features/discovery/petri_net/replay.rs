use std::collections::{HashMap, VecDeque};

use crate::event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace};

use super::{marking::Marking, petri_net::DefaultPetriNet};

struct ReplayState {
    markings: HashMap<u64, usize>,
}

impl ReplayState {
    pub fn new_raw(markings: HashMap<u64, usize>) -> Self {
        Self { markings }
    }

    pub fn new(initial_marking: Marking) -> Self {
        Self {
            markings: initial_marking
                .active_places()
                .iter()
                .map(|c| (c.place_id(), c.tokens_count()))
                .collect(),
        }
    }

    pub fn handle_transition(state: &ReplayState, net: &DefaultPetriNet, transition: &str) -> Option<Vec<ReplayState>> {
        let candidates = net.find_all_transitions_by_name(transition)?;

        let mut new_states = vec![];
        for candidate_transition in candidates {
            let mut can_fire = true;
            for arc in candidate_transition.incoming_arcs() {
                if !state.markings.contains_key(&arc.place_id()) {
                    can_fire = false;
                    break;
                }
            }

            if can_fire {
                let mut new_markings = state.markings.clone();
                for arc in candidate_transition.incoming_arcs() {
                    let place_id = &arc.place_id();
                    let count = new_markings[place_id];
                    let new_count = count - arc.tokens_count();
                    if new_count <= 0 {
                        new_markings.remove(place_id);
                    } else {
                        *new_markings.get_mut(place_id).unwrap() = new_count;
                    }
                }

                for arc in candidate_transition.outgoing_arcs() {
                    let place_id = &arc.place_id();
                    *new_markings.get_mut(place_id).unwrap() = if let Some(count) = new_markings.get(place_id) {
                        count + arc.tokens_count()
                    } else {
                        *arc.tokens_count()
                    }
                }

                new_states.push(ReplayState::new_raw(new_markings))
            }
        }

        Some(new_states)
    }
}

pub fn replay_petri_net(log: &impl EventLog, net: &DefaultPetriNet) -> bool {
    for trace in log.traces() {
        let marking =
            match net.initial_marking() {
                Some(marking) => marking.clone(),
                None => return false,
            };

        let trace = trace.borrow();
        let mut stack = VecDeque::new();
        stack.push_back((0usize, ReplayState::new(marking)));

        loop {
            if stack.len() == 0 {
                return false;
            }

            let current_state = stack.pop_back().unwrap();
            let events = trace.events();
            if current_state.0 >= events.len() {
                break;
            }

            let transition = trace.events().get(current_state.0).unwrap();
            let new_states = ReplayState::handle_transition(&current_state.1, net, transition.borrow().name());

            if let Some(new_states) = new_states {
                for new_state in new_states {
                    stack.push_back((current_state.0 + 1, new_state));
                }
            }
        }
    }

    true
}
