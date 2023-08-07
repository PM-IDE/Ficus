use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::utils::hash_map_utils::{compare_maps_by_keys, increase_in_map_by};

use super::node::Node;
use super::suffix_tree_slice::{MultipleWordsSuffixTreeSlice, SuffixTreeSlice};

pub struct SuffixTree<'a, TElement>
where
    TElement: Eq + Hash + Copy,
{
    pub(super) slice: &'a dyn SuffixTreeSlice<TElement>,
    pub(super) nodes: Rc<RefCell<Vec<Node<TElement>>>>,
}

enum RepeatType {
    MaximalRepeat,
    SuperMaximalRepeat,
    NearSuperMaximalRepeat,
}

impl<'a, TElement> SuffixTree<'a, TElement>
where
    TElement: Eq + Hash + Copy
{
    //docs: http://vis.usal.es/rodrigo/documentos/bioinfo/avanzada/soluciones/12-suffixtrees2.pdf
    pub fn find_maximal_repeats(&self) -> Vec<(usize, usize)> {
        self.find_repeats(RepeatType::MaximalRepeat)
    }

    fn find_repeats(&self, repeat_type: RepeatType) -> Vec<(usize, usize)> {
        let mut maximal_repeats = HashSet::new();
        let mut nodes_to_awc = HashMap::new();
        let mut nodes_any_suffix_len = HashMap::new();
        self.dfs_repeats(
            &repeat_type,
            0,
            0,
            &mut nodes_to_awc,
            &mut nodes_any_suffix_len,
            &mut maximal_repeats,
        );

        let mut maximal_repeats: Vec<(usize, usize)> = maximal_repeats.into_iter().collect();
        maximal_repeats.sort();

        let mut seen = HashSet::new();
        let mut filtered_repeats = Vec::new();
        for repeat in &maximal_repeats {
            if let Some(sub_slice) = self.slice.sub_slice(repeat.0, repeat.1) {
                if seen.contains(sub_slice) {
                    continue;
                }

                seen.insert(sub_slice);
                filtered_repeats.push(*repeat);
            }
        }

        filtered_repeats
    }

    pub fn find_super_maximal_repeats(&self) -> Vec<(usize, usize)> {
        self.find_repeats(RepeatType::SuperMaximalRepeat)
    }

    pub fn find_near_super_maximal_repeats(&self) -> Vec<(usize, usize)> {
        self.find_repeats(RepeatType::NearSuperMaximalRepeat)
    }

    pub fn find_patterns(&self, pattern: &[TElement]) -> Option<Vec<(usize, usize)>> {
        let mut current_node_index = 0;
        let mut pattern_index = 0;
        let mut suffix_length = 0;

        let nodes = self.nodes.borrow();

        loop {
            if pattern_index == pattern.len() {
                break;
            }

            let current_node = nodes.get(current_node_index).unwrap();
            if !current_node.children.contains_key(&Some(pattern[pattern_index])) {
                return None;
            }

            let child_index = current_node.children.get(&Some(pattern[pattern_index])).unwrap();
            let child_node = nodes.get(*child_index).unwrap();

            for i in child_node.left..child_node.right {
                if pattern_index == pattern.len() {
                    break;
                }

                let slice_element = self.slice.get(i);
                if slice_element.is_none() || slice_element.unwrap() != pattern[pattern_index] {
                    return None;
                }

                pattern_index += 1;
            }

            current_node_index = *child_index;
            suffix_length += child_node.edge_len();
        }

        let mut patterns = Vec::new();

        suffix_length -= nodes.get(current_node_index).unwrap().edge_len();
        self.dfs_pattern_search(current_node_index, &mut patterns, pattern.len(), suffix_length);

        patterns.sort();

        Some(patterns)
    }

    fn dfs_pattern_search(
        &self,
        index: usize,
        patterns: &mut Vec<(usize, usize)>,
        pattern_length: usize,
        mut suffix_length: usize,
    ) {
        let nodes = self.nodes.borrow();
        let node = nodes.get(index).unwrap();
        suffix_length += node.edge_len();

        if node.is_leaf() {
            let left = self.slice.len() - suffix_length;
            patterns.push((left, left + pattern_length));

            return;
        }

        for (_, child_node_index) in &node.children {
            self.dfs_pattern_search(*child_node_index, patterns, pattern_length, suffix_length);
        }
    }

    fn add_repeats(
        &self,
        repeat_type: &RepeatType,
        node_index: &usize,
        children: &Vec<&usize>,
        suffix_length: usize,
        nodes_to_awc: &HashMap<usize, HashMap<Option<TElement>, usize>>,
        nodes_to_any_suffix_len: &HashMap<usize, usize>,
        repeats: &mut HashSet<(usize, usize)>,
    ) {
        match repeat_type {
            RepeatType::MaximalRepeat => {}
            RepeatType::SuperMaximalRepeat => {
                let nodes = &self.nodes.borrow();

                for (_, child_index) in &nodes.get(*node_index).unwrap().children {
                    let child_node = nodes.get(*child_index).unwrap();
                    if !child_node.is_leaf() {
                        return;
                    }

                    if child_node.is_leaf() {
                        let element = self.get_element_for_suffix(nodes_to_any_suffix_len[child_index]);
                        if element != None && nodes_to_awc[node_index][&element] != 1 {
                            return;
                        }
                    }
                }
            }
            RepeatType::NearSuperMaximalRepeat => {
                let mut witnesses_near_supermaximality = false;
                let nodes = &self.nodes.borrow();
                for (_, child_index) in &nodes.get(*node_index).unwrap().children {
                    let child_node = nodes.get(*child_index).unwrap();
                    if child_node.is_leaf() {
                        let element = self.get_element_for_suffix(nodes_to_any_suffix_len[child_index]);

                        if element != None && nodes_to_awc[node_index][&element] == 1 {
                            witnesses_near_supermaximality = true;
                        }
                    }
                }

                if !witnesses_near_supermaximality {
                    return;
                }
            }
        }

        for first_child in children {
            for second_child in children {
                if first_child == second_child {
                    continue;
                }

                let first_map = nodes_to_awc.get(first_child).unwrap();
                let second_map = nodes_to_awc.get(second_child).unwrap();

                if !compare_maps_by_keys(first_map, second_map, HashSet::from_iter([None])) {
                    let first_child_suffix_len = nodes_to_any_suffix_len[first_child];
                    let start = self.slice.len() - first_child_suffix_len;

                    repeats.insert((start, start + suffix_length));
                }
            }
        }
    }

    fn get_element_for_suffix(&self, suffix_length: usize) -> Option<TElement> {
        if suffix_length + 1 > self.slice.len() {
            None
        } else {
            self.slice.get(self.slice.len() - suffix_length - 1)
        }
    }

    fn dfs_repeats(
        &self,
        repeat_type: &RepeatType,
        index: usize,
        mut suffix_length: usize,
        nodes_to_awc: &mut HashMap<usize, HashMap<Option<TElement>, usize>>,
        nodes_to_any_suffix_len: &mut HashMap<usize, usize>,
        repeats: &mut HashSet<(usize, usize)>,
    ) {
        let nodes = self.nodes.borrow();
        let node = nodes.get(index).unwrap();
        suffix_length += node.edge_len();

        if node.is_leaf() {
            let element = self.get_element_for_suffix(suffix_length);
            nodes_to_any_suffix_len.insert(index, suffix_length);
            nodes_to_awc.insert(index, HashMap::from_iter(vec![(element, 1)]));
            return;
        }

        let mut child_set = HashMap::new();
        for (_, child_index) in &node.children {
            self.dfs_repeats(
                repeat_type,
                *child_index,
                suffix_length,
                nodes_to_awc,
                nodes_to_any_suffix_len,
                repeats,
            );

            for (element, count) in nodes_to_awc.get(child_index).unwrap() {
                increase_in_map_by(&mut child_set, element, *count);
            }
        }

        nodes_to_awc.insert(index, child_set);

        let children: Vec<&usize> = node.children.values().into_iter().collect();

        let child_suffix_len = nodes_to_any_suffix_len[children.iter().min().unwrap()];
        nodes_to_any_suffix_len.insert(index, child_suffix_len);

        if suffix_length != 0 {
            self.add_repeats(
                repeat_type,
                &index,
                &children,
                suffix_length,
                nodes_to_awc,
                nodes_to_any_suffix_len,
                repeats,
            );
        }
    }
}
