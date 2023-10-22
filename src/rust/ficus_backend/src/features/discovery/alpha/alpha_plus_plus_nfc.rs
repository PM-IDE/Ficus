use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha::{
    discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop, ALPHA_SET,
};
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::utils::user_data::user_data::UserData;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashSet};
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

    let mut current_triples = triples;
    loop {
        let mut any_change = false;
        let vec: Vec<&AlphaPlusPlusNfcTriple> = current_triples.iter().collect();
        let mut merged_indices = HashSet::new();
        let mut new_triples = HashSet::new();

        for i in 0..vec.len() {
            for j in (i + 1)..vec.len() {
                let first = vec.get(i).unwrap();
                let second = vec.get(j).unwrap();

                if let Some(merged_triple) = AlphaPlusPlusNfcTriple::try_merge(first, second, &provider) {
                    any_change = true;
                    new_triples.insert(merged_triple);
                    merged_indices.insert(i);
                    merged_indices.insert(j);
                }
            }
        }

        if !any_change {
            break;
        }

        for i in 0..vec.len() {
            if !merged_indices.contains(&i) {
                new_triples.insert((*vec.get(i).unwrap()).clone());
            }
        }

        current_triples = new_triples;
    }

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

    let alpha_net = discover_petri_net_alpha(&info);
    for place in alpha_net.all_places() {
        if let Some(alpha_set) = place.user_data().concrete(&ALPHA_SET) {
            'pair_loop: for pair in w1_relations.iter().chain(w2_relations.iter()) {
                let a = pair.0;
                let b = pair.1;

                if alpha_set.contains_left(a) || alpha_set.contains_right(b) {
                    continue;
                }

                for a_class in alpha_set.left_classes() {
                    if !(w1_relations.contains(&(a_class, b)) || w2_relations.contains(&(a_class, b))) {
                        continue 'pair_loop;
                    }

                    if !(provider.unrelated_relation(a, a_class) && !provider.right_double_arrow_relation(a, a_class)) {
                        continue 'pair_loop;
                    }
                }

                for b_class in alpha_set.right_classes() {
                    if !(w1_relations.contains(&(a, b_class)) || w2_relations.contains(&(a, b_class))) {
                        continue 'pair_loop;
                    }

                    if !(provider.unrelated_relation(b_class, b) && !provider.right_double_arrow_relation(b_class, b)) {
                        continue 'pair_loop;
                    }
                }

                if !(w1_relations.contains(&(a, b)) || w2_relations.contains(&(a, b))) {
                    continue 'pair_loop;
                }

                println!("{}", alpha_set.to_string());
            }
        }
    }

    for tuple in &w2_relations {
        println!("({}, {})", tuple.0, tuple.1);
    }
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
