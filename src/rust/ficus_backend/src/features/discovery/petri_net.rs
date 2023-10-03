use std::sync::atomic;
use std::sync::atomic::{AtomicU64, Ordering};
use tonic::metadata::VacantEntry;

#[derive(Debug)]
pub struct PetriNet<TData, TArcData> {
    places: Vec<Place>,
    transitions: Vec<Transition<TData, TArcData>>,
    marking: Option<Marking>
}

#[derive(Debug)]
pub struct Place {
    id: u64,
    deleted: bool
}

impl Place {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        Self { id: NEXT_ID.fetch_add(1, Ordering::SeqCst), deleted: false }
    }

    pub fn deleted(&self) -> bool {
        self.deleted.clone()
    }

    pub fn id(&self) -> u64 { self.id }
}

#[derive(Debug)]
pub struct Transition<TData, TArcData> {
    id: u64,
    incoming_arcs: Vec<Arc<TArcData>>,
    outgoing_arcs: Vec<Arc<TArcData>>,
    data: Option<TData>
}

impl<TData, TArcData> Transition<TData, TArcData> {
    pub fn empty(data: Option<TData>) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            incoming_arcs: Vec::new(),
            outgoing_arcs: Vec::new(),
            data
        }
    }

    pub fn add_incoming_arc(&mut self, place_index: usize, data: Option<TArcData>) {
        self.incoming_arcs.push(Arc::new(place_index, data))
    }

    pub fn add_outgoing_arc(&mut self, place_index: usize, data: Option<TArcData>) {
        self.outgoing_arcs.push(Arc::new(place_index, data))
    }

    pub fn remove_incoming_arc(&mut self, arc_index: usize) -> Arc<TArcData> {
        self.incoming_arcs.remove(arc_index)
    }

    pub fn remove_outgoing_arc(&mut self, arc_index: usize) -> Arc<TArcData> {
        self.outgoing_arcs.remove(arc_index)
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn incoming_arcs(&self) -> &Vec<Arc<TArcData>> {
        &self.incoming_arcs
    }

    pub fn outgoing_args(&self) -> &Vec<Arc<TArcData>> {
        &self.outgoing_arcs
    }
}

#[derive(Debug)]
pub struct Arc<TArcData> {
    id: u64,
    place_index: usize,
    data: Option<TArcData>
}

impl<TArcData> Arc<TArcData> {
    pub fn new(place_index: usize, data: Option<TArcData>) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            place_index,
            data
        }
    }

    pub fn id(&self) -> u64 { self.id }

    pub fn place_index(&self) -> usize {
        self.place_index
    }
}

#[derive(Debug)]
pub struct Marking {
    active_places: Vec<SingleMarking>
}

#[derive(Debug)]
pub struct SingleMarking {
    place_index: usize,
    tokens_count: usize
}

impl<TData, TArcData> PetriNet<TData, TArcData> {
    pub fn empty() -> Self {
        Self {
            places: Vec::new(),
            transitions: Vec::new(),
            marking: None
        }
    }

    pub fn add_place(&mut self, place: Place) -> usize {
        self.places.push(place);
        self.places.len() - 1
    }

    pub fn all_places(&self) -> &Vec<Place> {
        &self.places
    }

    pub fn non_deleted_places(&self) -> Vec<&Place> {
        self.places.iter().filter(|place| !place.deleted()).collect()
    }

    pub fn all_transitions(&self) -> &Vec<Transition<TData, TArcData>> {
        &self.transitions
    }

    pub fn delete_transition(&mut self, index: usize) -> Transition<TData, TArcData> {
        self.transitions.remove(index)
    }

    pub fn add_transition(&mut self, transition: Transition<TData, TArcData>) -> usize {
        self.transitions.push(transition);
        self.transitions.len() - 1
    }

    pub fn connect_place_to_transition(&mut self, from_place_index: usize, to_transition_index: usize, arc_data: Option<TArcData>) {
        self.transitions[to_transition_index].add_incoming_arc(from_place_index, arc_data);
    }

    pub fn connect_transition_to_place(&mut self, from_transition_index: usize, to_place_index: usize, arc_data: Option<TArcData>) {
        self.transitions[from_transition_index].add_outgoing_arc(to_place_index, arc_data);
    }

    pub fn place(&self, index: usize) -> &Place {
        self.places.get(index).as_ref().unwrap()
    }
}
