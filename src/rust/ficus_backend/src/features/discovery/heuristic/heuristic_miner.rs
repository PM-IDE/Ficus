use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::providers::alpha_provider::{AlphaRelationsProvider, DefaultAlphaRelationsProvider};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use std::collections::HashMap;
use crate::features::discovery::alpha::providers::alpha_plus_provider::calculate_triangle_relations;

pub fn discover_petri_net_heuristic(
    log: &impl EventLog,
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
) -> DefaultPetriNet {
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let provider = DefaultAlphaRelationsProvider::new(&info);
    let provider = HeuristicMinerMeasureProvider::new(
        log,
        provider,
        dependency_threshold,
        positive_observations_threshold,
        relative_to_best_threshold,
    );

    let mut petri_net = DefaultPetriNet::empty();

    let mut classes_to_ids = HashMap::new();
    for class in provider.log_info().all_event_classes() {
        let id = petri_net.add_transition(Transition::empty(class.to_owned(), Some(class.to_owned())));
        classes_to_ids.insert(class.to_owned(), id);
    }

    for first_class in provider.log_info().all_event_classes() {
        for second_class in provider.log_info().all_event_classes() {
            if provider.dependency_relation(first_class, second_class) {
                let first_id = classes_to_ids.get(first_class).unwrap();
                let second_id = classes_to_ids.get(second_class).unwrap();

                let place_id = petri_net.add_place(Place::with_name(format!("{}, {}", first_class, second_class)));
                petri_net.connect_place_to_transition(&place_id, second_id, None);
                petri_net.connect_transition_to_place(first_id, &place_id, None);
            }
        }
    }

    petri_net
}

pub(crate) struct HeuristicMinerMeasureProvider<'a> {
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
    triangle_relations: HashMap<(String, String), usize>,
    provider: DefaultAlphaRelationsProvider<'a>,
}

impl<'a> HeuristicMinerMeasureProvider<'a> {
    pub fn new(
        log: &impl EventLog,
        provider: DefaultAlphaRelationsProvider<'a>,
        dependency_threshold: f64,
        positive_observations_threshold: usize,
        relative_to_best_threshold: f64,
    ) -> Self {
        Self {
            triangle_relations: calculate_triangle_relations(log),
            dependency_threshold,
            positive_observations_threshold,
            relative_to_best_threshold,
            provider,
        }
    }

    pub fn dependency_relation(&self, first: &str, second: &str) -> bool {
        let b_follows_a = self
            .provider
            .log_info()
            .dfg_info()
            .get_directly_follows_count(&(first.to_owned(), second.to_owned()));

        let a_follows_b = self
            .provider
            .log_info()
            .dfg_info()
            .get_directly_follows_count(&(second.to_owned(), first.to_owned()));

        let measure = (b_follows_a as f64 - a_follows_b as f64) / (b_follows_a as f64 + a_follows_b as f64 + 1.0);
        measure > self.dependency_threshold
    }

    fn triangle_measure(&self, first: &str, second: &str) -> usize {
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
