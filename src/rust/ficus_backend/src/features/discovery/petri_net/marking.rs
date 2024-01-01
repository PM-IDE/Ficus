#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
