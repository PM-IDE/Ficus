use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::analysis::patterns::activity_instances::process_activities_in_trace;
use crate::features::discovery::alpha::alpha::{
    discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop, ALPHA_SET,
};
use crate::features::discovery::alpha::alpha_set::AlphaSet;
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;
use crate::features::discovery::alpha::utils::maximize;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::utils::user_data::user_data::UserData;
use futures::AsyncReadExt;
use quick_xml::name::PrefixDeclaration::Default;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

struct AlphaPlusPlusNfcTriple<'a> {
    a_classes: BTreeSet<&'a String>,
    b_classes: BTreeSet<&'a String>,
    c_classes: BTreeSet<&'a String>,
}

impl<'a> AlphaPlusPlusNfcTriple<'a> {
    pub fn new(a_class: &'a String, b_class: &'a String, c_class: &'a String) -> Self {
        Self {
            a_classes: BTreeSet::from_iter(vec![a_class]),
            b_classes: BTreeSet::from_iter(vec![b_class]),
            c_classes: BTreeSet::from_iter(vec![c_class]),
        }
    }

    pub fn try_new<TLog: EventLog>(
        a_class: &'a String,
        b_class: &'a String,
        c_class: &'a String,
        provider: &AlphaPlusNfcRelationsProvider<'a, TLog>,
    ) -> Option<Self> {
        let candidate = Self::new(a_class, b_class, c_class);
        match candidate.valid(provider) {
            true => Some(candidate),
            false => None,
        }
    }

    pub fn try_merge<TLog: EventLog>(first: &Self, second: &Self, provider: &AlphaPlusNfcRelationsProvider<'a, TLog>) -> Option<Self> {
        let merge_sets = |first: &BTreeSet<&'a String>, second: &BTreeSet<&'a String>| -> BTreeSet<&'a String> {
            first.iter().chain(second.iter()).map(|class| *class).collect()
        };

        let new_triple = Self {
            a_classes: merge_sets(&first.a_classes, &second.a_classes),
            b_classes: merge_sets(&first.b_classes, &second.b_classes),
            c_classes: merge_sets(&first.c_classes, &second.c_classes),
        };

        match new_triple.valid(provider) {
            true => Some(new_triple),
            false => None,
        }
    }

    pub fn valid<TLog: EventLog>(&self, provider: &AlphaPlusNfcRelationsProvider<'a, TLog>) -> bool {
        for a_class in &self.a_classes {
            for b_class in &self.b_classes {
                for c_class in &self.c_classes {
                    if !(provider.direct_relation(a_class, c_class) && !provider.triangle_relation(c_class, a_class)) {
                        return false;
                    }

                    if !(provider.direct_relation(c_class, b_class) && !provider.triangle_relation(c_class, b_class)) {
                        return false;
                    }

                    if provider.parallel_relation(a_class, b_class) {
                        return false;
                    }

                    if !provider.unrelated_relation(a_class, a_class) || !provider.unrelated_relation(b_class, b_class) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl<'a> Hash for AlphaPlusPlusNfcTriple<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash_classes = |set: &BTreeSet<&'a String>| {
            for class in set {
                state.write(class.as_bytes());
            }
        };

        hash_classes(&self.a_classes);
        hash_classes(&self.b_classes);
        hash_classes(&self.c_classes);
    }
}

impl<'a> PartialEq for AlphaPlusPlusNfcTriple<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hasher = DefaultHasher::new();
        self.hash(&mut self_hasher);

        let mut other_hasher = DefaultHasher::new();
        other.hash(&mut other_hasher);

        self_hasher.finish() == other_hasher.finish()
    }
}

impl<'a> Clone for AlphaPlusPlusNfcTriple<'a> {
    fn clone(&self) -> Self {
        let clone_set = |set: &BTreeSet<&'a String>| -> BTreeSet<&'a String> { set.iter().map(|class| *class).collect() };

        Self {
            a_classes: clone_set(&self.a_classes),
            b_classes: clone_set(&self.b_classes),
            c_classes: clone_set(&self.c_classes),
        }
    }
}

impl<'a> Eq for AlphaPlusPlusNfcTriple<'a> {}

impl<'a> ToString for AlphaPlusPlusNfcTriple<'a> {
    fn to_string(&self) -> String {
        let mut repr = String::new();
        repr.push('(');

        let mut push_set = |set: &BTreeSet<&'a String>| {
            repr.push('{');

            for class in set.iter() {
                repr.push_str(class.as_str());
                repr.push(',')
            }

            if set.len() > 0 {
                repr.remove(repr.len() - 1);
            }

            repr.push_str("}, ");
        };

