use crate::utils::hash_utils::compare_based_on_hashes;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

pub struct OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    set: BTreeSet<T>,
}

impl<T> OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    pub fn empty() -> Self {
        Self { set: BTreeSet::new() }
    }

    pub fn new(el: T) -> Self {
        Self {
            set: BTreeSet::from_iter(vec![el]),
        }
    }

    pub fn new_two_elements(first: T, second: T) -> Self {
        Self {
            set: BTreeSet::from_iter(vec![first, second]),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            set: self.set.iter().chain(other.set.iter()).map(|el| el.clone()).collect(),
        }
    }

    pub fn set(&self) -> &BTreeSet<T> {
        &self.set
    }

    pub fn insert(&mut self, item: T) {
        self.set.insert(item);
    }
}

impl<T> Hash for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for el in &self.set {
            el.hash(state);
        }
    }
}

impl<T> PartialEq for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        compare_based_on_hashes(self, other)
    }
}

impl<T> Eq for OneSet<T> where T: Hash + Eq + Ord + Clone {}

impl<T> Clone for OneSet<T>
where
    T: Hash + Eq + Ord + Clone,
{
    fn clone(&self) -> Self {
        Self { set: self.set.clone() }
    }
}
