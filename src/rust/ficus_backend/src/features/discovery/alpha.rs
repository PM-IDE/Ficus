use crate::features::analysis::event_log_info::{DfgInfo, EventLogInfo};
use crate::features::discovery::petri_net::{DefaultPetriNet, Marking, PetriNet, Place, SingleMarking, Transition};
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};

struct DefaultAlphaRelationsProvider<'a> {
    dfg_info: &'a DfgInfo,
}

impl<'a> DefaultAlphaRelationsProvider<'a> {
    pub fn new(dfg_info: &'a DfgInfo) -> Self {
        Self { dfg_info }
    }

    pub fn is_in_casual_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }

    pub fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second) && self.is_in_direct_relation(second, first)
    }

    pub fn is_in_direct_relation(&self, first: &str, second: &str) -> bool {
        self.dfg_info.is_in_directly_follows_relation(first, second)
    }

    pub fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }
}

#[derive(Debug)]
struct AlphaSet<'a> {
    left_classes: BTreeSet<&'a String>,
    right_classes: BTreeSet<&'a String>,
}

impl<'a> AlphaSet<'a> {
    pub fn new(left_class: &'a String, right_classes: Vec<&'a String>) -> Self {
        let mut left_classes = BTreeSet::new();
        left_classes.insert(left_class);

        let right_classes = BTreeSet::from_iter(right_classes);
        return Self {
            left_classes,
            right_classes,
        };
    }

    pub fn is_left_subset(&self, other: &Self) -> bool {
        self.left_classes.is_subset(&other.right_classes)
    }

    pub fn is_right_subset(&self, other: &Self) -> bool {
        self.right_classes.is_subset(&other.left_classes)
    }

    pub fn is_full_subset(&self, other: &Self) -> bool {
        self.is_left_subset(other) && self.is_right_subset(other)
    }

    pub fn left_classes(&self) -> Vec<&'a String> {
        (&self.left_classes).iter().map(|c| *c).collect()
    }

    pub fn right_classes(&self) -> Vec<&'a String> {
        (&self.right_classes).iter().map(|c| *c).collect()
    }

    pub fn can_extend(&self, other: &Self, provider: &DefaultAlphaRelationsProvider) -> bool {
        for left_class in self.left_classes.iter().chain(other.left_classes.iter()) {
            for right_class in self.right_classes.iter().chain(other.right_classes.iter()) {
                if !provider.is_in_casual_relation(left_class, right_class) {
                    return false;
                }
            }
        }

        for first_left_class in self.left_classes.iter().chain(other.left_classes.iter()) {
            for second_left_class in self.left_classes.iter().chain(other.left_classes.iter()) {
                if first_left_class != second_left_class {
                    if !provider.is_in_unrelated_relation(first_left_class, second_left_class) {
                        return false;
                    }
                }
            }
        }

        for first_right_class in self.right_classes.iter().chain(other.right_classes.iter()) {
            for second_right_class in self.right_classes.iter().chain(other.right_classes.iter()) {
                if first_right_class != second_right_class {
                    if !provider.is_in_unrelated_relation(first_right_class, second_right_class) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    pub fn extend(&self, other: &Self) -> AlphaSet {
        let mut left_classes = self.left_classes.clone();
        left_classes.extend(other.left_classes.iter());

        let mut right_classes = self.right_classes.clone();
        right_classes.extend(other.right_classes.iter());

        Self {
            left_classes,
            right_classes,
        }
    }
}

impl<'a> Hash for AlphaSet<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for class in self.left_classes.iter() {
            state.write(class.as_bytes());
        }

        for class in self.right_classes.iter() {
            state.write(class.as_bytes());
        }
    }
}

impl<'a> PartialEq for AlphaSet<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut this_hasher = DefaultHasher::new();
        self.hash(&mut this_hasher);

        let mut other_hasher = DefaultHasher::new();
        other.hash(&mut other_hasher);

        this_hasher.finish() == other_hasher.finish()
    }
}

impl<'a> Eq for AlphaSet<'a> {}

pub fn discover_petri_net_alpha(event_log_info: EventLogInfo) -> DefaultPetriNet {
    let event_classes = event_log_info.get_all_event_classes();
    let dfg_info = event_log_info.get_dfg_info();

    let provider = DefaultAlphaRelationsProvider::new(&dfg_info);

    let mut set_pairs: Vec<AlphaSet> = event_classes
        .iter()
        .filter(|class| dfg_info.get_followed_events(class).is_some())
        .map(|class| {
            AlphaSet::new(
                class,
                Vec::from_iter(
                    dfg_info
                        .get_followed_events(class)
                        .unwrap()
                        .keys()
                        .filter(|second_class| provider.is_in_casual_relation(class, second_class)),
                ),
            )
        })
        .collect();

    let mut extended_pairs = vec![];
    for i in 0..set_pairs.len() {
        for j in (i + 1)..set_pairs.len() {
            let first_set = set_pairs.get(i);
            let first_set = first_set.unwrap();

            let second_set = set_pairs.get(j);
            let second_set = second_set.unwrap();

            let should_extend = (first_set.is_left_subset(second_set) || first_set.is_right_subset(second_set))
                && first_set.can_extend(second_set, &provider);

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
        let id = petri_net.add_transition(Transition::empty(Some(class.to_owned())));
        event_classes_to_transition_ids.insert(class, id);
    }

    for alpha_set in alpha_sets {
        let place_id = petri_net.add_place(Place::new());

        for class in alpha_set.left_classes() {
            petri_net.connect_transition_to_place(event_classes_to_transition_ids[class], place_id, None);
        }

        for class in alpha_set.right_classes() {
            petri_net.connect_place_to_transition(place_id, event_classes_to_transition_ids[class], None);
        }
    }

    let start_place_id = petri_net.add_place(Place::new());
    for start_activity in event_log_info.start_event_classes() {
        petri_net.connect_place_to_transition(start_place_id, event_classes_to_transition_ids[start_activity], None);
    }

    let end_place_id = petri_net.add_place(Place::new());
    for end_activity in event_log_info.end_event_classes() {
        petri_net.connect_transition_to_place(event_classes_to_transition_ids[end_activity], end_place_id, None);
    }

    petri_net.set_initial_marking(Marking::new(vec![SingleMarking::new(start_place_id, 1)]));
    petri_net.set_final_marking(Marking::new(vec![SingleMarking::new(end_place_id, 1)]));

    return petri_net;
}
