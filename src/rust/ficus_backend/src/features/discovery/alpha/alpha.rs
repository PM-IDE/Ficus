use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::EventLogInfo;
use crate::features::discovery::alpha::alpha_set::AlphaSet;
use crate::features::discovery::alpha::provider::{
    AlphaPlusRelationsProvider, AlphaRelationsProvider, DefaultAlphaRelationsProvider,
};
use crate::features::discovery::petri_net::{DefaultPetriNet, Marking, PetriNet, Place, SingleMarking, Transition};
use std::collections::HashMap;
use std::hash::Hash;

pub fn discover_petri_net_alpha(event_log_info: &EventLogInfo) -> DefaultPetriNet {
    let dfg_info = event_log_info.get_dfg_info();
    let provider = DefaultAlphaRelationsProvider::new(&dfg_info);

    do_discover_petri_net_alpha(event_log_info, &provider)
}

pub fn discover_petri_net_alpha_plus(log: &impl EventLog, event_log_info: &EventLogInfo) -> DefaultPetriNet {
    let dfg_info = event_log_info.get_dfg_info();
    let provider = AlphaPlusRelationsProvider::new(dfg_info, log);

    do_discover_petri_net_alpha(event_log_info, &provider)
}

fn do_discover_petri_net_alpha(
    event_log_info: &EventLogInfo,
    provider: &impl AlphaRelationsProvider,
) -> DefaultPetriNet {
    let event_classes = event_log_info.get_all_event_classes();
    let dfg_info = event_log_info.get_dfg_info();

    let mut set_pairs: Vec<AlphaSet> = event_classes
        .iter()
        .filter(|class| {
            dfg_info.get_followed_events(class).is_some() && provider.is_in_unrelated_relation(class, class)
        })
        .map(|class| {
            AlphaSet::new(
                class,
                Vec::from_iter(
                    dfg_info
                        .get_followed_events(class)
                        .unwrap()
                        .keys()
                        .filter(|second_class| {
                            provider.is_in_casual_relation(class, second_class)
                                && provider.is_in_unrelated_relation(second_class, second_class)
                        }),
                ),
            )
        })
        .filter(|set| set.left_classes().len() > 0 && set.right_classes().len() > 0)
        .collect();

    let mut extended_pairs = vec![];
    for i in 0..set_pairs.len() {
        for j in (i + 1)..set_pairs.len() {
            let first_set = set_pairs.get(i);
            let first_set = first_set.unwrap();

            let second_set = set_pairs.get(j);
            let second_set = second_set.unwrap();

            let should_extend = (first_set.is_left_subset(second_set) || first_set.is_right_subset(second_set))
                && first_set.can_extend(second_set, provider);

            if should_extend {
                let new_set = first_set.extend(&second_set);
                extended_pairs.push(new_set);
            }
        }
    }

    let alpha_sets: Vec<&AlphaSet> = set_pairs.iter().chain(extended_pairs.iter()).collect();
    let alpha_sets: Vec<&AlphaSet> = alpha_sets
        .iter()
        .filter(|pair| {
            !alpha_sets
                .iter()
                .any(|candidate| *pair != candidate && pair.is_full_subset(candidate))
        })
        .map(|s| *s)
        .collect();

    let mut petri_net = PetriNet::empty();
    let mut event_classes_to_transition_ids = HashMap::new();
    for class in event_classes {
        let id = petri_net.add_transition(Transition::empty(class.to_owned(), Some(class.to_owned())));
        event_classes_to_transition_ids.insert(class, id);
    }

    for alpha_set in alpha_sets {
        let place_id = petri_net.add_place(Place::with_name(alpha_set.to_string()));

        for class in alpha_set.left_classes() {
            petri_net.connect_transition_to_place(event_classes_to_transition_ids[class], place_id, None);
        }

        for class in alpha_set.right_classes() {
            petri_net.connect_place_to_transition(place_id, event_classes_to_transition_ids[class], None);
        }
    }

    let start_place_id = petri_net.add_place(Place::empty());
    for start_activity in event_log_info.start_event_classes() {
        petri_net.connect_place_to_transition(start_place_id, event_classes_to_transition_ids[start_activity], None);
    }

    let end_place_id = petri_net.add_place(Place::empty());
    for end_activity in event_log_info.end_event_classes() {
        petri_net.connect_transition_to_place(event_classes_to_transition_ids[end_activity], end_place_id, None);
    }

    petri_net.set_initial_marking(Marking::new(vec![SingleMarking::new(start_place_id, 1)]));
    petri_net.set_final_marking(Marking::new(vec![SingleMarking::new(end_place_id, 1)]));

    return petri_net;
}
