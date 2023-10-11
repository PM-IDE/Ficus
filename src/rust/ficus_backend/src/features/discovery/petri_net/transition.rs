use crate::features::discovery::petri_net::arc::Arc;
use crate::features::discovery::petri_net::ids::next_id;

#[derive(Debug)]
pub struct Transition<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    id: u64,
    name: String,
    incoming_arcs: Vec<Arc<TArcData>>,
    outgoing_arcs: Vec<Arc<TArcData>>,
    data: Option<TTransitionData>,
}

impl<TTransitionData, TArcData> Transition<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    pub fn empty(name: String, data: Option<TTransitionData>) -> Self {
        Self {
            id: next_id(),
            name,
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

    pub fn name(&self) -> &String {
        &self.name
    }
}