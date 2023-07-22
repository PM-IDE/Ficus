use std::collections::HashMap;
use std::hash::Hash;

struct Node<TElement>
where
    TElement: Eq + PartialEq + Hash + Copy,
{
    left: usize,
    right: usize,
    link: Option<usize>,
    parent: Option<usize>,
    children: HashMap<TElement, usize>,
}

impl<TElement> Node<TElement>
where
    TElement: Eq + PartialEq + Hash + Copy,
{
    pub fn create_default() -> Self {
        Self { left: 0, right: 0, link: None, parent: None, children: HashMap::new() }
    }

    fn edge_len(&self) -> usize {
        self.right - self.left
    }

    fn update_child(&mut self, element: &TElement, new_child: usize) {
        if self.children.contains_key(element) {
            *self.children.get_mut(element).unwrap() = new_child;
        } else {
            self.children.insert(*element, new_child);
        }
    }

    fn update_parent(&mut self, new_parent: usize) {
        self.parent = Some(new_parent);
    }

    fn go(&mut self, element: &TElement) -> Option<usize> {
        match self.children.get(element) {
            Some(next) => Some(*next),
            None => None,
        }
    }

    fn left(&self) -> usize {
        self.left
    }

    fn right(&self) -> usize {
        self.right
    }
}

pub struct SuffixTree<'a, TElement>
where
    TElement: Eq + PartialEq + Hash + Copy,
{
    slice: &'a [TElement],
    nodes: Vec<Node<TElement>>,
}

#[derive(Copy, Clone)]
struct BuildState {
    pub pos: usize,
    pub node_index: Option<usize>,
}

impl<'a, TElement> SuffixTree<'a, TElement>
where
    TElement: Eq + PartialEq + Hash + Copy,
{
    pub fn new(slice: &'a [TElement]) -> Self {
        Self {
            slice,
            nodes: vec![Node::create_default()],
        }
    }

    pub fn build_tree(&mut self) {
        let mut state = BuildState {
            pos: 0,
            node_index: Some(0),
        };

        for pos in 0..self.slice.len() {
            loop {
                let next_state = self.go(state, pos, pos + 1);
                if next_state.node_index.is_some() {
                    state = next_state;
                    break;
                }

                let mid = self.split(state).unwrap();
                let leaf_index = self.nodes.len();
                self.nodes.push(Node {
                    left: pos,
                    right: self.slice.len(),
                    link: Some(mid),
                    parent: None,
                    children: HashMap::new(),
                });

                self.nodes
                    .get_mut(mid)
                    .unwrap()
                    .update_child(self.slice.get(pos).unwrap(), leaf_index);

                state.node_index = Some(self.get_link(mid));
                state.pos = self.nodes.get(state.node_index.unwrap()).unwrap().edge_len();

                if mid == 0 {
                    break;
                }
            }
        }
    }

    fn go(&mut self, mut current_state: BuildState, mut left: usize, right: usize) -> BuildState {
        while left < right {
            let current_node = self.nodes.get_mut(current_state.node_index.unwrap()).unwrap();
            if current_state.pos == current_node.edge_len() {
                current_state = BuildState {
                    node_index: current_node.go(self.slice.get(left).unwrap()),
                    pos: 0,
                };

                if current_state.node_index.is_none() {
                    return current_state;
                }

                continue;
            }

            if self.slice[current_node.left() + current_state.pos] != self.slice[left] {
                return BuildState {
                    node_index: None,
                    pos: 0,
                };
            }

            let current_interval_len = right - left;
            let diff = current_node.edge_len() - current_state.pos;

            if current_interval_len < diff {
                return BuildState {
                    node_index: current_state.node_index,
                    pos: current_state.pos + current_interval_len,
                };
            }

            left += diff;
            current_state.pos = current_node.edge_len();
        }

        current_state
    }

    fn split(&mut self, current_state: BuildState) -> Option<usize> {
        let current_index = current_state.node_index.unwrap();
        let current_node = self.nodes.get(current_index).unwrap();
        let current_node_left = current_node.left();
        let current_node_parent = current_node.parent;

        if current_state.pos == current_node.edge_len() {
            return Some(current_index);
        }

        if current_state.pos == 0 {
            return current_node_parent;
        }

        let index = self.nodes.len();
        let new_node = Node {
            parent: current_node_parent,
            left: current_node_left,
            right: current_node_left + current_state.pos,
            children: HashMap::new(),
            link: None,
        };

        self.nodes.push(new_node);

        self.nodes[current_node_parent.unwrap()].update_child(self.slice.get(current_node_left).unwrap(), index);

        let element = self.slice.get(current_node_left + current_state.pos).unwrap();
        self.nodes[index].update_child(element, current_index);
        
        self.nodes[current_index].update_parent(index);
        self.nodes[current_index].left += current_state.pos;

        Some(index)
    }

    fn get_link(&mut self, node_index: usize) -> usize {
        let node = self.nodes.get_mut(node_index).unwrap();
        let node_parent = node.parent;
        let node_right = node.right();
        let node_left = node.left();

        if node.link.is_some() {
            return node.link.unwrap();
        }

        if node_parent.is_none() {
            return 0usize;
        }

        let to = self.get_link(node_parent.unwrap());

        let state = BuildState {
            node_index: Some(to),
            pos: self.nodes[to].edge_len(),
        };

        let left = node_left + (if node_parent.unwrap() == 0 { 1 } else { 0 });
        let next = self.go(state, left, node_right);
        let link = self.split(next);

        self.nodes.get_mut(node_index).unwrap().link = link;

        link.unwrap()
    }
}
