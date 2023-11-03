use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::providers::alpha_plus_provider::calculate_triangle_relations;
use crate::features::discovery::alpha::providers::alpha_provider::{AlphaRelationsProvider, DefaultAlphaRelationsProvider};
use crate::features::discovery::alpha::utils::maximize;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::utils::hash_utils::compare_based_on_hashes;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::features::analysis::patterns::activity_instances::process_activities_in_trace;

struct OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    set: BTreeSet<T>,
}

impl<T> OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    pub fn empty() -> Self {
        Self { set: BTreeSet::new() }
    }

    pub fn new(el: T) -> Self {
        Self {
            set: BTreeSet::from_iter(vec![el]),
        }
    }

    pub fn new_two_elements(first: T, second: T) -> Self {
        Self {
            set: BTreeSet::from_iter(vec![first, second]),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            set: self.set.iter().chain(other.set.iter()).map(|el| el.clone()).collect(),
        }
    }

    pub fn set(&self) -> &BTreeSet<T> {
        &self.set
    }
}

impl<T> Hash for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for el in &self.set {
            el.hash(state);
        }
    }
}

impl<T> PartialEq for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        compare_based_on_hashes(self, other)
    }
}

impl<T> Eq for OneSet<T> where T: Hash + Eq + Ord + Clone {}

impl<T> Clone for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn clone(&self) -> Self {
        Self { set: self.set.clone() }
    }
}

pub fn discover_petri_net_heuristic(
    log: &impl EventLog,
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
    and_threshold: f64,
) -> DefaultPetriNet {
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let provider = DefaultAlphaRelationsProvider::new(&info);
    let provider = HeuristicMinerMeasureProvider::new(
        log,
        provider,
        dependency_threshold,
        positive_observations_threshold,
        relative_to_best_threshold,
        and_threshold,
    );

    let mut petri_net = DefaultPetriNet::empty();

    let mut classes_to_ids = HashMap::new();
    for class in provider.log_info().all_event_classes() {
        let id = petri_net.add_transition(Transition::empty(class.to_owned(), Some(class.to_owned())));
        classes_to_ids.insert(class.to_owned(), id);
    }

    for first_class in provider.log_info().all_event_classes() {
        let mut followers = Vec::new();
        for second_class in provider.log_info().all_event_classes() {
            if provider.dependency_relation(first_class, second_class) {
                followers.push(second_class);
            }
        }

        if followers.len() == 0 {
            continue;
        }

        let mut and_relations = HashSet::new();
        for i in 0..followers.len() {
            for j in (i + 1)..followers.len() {
                let first = *followers.get(i).unwrap();
                let second = *followers.get(j).unwrap();

                if first != second && provider.and_or_xor_relation(first_class, first, second) == AndOrXorRelation::And {
                    and_relations.insert(OneSet::new_two_elements(first, second));
                }
            }
        }

        let parallel_groups = maximize(and_relations, |first, second| {
            let candidate = first.merge(second);
            for first_el in candidate.set() {
                for second_el in candidate.set() {
                    if first_el != second_el && provider.and_or_xor_relation(first_class, first_el, second_el) != AndOrXorRelation::And {
                        return None;
                    }
                }
            }

            Some(candidate)
        });

        let mut used = HashSet::new();
        let post_place_id = petri_net.add_place(Place::with_name(format!("post_{first_class}")));
        petri_net.connect_transition_to_place(classes_to_ids.get(first_class).unwrap(), &post_place_id, None);

        for group in &parallel_groups {
            let name = format!("silent_start_{first_class}");
            let id = petri_net.add_transition(Transition::empty(name.to_owned(), Some(name.to_owned())));
            petri_net.connect_place_to_transition(&post_place_id, &id, None);

            for el in group.set().iter() {
                let place_id = petri_net.add_place(Place::with_name(format!("pre_{el}")));
                petri_net.connect_transition_to_place(&id, &place_id, None);
                petri_net.connect_place_to_transition(&place_id, classes_to_ids.get(*el).unwrap(), None);

                used.insert(*el);
            }
        }

        for follower in &followers {
            if !used.contains(follower) {
                petri_net.connect_place_to_transition(&post_place_id, classes_to_ids.get(*follower).unwrap(), None);
            }
        }
    }

    let mut places_to_transitions = vec![];
    let mut transitions_to_places = vec![];
    for first_class in info.all_event_classes() {
        for second_class in info.all_event_classes() {
            if first_class != second_class && provider.loop_length_two_relation(first_class, second_class) {
                let first_transition = petri_net.find_transition_by_name(first_class).unwrap();
                let second_transition = petri_net.find_transition_by_name(second_class).unwrap();

                for output_arc in second_transition.outgoing_arcs() {
                    places_to_transitions.push((output_arc.place_id(), first_transition.id()));
                }

                for incoming_arc in second_transition.incoming_arcs() {
                    transitions_to_places.push((first_transition.id(), incoming_arc.place_id()));
                }
            }
        }
    }

    for (place_id, transition_id) in places_to_transitions {
        petri_net.connect_place_to_transition(&place_id, &transition_id, None);
    }

    for (transition_id, place_id) in transitions_to_places {
        petri_net.connect_transition_to_place(&transition_id, &place_id, None);
    }

    petri_net
}

