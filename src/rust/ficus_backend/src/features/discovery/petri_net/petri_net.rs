use crate::features::discovery::petri_net::marking::Marking;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub type DefaultPetriNet = PetriNet<String, ()>;

#[derive(Debug)]
pub struct PetriNet<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    places: HashMap<u64, Place>,
    transitions: HashMap<u64, Transition<TTransitionData, TArcData>>,
    initial_marking: Option<Marking>,
    final_marking: Option<Marking>,
}

impl<TTransitionData, TArcData> PetriNet<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    pub fn empty() -> Self {
        Self {
            places: HashMap::new(),
            transitions: HashMap::new(),
            initial_marking: None,
            final_marking: None,
        }
    }

    pub fn add_place(&mut self, place: Place) -> u64 {
        let id = place.id();
        self.places.insert(place.id(), place);
        id
    }

    pub fn all_places(&self) -> Vec<&Place> {
        self.places.values().into_iter().collect()
    }

    pub fn all_transitions(&self) -> Vec<&Transition<TTransitionData, TArcData>> {
        self.transitions.values().into_iter().collect()
    }

    pub fn delete_transition(&mut self, id: &u64) -> Option<Transition<TTransitionData, TArcData>> {
        self.transitions.remove(id)
    }

    pub fn add_transition(&mut self, transition: Transition<TTransitionData, TArcData>) -> u64 {
        let id = transition.id();
        self.transitions.insert(transition.id(), transition);
        id
    }

    pub fn connect_place_to_transition(
        &mut self,
        from_place_id: u64,
        to_transition_index: u64,
        arc_data: Option<TArcData>,
    ) {
        self.transitions
            .get_mut(&to_transition_index)
            .unwrap()
            .add_incoming_arc(from_place_id, arc_data);
    }

    pub fn connect_transition_to_place(
        &mut self,
        from_transition_id: u64,
        to_place_index: u64,
        arc_data: Option<TArcData>,
    ) {
        self.transitions
            .get_mut(&from_transition_id)
            .unwrap()
            .add_outgoing_arc(to_place_index, arc_data);
    }

    pub fn place(&self, id: &u64) -> &Place {
        self.places.get(id).as_ref().unwrap()
    }

    pub fn set_initial_marking(&mut self, marking: Marking) {
        self.initial_marking = Some(marking)
    }

    pub fn set_final_marking(&mut self, marking: Marking) {
        self.final_marking = Some(marking)
    }

    pub fn initial_marking(&self) -> Option<&Marking> {
        self.initial_marking.as_ref()
    }

    pub fn final_marking(&self) -> Option<&Marking> {
        self.final_marking.as_ref()
    }

    pub fn find_place_id_by_name(&self, name: &str) -> Option<u64> {
        for place in self.places.values() {
            if place.name() == name {
                return Some(place.id());
            }
        }

        None
    }

    pub fn find_transition_by_name(&self, name: &str) -> Option<&Transition<TTransitionData, TArcData>> {
        for transition in self.transitions.values() {
            if transition.name() == name {
                return Some(transition);
            }
        }

        None
    }
}
