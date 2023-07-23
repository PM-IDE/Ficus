use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use super::node::Node;
use super::suffix_tree_slice::SuffixTreeSlice;

pub struct SuffixTree<'a, TElement>
where
    TElement: Eq + Hash + Copy,
{
    slice: SuffixTreeSlice<'a, TElement>,
    nodes: Vec<Node<TElement>>,
}

impl<'a, TElement> SuffixTree<'a, TElement>
where
    TElement: Eq + Hash + Copy,
{
    //docs: http://vis.usal.es/rodrigo/documentos/bioinfo/avanzada/soluciones/12-suffixtrees2.pdf
    pub fn calculate_maximal_repeats(&self) -> HashSet<(usize, usize)> {
        let mut maximal_repeats = HashSet::new();
        let mut nodes_to_awc = HashMap::new();
        self.dfs(0, 0, &mut nodes_to_awc, &mut maximal_repeats);

        maximal_repeats
    }

    fn dfs(
        &self,
        index: usize,
        mut suffix_length: usize,
        nodes_to_awc: &mut HashMap<usize, HashSet<Option<TElement>>>,
        maximal_repeats: &mut HashSet<(usize, usize)>,
    ) {
        let node = self.nodes.get(index).unwrap();
        suffix_length += node.edge_len();

        if node.is_leaf() {
            let element = if suffix_length + 1 > self.slice.len() {
                None
            } else {
                self.slice.get(self.slice.len() - suffix_length - 1)
            };

            nodes_to_awc.insert(index, HashSet::from_iter(vec![(element)]));
            return;
        }

        let mut child_set = HashSet::new();
        for (_, child_index) in &node.children {
            self.dfs(*child_index, suffix_length, nodes_to_awc, maximal_repeats);
            child_set.extend(nodes_to_awc.get(child_index).unwrap());
        }

        nodes_to_awc.insert(index, child_set);

        if suffix_length > 1 {
            for (_, first_child) in &node.children {
                for (_, second_child) in &node.children {
                    if first_child == second_child {
                        continue;
                    }

                    let first_set = nodes_to_awc.get(first_child).unwrap();
                    let second_set = nodes_to_awc.get(second_child).unwrap();
                    if first_set != second_set {
                        maximal_repeats.insert((node.right - suffix_length, suffix_length));
                    }
                }
            }
        }
    }
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
            slice: SuffixTreeSlice { slice },
            nodes: vec![Node::create_default()],
        }
    }

    pub fn dump_nodes(&self) -> Vec<(usize, usize, Option<usize>, Option<usize>)> {
        (&self.nodes)
            .into_iter()
            .map(|node| (node.left, node.right, node.parent, node.link))
            .collect()
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
                    link: None,
                    parent: Some(mid),
                    children: HashMap::new(),
                });

                self.nodes
                    .get_mut(mid)
                    .unwrap()
                    .update_child(&self.slice.get(pos), leaf_index);

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
                    node_index: current_node.go(&self.slice.get(left)),
                    pos: 0,
                };

                if current_state.node_index.is_none() {
                    return current_state;
                }

                continue;
            }

            if !self.slice.equals(current_node.left + current_state.pos, left) {
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
        let current_node_left = current_node.left;
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

        self.nodes[current_node_parent.unwrap()].update_child(&self.slice.get(current_node_left), index);

        let element = self.slice.get(current_node_left + current_state.pos);
        self.nodes[index].update_child(&element, current_index);

        self.nodes[current_index].parent = Some(index);
        self.nodes[current_index].left += current_state.pos;

        Some(index)
    }

    fn get_link(&mut self, node_index: usize) -> usize {
        let node = self.nodes.get_mut(node_index).unwrap();
        let node_parent = node.parent;
        let node_right = node.right;
        let node_left = node.left;

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
