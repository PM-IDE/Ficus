use std::collections::HashSet;
use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha::find_transitions_one_length_loop;
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;

pub fn discover_petri_net_alpha_plus_plus_nfc<TLog: EventLog>(log: &TLog) {
    let one_length_loop_transitions = find_transitions_one_length_loop(log);
    let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));

    let provider = AlphaPlusNfcRelationsProvider::new(&info, log);

    let mut triples = HashSet::new();

    for a_class in info.all_event_classes() {
        for b_class in info.all_event_classes() {
            for c_class in &one_length_loop_transitions {
                if !(provider.is_in_direct_relation(a_class, c_class) && !provider.is_in_triangle_relation(c_class, a_class)) {
                    continue;
                }

                if !(provider.is_in_direct_relation(c_class, b_class) && !provider.is_in_triangle_relation(c_class, b_class)) {
                    continue;
                }

                if provider.is_in_parallel_relation(a_class, b_class) {
                    continue;
                }

                if !provider.is_in_unrelated_relation(a_class, a_class) || !provider.is_in_unrelated_relation(b_class, b_class) {
                    continue;
                }

                triples.insert((a_class, b_class, c_class));
            }
        }
    }

    for triple in &triples {
        println!("{:?}", triple);
    }
}