        push_set(&self.a_classes);
        push_set(&self.b_classes);
        push_set(&self.c_classes);

        repr.remove(repr.len() - 1);
        repr.remove(repr.len() - 1);

        repr.push(')');
        repr
    }
}

struct ExtendedAlphaSet<'a> {
    alpha_set: AlphaSet,
    left_extension: BTreeSet<&'a String>,
    right_extension: BTreeSet<&'a String>,
}

impl<'a> ExtendedAlphaSet<'a> {
    pub fn new_without_extensions(alpha_set: AlphaSet) -> Self {
        Self {
            alpha_set,
            left_extension: BTreeSet::new(),
            right_extension: BTreeSet::new(),
        }
    }

    pub fn new(alpha_set: AlphaSet, left_extension: &'a String, right_extension: &'a String) -> Self {
        Self {
            alpha_set,
            left_extension: BTreeSet::from_iter(vec![left_extension]),
            right_extension: BTreeSet::from_iter(vec![right_extension]),
        }
    }

    pub fn try_new<TLog: EventLog>(
        alpha_set: AlphaSet,
        left_extension: &'a String,
        right_extension: &'a String,
        provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
        w1_relations: &HashSet<(&'a String, &'a String)>,
        w2_relations: &HashSet<(&'a String, &'a String)>,
    ) -> Option<Self> {
        let new_set = Self::new(alpha_set, left_extension, right_extension);
        match new_set.valid(provider, w1_relations, w2_relations) {
            true => Some(new_set),
            false => None,
        }
    }

    pub fn valid<TLog: EventLog>(
        &self,
        provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
        w1_relations: &HashSet<(&'a String, &'a String)>,
        w2_relations: &HashSet<(&'a String, &'a String)>,
    ) -> bool {
        for a in &self.left_extension {
            if self.alpha_set.contains_left(a) {
                return false;
            }
        }

        for b in &self.right_extension {
            if self.alpha_set.contains_right(b) {
                return false;
            }
        }

        for a_class in self.alpha_set.left_classes() {
            for b in &self.right_extension {
                if !(w1_relations.contains(&(a_class, b)) || w2_relations.contains(&(a_class, b))) {
                    return false;
                }
            }
        }

        for b_class in self.alpha_set.right_classes().iter().chain(self.right_extension.iter()) {
            for a in &self.left_extension {
                if !(w1_relations.contains(&(a, b_class)) || w2_relations.contains(&(a, b_class))) {
                    return false;
                }
            }
        }

        for a_class in self.alpha_set.left_classes() {
            for a in &self.left_extension {
                if !(provider.unrelated_relation(a, a_class) && !provider.right_double_arrow_relation(a, a_class)) {
                    return false;
                }
            }
        }

        for b_class in self.alpha_set.right_classes() {
            for b in &self.right_extension {
                if !(provider.unrelated_relation(b_class, b) && !provider.right_double_arrow_relation(b_class, b)) {
                    return false;
                }
            }
        }

        true
    }

    pub fn subset(&self, other: &Self) -> bool {
        if !self.alpha_set.is_full_subset(&other.alpha_set) {
            false
        } else {
            self.left_extension.is_subset(&other.left_extension) && self.right_extension.is_subset(&other.right_extension)
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            alpha_set: self.alpha_set.extend(&other.alpha_set),
            left_extension: self.left_extension.iter().chain(&other.left_extension).map(|c| *c).collect(),
            right_extension: self.right_extension.iter().chain(&other.right_extension).map(|c| *c).collect(),
        }
    }
}

impl<'a> Hash for ExtendedAlphaSet<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.alpha_set.hash(state);
        for left in &self.left_extension {
            state.write(left.as_bytes());
        }

        for right in &self.right_extension {
            state.write(right.as_bytes());
        }
    }
}

impl<'a> PartialEq for ExtendedAlphaSet<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hasher = DefaultHasher::new();
        self.hash(&mut self_hasher);

        let mut other_hasher = DefaultHasher::new();
        other.hash(&mut other_hasher);

        self_hasher.finish() == other_hasher.finish()
    }
}

impl<'a> Eq for ExtendedAlphaSet<'a> {}

impl<'a> ToString for ExtendedAlphaSet<'a> {
    fn to_string(&self) -> String {
        let mut repr = String::new();
        repr.push('(');
        repr.push_str(self.alpha_set.to_string().as_str());
        repr.push_str(", ");

        let serilize_set = |set: &BTreeSet<&'a String>| {
            repr.push('{');
            for item in set {
                repr.push_str(item);
                repr.push(',');
            }

            if set.len() > 0 {
                repr.remove(repr.len() - 1);
            }

            repr.push_str("}, ");
        };