type DependencyRelations = HashMap<String, HashMap<String, f64>>;

pub(crate) struct HeuristicMinerMeasureProvider<'a> {
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
    and_threshold: f64,
    triangle_relations: HashMap<(String, String), usize>,
    provider: DefaultAlphaRelationsProvider<'a>,
    dependency_relations: DependencyRelations,
}

#[derive(PartialEq)]
pub enum AndOrXorRelation {
    And,
    Xor,
}

impl<'a> HeuristicMinerMeasureProvider<'a> {
    pub fn new(
        log: &impl EventLog,
        provider: DefaultAlphaRelationsProvider<'a>,
        dependency_threshold: f64,
        positive_observations_threshold: usize,
        relative_to_best_threshold: f64,
        and_threshold: f64,
    ) -> Self {
        let mut provider = Self {
            triangle_relations: calculate_triangle_relations(log),
            dependency_threshold,
            positive_observations_threshold,
            relative_to_best_threshold,
            provider,
            dependency_relations: DependencyRelations::new(),
            and_threshold,
        };

        provider.initialize_dependency_relations();
        provider
    }

    fn initialize_dependency_relations(&mut self) {
        let mut relations = HashMap::<String, Vec<(String, f64)>>::new();
        for first_class in self.provider.log_info().all_event_classes() {
            for second_class in self.provider.log_info().all_event_classes() {
                let second_follows_first = self.get_directly_follows_count(first_class, second_class);
                if second_follows_first < self.positive_observations_threshold {
                    continue;
                }

                let measure = self.calculate_dependency_measure(first_class, second_class);
                if measure <= self.dependency_threshold {
                    continue;
                }

                if let Some(values) = relations.get_mut(first_class) {
                    values.push((second_class.to_owned(), measure));
                } else {
                    relations.insert(first_class.to_owned(), vec![(second_class.to_owned(), measure)]);
                }
            }
        }

        for key in self.provider.log_info().all_event_classes() {
            if let Some(values) = relations.get_mut(key.as_str()) {
                let best_value = values.iter().max_by(|first, second| first.1.total_cmp(&second.1)).unwrap().1;

                let min_value = best_value * (1.0 - self.relative_to_best_threshold);
                for i in (0..values.len()).rev() {
                    if values[i].1 < min_value {
                        values.remove(i);
                    }
                }
            }
        }

        for (key, values) in relations {
            let mut map = HashMap::new();
            for (second_key, value) in values {
                map.insert(second_key, value);
            }

            self.dependency_relations.insert(key, map);
        }
    }

    pub fn dependency_relation(&self, first: &str, second: &str) -> bool {
        if let Some(values) = self.dependency_relations.get(first) {
            values.contains_key(second)
        } else {
            false
        }
    }

    pub fn and_or_xor_relation(&self, a: &str, b: &str, c: &str) -> AndOrXorRelation {
        let b_c = self.get_directly_follows_count(b, c) as f64;
        let c_b = self.get_directly_follows_count(c, b) as f64;
        let a_b = self.get_directly_follows_count(a, b) as f64;
        let a_c = self.get_directly_follows_count(a, c) as f64;

        let and_xor_measure = (b_c + c_b) / (a_b + a_c + 1.0);

        if and_xor_measure > self.and_threshold {
            AndOrXorRelation::And
        } else {
            AndOrXorRelation::Xor
        }
    }

    pub fn loop_length_two_relation(&self, first: &str, second: &str) -> bool {
        let a_b = self.triangle_occurences_count(first, second) as f64;
        let b_a = self.triangle_occurences_count(second, first) as f64;

        (a_b + b_a) / (a_b + b_a + 1.0) > self.dependency_threshold
    }

    fn calculate_dependency_measure(&self, first: &str, second: &str) -> f64 {
        let b_follows_a = self.get_directly_follows_count(first, second) as f64;
        let a_follows_b = self.get_directly_follows_count(second, first) as f64;

        if first != second {
            (b_follows_a - a_follows_b) / (b_follows_a + a_follows_b + 1.0)
        } else {
            a_follows_b / (a_follows_b + 1.0)
        }
    }

    fn get_directly_follows_count(&self, first: &str, second: &str) -> usize {
        self.provider
            .log_info()
            .dfg_info()
            .get_directly_follows_count(&(first.to_owned(), second.to_owned()))
    }

    fn triangle_occurences_count(&self, first: &str, second: &str) -> usize {
        if let Some(measure) = self.triangle_relations.get(&(first.to_owned(), second.to_owned())) {
            *measure
        } else {
            0
        }
    }

    pub fn log_info(&self) -> &EventLogInfo {
        self.provider.log_info()
    }
}
