use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

use crate::utils::interval_tree::interval::Interval;
use crate::utils::interval_tree::interval_tree::IntervalTree;

use super::node::Node;
use super::suffix_tree_slice::{MultipleWordsSuffixTreeSlice, SuffixTreeSlice};

pub struct SuffixTree<TElement, TSlice>
where
    TElement: Eq + Hash + Copy,
    TSlice: SuffixTreeSlice<TElement>,
{
    pub(super) slice: Rc<Box<TSlice>>,
    pub(super) nodes: Rc<RefCell<Vec<Node<TElement>>>>,
}

impl<TElement, TSlice> SuffixTree<TElement, TSlice>
where
    TElement: Eq + Hash + Copy,
    TSlice: SuffixTreeSlice<TElement>,
{
    //docs: http://vis.usal.es/rodrigo/documentos/bioinfo/avanzada/soluciones/12-suffixtrees2.pdf
    pub fn find_maximal_repeats(&self) -> Vec<(usize, usize)> {
        let mut maximal_repeats = HashSet::new();
        let mut nodes_to_awc = HashMap::new();
        let mut nodes_any_suffix_len = HashMap::new();
        self.dfs_maximal_repeats(0, 0, &mut nodes_to_awc, &mut nodes_any_suffix_len, &mut maximal_repeats);

        let mut maximal_repeats: Vec<(usize, usize)> = maximal_repeats.into_iter().collect();
        maximal_repeats.sort();

        let mut seen = HashSet::new();
        let mut filtered_repeats = Vec::new();
        for repeat in &maximal_repeats {
            let sub_slice = self.slice.sub_slice(repeat.0, repeat.1);
            if seen.contains(sub_slice) {
                continue;
            }

            seen.insert(sub_slice);
            filtered_repeats.push(*repeat);
        }

        filtered_repeats
    }

    pub fn find_super_maximal_repeats(&self) -> Vec<(usize, usize)> {
        let (maximal_repeats, maximal_repeats_tree) = self.find_maximal_repeats_and_build_suffix_tree();

        let mut super_maximal_repeats = Vec::new();
        for repeat in maximal_repeats {
            let sub_slice = self.slice.sub_slice(repeat.0, repeat.1);
            let patterns = maximal_repeats_tree.find_patterns(sub_slice);

            if let Some(patterns) = patterns {
                if patterns.len() == 1 {
                    super_maximal_repeats.push((repeat.0, repeat.1));
                }
            }
        }

        super_maximal_repeats
    }

    fn find_maximal_repeats_and_build_suffix_tree(
        &self,
    ) -> (
        Vec<(usize, usize)>,
        SuffixTree<TElement, MultipleWordsSuffixTreeSlice<TElement>>,
    ) {
        let found_maximal_repeats = self.find_maximal_repeats();
        let mut slices = Vec::new();
        for repeat in &found_maximal_repeats {
            let sub_slice = self.slice.sub_slice(repeat.0, repeat.1);
            slices.push(sub_slice);
        }

        let slice = MultipleWordsSuffixTreeSlice::new(slices);
        let mut suffix_tree = SuffixTree::new(Rc::new(Box::new(slice)));
        suffix_tree.build_tree();

        (found_maximal_repeats, suffix_tree)
    }

    pub fn find_near_super_maximal_repeats(&self) -> Vec<(usize, usize)> {
        let (maximal_repeats, maximal_repeats_tree) = self.find_maximal_repeats_and_build_suffix_tree();

        let mut intervals = vec![];
        for index in 0..maximal_repeats.len() {
            let repeat = maximal_repeats[index];
            let repeat_positions = maximal_repeats_tree.find_patterns(self.slice.sub_slice(repeat.0, repeat.1));

            if let Some(repeat_positions) = repeat_positions {
                for repeat_pos in repeat_positions {
                    intervals.push(Interval::new_with_data(repeat_pos.0, repeat_pos.1, Some(index)));
                }
            }
        }

        let mut visited = HashSet::new();
        let mut near_super_maximal_repeats = vec![];
        let mut interval_tree = IntervalTree::new(&intervals, |left, right| *left..*right);

        intervals.sort_by(|first, second| (second.right - second.left).cmp(&(first.right - first.left)));
        for interval in intervals {
            if visited.contains(&interval) {
                continue;
            }

            visited.insert(interval);

            let associated_repeat = maximal_repeats.get(interval.data.unwrap()).unwrap();
            near_super_maximal_repeats.push((associated_repeat.0, associated_repeat.1));

            for envelope in interval_tree.search_envelopes(interval.left, interval.right) {
                visited.insert(envelope);
            }
        }

        near_super_maximal_repeats.sort();

        near_super_maximal_repeats
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

    fn dfs_maximal_repeats(
        &self,
        index: usize,
        mut suffix_length: usize,
        nodes_to_awc: &mut HashMap<usize, HashSet<Option<TElement>>>,
        nodes_to_any_suffix_len: &mut HashMap<usize, usize>,
        maximal_repeats: &mut HashSet<(usize, usize)>,
    ) {
        let nodes = self.nodes.borrow();
        let node = nodes.get(index).unwrap();
        suffix_length += node.edge_len();

        if node.is_leaf() {
            let element = if suffix_length + 1 > self.slice.len() {
                None
            } else {
                self.slice.get(self.slice.len() - suffix_length - 1)
            };

            nodes_to_any_suffix_len.insert(index, suffix_length);
            nodes_to_awc.insert(index, HashSet::from_iter(vec![(element)]));
            return;
        }

        let mut child_set = HashSet::new();
        for (_, child_index) in &node.children {
            self.dfs_maximal_repeats(
                *child_index,
                suffix_length,
                nodes_to_awc,
                nodes_to_any_suffix_len,
                maximal_repeats,
            );

            child_set.extend(nodes_to_awc.get(child_index).unwrap());
        }

        nodes_to_awc.insert(index, child_set);

        let mut children: Vec<&usize> = node.children.values().into_iter().collect();
        children.sort();

        let child_suffix_len = nodes_to_any_suffix_len[children[0]];
        nodes_to_any_suffix_len.insert(index, child_suffix_len);

        if suffix_length != 0 {
            for first_child in &children {
                for second_child in &children {
                    if first_child == second_child {
                        continue;
                    }

                    let first_set = nodes_to_awc.get(first_child).unwrap();
                    let second_set = nodes_to_awc.get(second_child).unwrap();
                    if first_set != second_set {
                        let first_child_suffix_len = nodes_to_any_suffix_len[first_child];
                        let start = self.slice.len() - first_child_suffix_len;

                        maximal_repeats.insert((start, start + suffix_length));
                    }
                }
            }
        }
    }
}