        repr.remove(repr.len() - 1);
        repr.remove(repr.len() - 1);

        repr.push(')');
        repr
    }
}

struct W3Pair<'a> {
    first: BTreeSet<&'a String>,
    second: BTreeSet<&'a String>,
}

impl<'a> W3Pair<'a> {
    pub fn new(first: &'a String, second: &'a String) -> Self {
        Self {
            first: BTreeSet::from_iter(vec![first]),
            second: BTreeSet::from_iter(vec![second]),
        }
    }

    pub fn try_new<TLog: EventLog>(
        first: &'a String,
        second: &'a String,
        w3_relations: &HashSet<(&String, &String)>,
        provider: &AlphaPlusNfcRelationsProvider<TLog>,
    ) -> Option<Self> {
        let new_pair = Self::new(first, second);
        match new_pair.valid(w3_relations, provider) {
            true => Some(new_pair),
            false => None,
        }
    }

    fn valid<TLog: EventLog>(&self, w3_relations: &HashSet<(&String, &String)>, provider: &AlphaPlusNfcRelationsProvider<TLog>) -> bool {
        for first in self.first.iter() {
            for second in self.second.iter() {
                if !(w3_relations.contains(&(first, second))
                    && provider.unrelated_relation(first, first)
                    && provider.unrelated_relation(second, second))
                {
                    return false;
                }
            }
        }

        true
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            first: self.first.iter().chain(other.first.iter()).map(|c| *c).collect(),
            second: self.second.iter().chain(other.second.iter()).map(|c| *c).collect(),
        }
    }
}

impl<'a> Hash for W3Pair<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for class in self.first.iter().chain(self.second.iter()) {
            state.write(class.as_bytes());
        }
    }
}

impl<'a> PartialEq for W3Pair<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_hasher = DefaultHasher::new();
        self.hash(&mut self_hasher);

        let mut other_hasher = DefaultHasher::new();
        other.hash(&mut other_hasher);

        self_hasher.finish() == other_hasher.finish()
    }
}

impl<'a> Eq for W3Pair<'a> {}

impl<'a> Clone for W3Pair<'a> {
    fn clone(&self) -> Self {
        Self {
            first: self.first.iter().map(|c| *c).collect(),
            second: self.second.iter().map(|c| *c).collect(),
        }
    }
}

impl<'a> Clone for ExtendedAlphaSet<'a> {
    fn clone(&self) -> Self {
        Self {
            alpha_set: self.alpha_set.clone(),
            left_extension: self.left_extension.iter().map(|c| *c).collect(),
            right_extension: self.right_extension.iter().map(|c| *c).collect(),
        }
    }
}

pub fn discover_petri_net_alpha_plus_plus_nfc<TLog: EventLog>(log: &TLog) {
    let one_length_loop_transitions = find_transitions_one_length_loop(log);
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));

    let provider = AlphaPlusNfcRelationsProvider::new(&info, log);

    let mut triples = HashSet::new();

    for a_class in info.all_event_classes() {
        for b_class in info.all_event_classes() {
            for c_class in &one_length_loop_transitions {
                if let Some(triple) = AlphaPlusPlusNfcTriple::try_new(a_class, b_class, c_class, &provider) {
                    triples.insert(triple);
                }
            }
        }
    }

    let mut current_triples = maximize(triples, |first, second| AlphaPlusPlusNfcTriple::try_merge(first, second, &provider));

    let petri_net = discover_petri_net_alpha_plus(log, false);

    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default_ignore(log, &one_length_loop_transitions));
    let mut provider = AlphaPlusNfcRelationsProvider::new(&info, log);

    let mut w1_relations = HashSet::new();
    for a_class in info.all_event_classes() {
        for b_class in info.all_event_classes() {
            if provider.w1_relation(a_class, b_class, &petri_net) {
                w1_relations.insert((a_class, b_class));
                provider.add_additional_causal_relation(a_class, b_class);
            }
        }
    }

    let mut w2_relations = HashSet::new();
    for a_class in info.all_event_classes() {
        for b_class in info.all_event_classes() {
            if provider.w2_relation(a_class, b_class, &petri_net) {
                w2_relations.insert((a_class, b_class));
            }
        }
    }

    eliminate_by_reduction_rule_1(&mut w2_relations, &mut provider, &petri_net, &info);

    let mut x_w = HashSet::new();
    let alpha_net = discover_petri_net_alpha(&info);
    for place in alpha_net.all_places() {
        if let Some(alpha_set) = place.user_data().concrete(&ALPHA_SET) {
            for pair in w1_relations.iter().chain(w2_relations.iter()) {
                let set = ExtendedAlphaSet::try_new(alpha_set.clone(), pair.0, pair.1, &mut provider, &w1_relations, &w2_relations);
                if let Some(extended_alpha_set) = set {
                    x_w.insert(extended_alpha_set);
                }
            }
        }
    }

    for place in alpha_net.all_places() {
        if let Some(alpha_set) = place.user_data().concrete(&ALPHA_SET) {
            x_w.insert(ExtendedAlphaSet::new_without_extensions(alpha_set.clone()));
        }
    }

    let y_w = maximize(x_w, |first, second| match first.subset(second) {
        true => Some(first.merge(second)),
        false => None,
    });

    for w2_relation in &w2_relations {
        provider.add_additional_causal_relation(w2_relation.0, w2_relation.1);
    }

    let mut w3_relations = HashSet::new();
    for a_class in info.all_event_classes() {
        for b_class in info.all_event_classes() {
            if provider.w3_relation(a_class, b_class, &petri_net) {
                w3_relations.insert((a_class, b_class));
            }
        }
    }

    let w3_closure = construct_w3_transitive_closure_cache(&w3_relations);
    eliminate_w3_relations_by_rule_2(&mut w3_relations, &w3_closure);

    let mut x_w = HashSet::new();
    for first_class in info.all_event_classes() {
        for second_class in info.all_event_classes() {
            if let Some(pair) = W3Pair::try_new(first_class, second_class, &w3_relations, &provider) {
                x_w.insert(pair);
            }
        }
    }

    let z_w = maximize(x_w, |first, second| {
        let new_pair = first.merge(second);
        if new_pair.valid(&w3_relations, &provider) {
            Some(new_pair)
        } else {
            None
        }
    });
}

