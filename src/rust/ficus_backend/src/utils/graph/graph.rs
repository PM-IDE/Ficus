use std::{sync::atomic::{AtomicU64, Ordering}, collections::HashMap};

pub struct Graph<TNodeData, TEdgeData> where TNodeData: ToString, TEdgeData: ToString {
    edges: HashMap<u64, GraphEdge<TEdgeData>>,
    nodes: HashMap<u64, GraphNode<TNodeData>>,
}

pub struct GraphEdge<TEdgeData> where TEdgeData: ToString {
    id: u64,
    first_node_id: u64,
    second_node_id: u64,
    data: Option<TEdgeData>,
}

pub struct GraphNode<TNodeData> where TNodeData: ToString {
    id: u64,
    data: Option<TNodeData>
}

impl<TNodeData, TEdgeData> Graph<TNodeData, TEdgeData> where TNodeData: ToString, TEdgeData: ToString  {
    pub fn empty() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashMap::new()
        }
    }

    pub fn all_nodes(&self) -> Vec<&GraphNode<TNodeData>> {
        (&self.nodes).values().into_iter().collect()
    }

    pub fn all_edges(&self) -> Vec<&GraphEdge<TEdgeData>> {
        (&self.edges).values().into_iter().collect()
    }

    pub fn add_node(&mut self, node_data: Option<TNodeData>) -> u64 {
        let new_node = GraphNode::new(node_data);
        let id = *new_node.id();
        self.nodes.insert(*new_node.id(), new_node);

        id
    }

    pub fn connect_nodes(&mut self, first_node_id: &u64, second_node_id: &u64, edge_data: Option<TEdgeData>) -> Option<u64> {
        if let Some(_) = self.nodes.get(first_node_id) {
            if let Some(_) = self.nodes.get(second_node_id) {
                let edge = GraphEdge::new(*first_node_id, *second_node_id, edge_data);
                let id = *edge.id();
                self.edges.insert(*edge.id(), edge);

                return Some(id);
            }
        }

        None
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

impl<TEdgeData> GraphEdge<TEdgeData> where TEdgeData: ToString {
    pub fn new(first_node_id: u64, second_node_id: u64, data: Option<TEdgeData>) -> Self {
        Self {
            first_node_id,
            second_node_id,
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            data
        }
    }

    pub fn data(&self) -> Option<&TEdgeData> {
        self.data.as_ref()
    }

    pub fn id(&self) -> &u64 {
        &self.id
    }
}

impl<TNodeData> GraphNode<TNodeData> where TNodeData: ToString {
    pub fn new(data: Option<TNodeData>) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            data
        }
    }

    pub fn data(&self) -> Option<&TNodeData> {
        self.data.as_ref()
    }

    pub fn id(&self) -> &u64 {
        &self.id
    }
}