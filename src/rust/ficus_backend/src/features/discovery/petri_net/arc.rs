use crate::features::discovery::petri_net::ids::next_id;

#[derive(Debug)]
pub struct Arc<TArcData> {
    id: u64,
    place_id: u64,
    data: Option<TArcData>,
}

impl<TArcData> Arc<TArcData> {
    pub fn new(place_id: u64, data: Option<TArcData>) -> Self {
        Self {
            id: next_id(),
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