fn eliminate_by_reduction_rule_1<TLog: EventLog>(
    w2_relations: &mut HashSet<(&String, &String)>,
    provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
    petri_net: &DefaultPetriNet,
    info: &EventLogInfo,
) {
    let mut to_remove = Vec::new();
    for w2_relation in w2_relations.iter() {
        let a = w2_relation.0;
        let c = w2_relation.1;
        for b in info.all_event_classes() {
            if (provider.w2_relation(a, b, petri_net) && provider.concave_arrow_relation(b, c))
                || (provider.w2_relation(b, c, petri_net) && provider.concave_arrow_relation(a, b))
            {
                to_remove.push(w2_relation.clone());
            }
        }
    }

    for item in &to_remove {
        w2_relations.remove(item);
    }
}

fn construct_w3_transitive_closure_cache<'a>(w3_relations: &'a HashSet<(&'a String, &'a String)>) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<&String, HashSet<&String>> = HashMap::new();
    let mut all_classes = HashSet::new();
    for relation in w3_relations {
        if let Some(children) = graph.get_mut(relation.0) {
            children.insert(relation.1);
        } else {
            graph.insert(relation.0, HashSet::from_iter(vec![relation.1]));
        }

        all_classes.insert(relation.0);
        all_classes.insert(relation.1);
    }

    let mut closure: HashMap<String, HashSet<String>> = HashMap::new();

    for first_class in &all_classes {
        for second_class in &all_classes {
            if let Some(children) = graph.get(first_class) {
                if children.contains(second_class) {
                    continue;
                }
            }

            let mut is_in_closure = false;
            let mut q = VecDeque::new();
            q.push_back(first_class);

            'q_loop: while !q.is_empty() {
                let current_class = q.pop_front().unwrap();
                if let Some(children) = graph.get(current_class) {
                    if children.contains(second_class) {
                        is_in_closure = true;
                        break 'q_loop;
                    } else {
                        for child in children {
                            q.push_back(child);
                        }
                    }
                }
            }

            if is_in_closure {
                if let Some(children) = closure.get_mut(*first_class) {
                    children.insert((**second_class).clone());
                } else {
                    closure.insert((**first_class).clone(), HashSet::from_iter(vec![(**second_class).clone()]));
                }
            }
        }
    }

    closure
}

fn eliminate_w3_relations_by_rule_2(w3_relations: &mut HashSet<(&String, &String)>, closure_cache: &HashMap<String, HashSet<String>>) {
    let mut to_remove = HashSet::new();
    for relation in w3_relations.iter() {
        if let Some(children) = closure_cache.get(relation.0) {
            if children.contains(relation.1) {
                to_remove.insert(relation.clone());
            }
        }
    }

    for item in &to_remove {
        w3_relations.remove(item);
    }
}
