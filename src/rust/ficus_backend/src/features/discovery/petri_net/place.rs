use crate::features::discovery::petri_net::ids::next_id;

const EMPTY_PLACE_NAME: &'static str = "EmptyPlace";

#[derive(Debug)]
pub struct Place {
    id: u64,
    name: String,
}

impl Place {
    pub fn empty() -> Self {
        Self {
            id: next_id(),
            name: EMPTY_PLACE_NAME.to_owned(),
        }
    }

    pub fn with_name(name: String) -> Self {
        Self { id: next_id(), name }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
