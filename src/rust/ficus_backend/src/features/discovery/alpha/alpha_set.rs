use crate::features::discovery::alpha::provider::{AlphaRelationsProvider, DefaultAlphaRelationsProvider};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct AlphaSet {
    left_classes: BTreeSet<String>,
    right_classes: BTreeSet<String>,
}

impl AlphaSet {
    pub fn empty() -> Self {
        Self {
            left_classes: BTreeSet::new(),
            right_classes: BTreeSet::new(),
        }
    }

    pub fn new(left_class: String, right_class: String) -> Self {
        let mut left_classes = BTreeSet::new();
        left_classes.insert(left_class);

        let mut right_classes = BTreeSet::new();
        right_classes.insert(right_class);

        return Self {
            left_classes,
            right_classes,
        };
    }

    pub fn is_left_subset(&self, other: &Self) -> bool {
        self.left_classes.is_subset(&other.left_classes)
    }

    pub fn is_right_subset(&self, other: &Self) -> bool {
        self.right_classes.is_subset(&other.right_classes)
    }

    pub fn is_full_subset(&self, other: &Self) -> bool {
        self.is_left_subset(other) && self.is_right_subset(other)
    }

    pub fn left_classes(&self) -> Vec<&String> {
        (&self.left_classes).iter().collect()
    }

    pub fn right_classes(&self) -> Vec<&String> {
        (&self.right_classes).iter().collect()
    }

    pub fn insert_left_class(&mut self, class: String) {
        self.left_classes.insert(class);
    }

    pub fn insert_right_class(&mut self, class: String) {
        self.right_classes.insert(class);
    }

    pub fn can_extend(&self, other: &Self, provider: &impl AlphaRelationsProvider) -> bool {
        for left_class in self.left_classes.iter().chain(other.left_classes.iter()) {
            for right_class in self.right_classes.iter().chain(other.right_classes.iter()) {
                if !provider.is_in_causal_relation(left_class, right_class) {
                    return false;
                }
            }
        }

        for first_left_class in self.left_classes.iter().chain(other.left_classes.iter()) {
            for second_left_class in self.left_classes.iter().chain(other.left_classes.iter()) {
                if !provider.is_in_unrelated_relation(first_left_class, second_left_class) {
                    return false;
                }
            }
        }

        for first_right_class in self.right_classes.iter().chain(other.right_classes.iter()) {
            for second_right_class in self.right_classes.iter().chain(other.right_classes.iter()) {
                if !provider.is_in_unrelated_relation(first_right_class, second_right_class) {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn extend(&self, other: &Self) -> AlphaSet {
        let mut left_classes = self.left_classes.clone();
        left_classes.extend(other.left_classes.iter().map(|c| c.to_owned()));

        let mut right_classes = self.right_classes.clone();
        right_classes.extend(other.right_classes.iter().map(|c| c.to_owned()));

        Self {
            left_classes,
            right_classes,
        }
    }
}

impl Hash for AlphaSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for class in self.left_classes.iter() {
            state.write(class.as_bytes());
        }

        for class in self.right_classes.iter() {
            state.write(class.as_bytes());
        }
    }
}

impl PartialEq for AlphaSet {
    fn eq(&self, other: &Self) -> bool {
        let mut this_hasher = DefaultHasher::new();
        self.hash(&mut this_hasher);

        let mut other_hasher = DefaultHasher::new();
        other.hash(&mut other_hasher);

        this_hasher.finish() == other_hasher.finish()
    }
}

impl Eq for AlphaSet {}

impl ToString for AlphaSet {
    fn to_string(&self) -> String {
        let mut repr = "[{".to_string();
        for left_class in &self.left_classes {
            repr.push_str(left_class.as_str());
            repr.push(',');
        }

        if self.left_classes.len() > 0 {
            repr.remove(repr.len() - 1);
        }

        repr.push_str("} {");

        for right_class in &self.right_classes {
            repr.push_str(right_class.as_str());
            repr.push(',');
        }

        if self.right_classes.len() > 0 {
            repr.remove(repr.len() - 1);
        }

        repr.push_str("}]");

        repr
    }
}
