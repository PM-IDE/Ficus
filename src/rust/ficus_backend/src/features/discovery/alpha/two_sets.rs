use crate::utils::hash_utils::compare_based_on_hashes;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

pub struct TwoSets<T>
where
    T: Hash + Eq + ToString + Ord + Copy,
{
    first_set: BTreeSet<T>,
    second_set: BTreeSet<T>,
}

impl<T> TwoSets<T>
where
    T: Hash + Eq + ToString + Ord + Copy,
{
    pub fn new(first_set: BTreeSet<T>, second_set: BTreeSet<T>) -> Self {
        Self {
            first_set,
            second_set
        }
    }

    pub fn new_one_element(first: T, second: T) -> Self {
        Self {
            first_set: BTreeSet::from_iter(vec![first]),
            second_set: BTreeSet::from_iter(vec![second]),
        }
    }

    pub fn is_full_subset(&self, other: &Self) -> bool {
        self.first_set.is_subset(&other.first_set) && self.second_set.is_subset(&other.second_set)
    }

    pub fn merge(&self, other: &TwoSets<T>) -> Self {
        Self {
            first_set: self.first_set.iter().chain(other.first_set.iter()).map(|c| *c).collect(),
            second_set: self.second_set.iter().chain(other.second_set.iter()).map(|c| *c).collect(),
        }
    }

    pub fn first_set(&self) -> &BTreeSet<T> {
        &self.first_set
    }

    pub fn second_set(&self) -> &BTreeSet<T> {
        &self.second_set
    }
}

impl<T> Hash for TwoSets<T>
where
    T: Hash + Eq + ToString + Ord + Copy,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.first_set {
            state.write(item.to_string().as_bytes());
        }

        for item in &self.second_set {
            state.write(item.to_string().as_bytes());
        }
    }
}

impl<T> PartialEq for TwoSets<T>
where
    T: Hash + Eq + ToString + Ord + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        compare_based_on_hashes(self, other)
    }
}

impl<T> Eq for TwoSets<T> where T: Hash + Eq + ToString + Ord + Copy {}

impl<T> Clone for TwoSets<T>
where
    T: Hash + Eq + ToString + Ord + Copy,
{
    fn clone(&self) -> Self {
        Self {
            first_set: self.first_set.iter().map(|c| *c).collect(),
            second_set: self.second_set.iter().map(|c| *c).collect(),
        }
    }
}

impl<T> ToString for TwoSets<T>
where
    T: Hash + Eq + ToString + Ord + Copy,
{
    fn to_string(&self) -> String {
        let mut repr = String::new();
        repr.push('(');

        let mut write_set = |set: &BTreeSet<T>, repr: &mut String| {
            repr.push('{');
            for item in set {
                repr.push_str(item.to_string().as_str());
                repr.push(',');
            }

            if set.len() > 0 {
                repr.remove(repr.len() - 1);
            }

            repr.push('}');
        };

        write_set(&self.first_set, &mut repr);

        repr.push_str(", ");

        write_set(&self.second_set, &mut repr);

        repr.push(')');
        repr
    }
}
