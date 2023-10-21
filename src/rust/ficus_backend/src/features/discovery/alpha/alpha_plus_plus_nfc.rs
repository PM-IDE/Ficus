use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha::find_transitions_one_length_loop;
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;
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

    pub fn valid<TLog: EventLog>(&self, provider: &AlphaPlusNfcRelationsProvider<'a, TLog>) -> bool {
        for a_class in &self.a_classes {
            for b_class in &self.b_classes {
                for c_class in &self.c_classes {
                    if !(provider.is_in_direct_relation(a_class, c_class)
                        && !provider.is_in_triangle_relation(c_class, a_class))
                    {
                        return false;
                    }

                    if !(provider.is_in_direct_relation(c_class, b_class)
                        && !provider.is_in_triangle_relation(c_class, b_class))
                    {
                        return false;
                    }

                    if provider.is_in_parallel_relation(a_class, b_class) {
                        return false;
                    }

                    if !provider.is_in_unrelated_relation(a_class, a_class)
                        || !provider.is_in_unrelated_relation(b_class, b_class)
                    {
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

    for triple in &triples {
        println!("{:?}", triple.to_string());
    }
}
