#[derive(Debug)]
pub struct PetriNet<TData, TArcData> {
    places: Vec<Place>,
    transitions: Vec<Transition<TData, TArcData>>,
    marking: Option<Marking>
}

#[derive(Debug)]
pub struct Place {
    deleted: bool
}

impl Place {
    pub fn new() -> Self {
        Self { deleted: false }
    }

    pub fn deleted(&self) -> bool {
        self.deleted.clone()
    }
}

#[derive(Debug)]
pub struct Transition<TData, TArcData> {
    incoming_arcs: Vec<Arc<TArcData>>,
    outgoing_arcs: Vec<Arc<TArcData>>,
    data: Option<TData>
}

impl<TData, TArcData> Transition<TData, TArcData> {
    pub fn empty(data: Option<TData>) -> Self {
        Self {
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
}

#[derive(Debug)]
pub struct Arc<TArcData> {
    place_index: usize,
    data: Option<TArcData>
}

impl<TArcData> Arc<TArcData> {
    pub fn new(place_index: usize, data: Option<TArcData>) -> Self {
        Self {
            place_index,
            data
        }
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
}
