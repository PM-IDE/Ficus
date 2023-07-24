use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Interval<TElement>
where
    TElement: PartialEq + Ord + Copy + Hash,
{
    pub left: TElement,
    pub right: TElement,
}

impl<TElement> Hash for Interval<TElement>
where
    TElement: PartialEq + Ord + Copy + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
    }
}

impl<TElement> Interval<TElement>
where
    TElement: PartialEq + Ord + Copy + Hash,
{
    pub fn new(left: TElement, right: TElement) -> Interval<TElement> {
        Interval { left, right }
    }
}

impl<TElement> Interval<TElement>
where
    TElement: PartialEq + Ord + Copy + Hash,
{
    pub fn contains(&self, point: TElement) -> bool {
        self.left <= point && point <= self.right
    }
}

struct Node<TElement>
where
    TElement: PartialEq + Ord + Copy + Hash,
{
    pub left_child: Option<usize>,
    pub right_child: Option<usize>,
    pub center: TElement,
    pub intervals: Vec<Interval<TElement>>,
}

pub struct IntervalTree<TElement>
where
    TElement: PartialEq + Ord + Copy + Hash,
{
    nodes: Vec<Node<TElement>>,
}

enum ChildOrientation {
    Left,
    Right,
}

impl<TElement> IntervalTree<TElement>
where
    TElement: Eq + Ord + Copy + Hash,
{
    pub fn new(intervals: Vec<Interval<TElement>>) -> IntervalTree<TElement> {
        let mut nodes: Vec<Node<TElement>> = vec![];
        let mut queue: VecDeque<(Option<(usize, ChildOrientation)>, Vec<Interval<TElement>>)> = VecDeque::new();
        queue.push_back((None, intervals));

        while !queue.is_empty() {
            let (parent_child, mut current_intervals) = queue.pop_front().unwrap();
            current_intervals.sort_by(|first, second| first.left.cmp(&second.left));

            let center = current_intervals[current_intervals.len() / 2].left;
            let mut left_intervals = vec![];
            let mut right_intervals = vec![];
            let mut node_intervals = vec![];

            for interval in &current_intervals {
                if interval.right < center {
                    left_intervals.push(*interval);
                } else if interval.left > center {
                    right_intervals.push(*interval);
                } else {
                    node_intervals.push(*interval);
                }
            }

            let node = Node {
                left_child: None,
                right_child: None,
                center,
                intervals: node_intervals,
            };
            let node_index = nodes.len();

            if let Some((parent, orientation)) = parent_child {
                match orientation {
                    ChildOrientation::Left => nodes[parent].left_child = Some(node_index),
                    ChildOrientation::Right => nodes[parent].right_child = Some(node_index),
                }
            }

            nodes.push(node);
            if left_intervals.len() > 0 {
                queue.push_back((Some((node_index, ChildOrientation::Left)), left_intervals));
            }

            if right_intervals.len() > 0 {
                queue.push_back((Some((node_index, ChildOrientation::Right)), right_intervals));
            }
        }

        IntervalTree { nodes }
    }

    pub fn search_point(&self, point: TElement) -> Vec<Interval<TElement>> {
        let mut result = HashSet::new();
        self.search_internal(0, point, &mut result);
        result.into_iter().collect()
    }

    pub fn search_interval<TIterator>(&self, interval_iter: TIterator) -> Vec<Interval<TElement>>
    where
        TIterator: Iterator<Item = TElement>,
    {
        let mut result = HashSet::new();
        for element in interval_iter {
            self.search_internal(0, element, &mut result);
        }

        result.into_iter().collect()
    }

    pub fn search_internal(&self, node_index: usize, point: TElement, result: &mut HashSet<Interval<TElement>>) {
        let node = &self.nodes[node_index];
        for interval in &node.intervals {
            if interval.contains(point) {
                result.insert(*interval);
            }

            if let Some(left_child) = node.left_child {
                if point < node.center {
                    self.search_internal(left_child, point, result);
                }
            }

            if let Some(right_child) = node.right_child {
                if point > node.center {
                    self.search_internal(right_child, point, result);
                }
            }
        }
    }

    pub fn dump_nodes(&self) -> Vec<(Option<usize>, Option<usize>, TElement, Vec<Interval<TElement>>)> {
        let mut nodes = vec![];
        for node in &self.nodes {
            nodes.push((node.left_child, node.right_child, node.center, node.intervals.to_vec()));
        }

        nodes
    }
}
