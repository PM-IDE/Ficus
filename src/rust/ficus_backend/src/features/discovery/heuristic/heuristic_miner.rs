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

type DependencyRelations = HashMap<String, HashMap<String, f64>>;

pub(crate) struct HeuristicMinerMeasureProvider<'a> {
    dependency_threshold: f64,
    positive_observations_threshold: usize,
    relative_to_best_threshold: f64,
    triangle_relations: HashMap<(String, String), usize>,
    provider: DefaultAlphaRelationsProvider<'a>,
    dependency_relations: DependencyRelations
}

impl<'a> HeuristicMinerMeasureProvider<'a> {
    pub fn new(
        log: &impl EventLog,
        provider: DefaultAlphaRelationsProvider<'a>,
        dependency_threshold: f64,
        positive_observations_threshold: usize,
        relative_to_best_threshold: f64,
    ) -> Self {
        let mut provider = Self {
            triangle_relations: calculate_triangle_relations(log),
            dependency_threshold,
            positive_observations_threshold,
            relative_to_best_threshold,
            provider,
            dependency_relations: DependencyRelations::new()
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
                let best_value = values.iter().max_by(|first, second| {
                    first.1.total_cmp(&second.1)
                }).unwrap().1;

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
        self.provider.log_info().dfg_info().get_directly_follows_count(&(first.to_owned(), second.to_owned()))
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
