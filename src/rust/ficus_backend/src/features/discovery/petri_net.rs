use crate::features::analysis::patterns::entry_points::find_patterns;
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

#[derive(Debug)]
pub struct Place {
    id: u64,
}

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

impl Place {
    pub fn new() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug)]
pub struct Transition<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    id: u64,
    incoming_arcs: Vec<Arc<TArcData>>,
    outgoing_arcs: Vec<Arc<TArcData>>,
    data: Option<TTransitionData>,
}

impl<TTransitionData, TArcData> Transition<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    pub fn empty(data: Option<TTransitionData>) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            incoming_arcs: Vec::new(),
            outgoing_arcs: Vec::new(),
            data,
        }
    }

    pub fn add_incoming_arc(&mut self, place_id: u64, data: Option<TArcData>) {
        self.incoming_arcs.push(Arc::new(place_id, data))
    }

    pub fn add_outgoing_arc(&mut self, place_id: u64, data: Option<TArcData>) {
        self.outgoing_arcs.push(Arc::new(place_id, data))
    }

    pub fn remove_incoming_arc(&mut self, arc_index: usize) -> Arc<TArcData> {
        self.incoming_arcs.remove(arc_index)
    }

    pub fn remove_outgoing_arc(&mut self, arc_index: usize) -> Arc<TArcData> {
        self.outgoing_arcs.remove(arc_index)
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn incoming_arcs(&self) -> &Vec<Arc<TArcData>> {
        &self.incoming_arcs
    }

    pub fn outgoing_arcs(&self) -> &Vec<Arc<TArcData>> {
        &self.outgoing_arcs
    }

    pub fn data(&self) -> Option<&TTransitionData> {
        self.data.as_ref()
    }
}

#[derive(Debug)]
pub struct Arc<TArcData> {
    id: u64,
    place_id: u64,
    data: Option<TArcData>,
}

impl<TArcData> Arc<TArcData> {
    pub fn new(place_id: u64, data: Option<TArcData>) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            place_id,
            data,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn place_id(&self) -> u64 {
        self.place_id
    }
}

#[derive(Debug)]
pub struct Marking {
    active_places: Vec<SingleMarking>,
}

impl Marking {
    pub fn new(single_markings: Vec<SingleMarking>) -> Self {
        Self {
            active_places: single_markings,
        }
    }

    pub fn active_places(&self) -> &Vec<SingleMarking> {
        &self.active_places
    }
}

#[derive(Debug)]
pub struct SingleMarking {
    place_id: u64,
    tokens_count: usize,
}

impl SingleMarking {
    pub fn new(place_id: u64, tokens_count: usize) -> Self {
        Self { place_id, tokens_count }
    }

    pub fn place_id(&self) -> u64 {
        self.place_id
    }
    pub fn tokens_count(&self) -> usize {
        self.tokens_count
    }
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
}
