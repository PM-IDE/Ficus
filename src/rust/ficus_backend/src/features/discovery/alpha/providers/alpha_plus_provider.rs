use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::{DfgInfo, EventLogInfo};
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use std::collections::HashSet;

pub trait AlphaPlusRelationsProvider: AlphaRelationsProvider {
    fn triangle_relation(&self, first: &str, second: &str) -> bool;
    fn romb_relation(&self, first: &str, second: &str) -> bool;

    fn one_length_loop_transitions(&self) -> &HashSet<String>;
}

pub struct AlphaPlusRelationsProviderImpl<'a> {
    pub log_info: &'a EventLogInfo,
    triangle_relations: HashSet<(String, String)>,
    one_length_loop_transitions: &'a HashSet<String>,
}

impl<'a> AlphaPlusRelationsProviderImpl<'a> {
    pub fn new(log_info: &'a EventLogInfo, log: &'a impl EventLog, one_length_loop_transitions: &'a HashSet<String>) -> Self {
        let mut triangle_relations = HashSet::new();
        for trace in log.traces() {
            let trace = trace.borrow();
            let events = trace.events();

            for index in 0..(events.len() - 2) {
                if events[index].borrow().name() == events[index + 2].borrow().name() {
                    triangle_relations.insert((
                        events[index].borrow().name().to_owned(),
                        events[index + 1].borrow().name().to_owned(),
                    ));
                }
            }
        }

        Self {
            log_info,
            triangle_relations,
            one_length_loop_transitions,
        }
    }
}

impl<'a> AlphaRelationsProvider for AlphaPlusRelationsProviderImpl<'a> {
    fn causal_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && (!self.direct_relation(second, first) || self.romb_relation(first, second))
    }

    fn parallel_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && self.direct_relation(second, first) && !self.romb_relation(first, second)
    }

    fn direct_relation(&self, first: &str, second: &str) -> bool {
        self.log_info.dfg_info().is_in_directly_follows_relation(first, second)
    }

    fn unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.direct_relation(first, second) && !self.direct_relation(second, first)
    }

    fn log_info(&self) -> &EventLogInfo {
        self.log_info
    }
}

impl<'a> AlphaPlusRelationsProvider for AlphaPlusRelationsProviderImpl<'a> {
    fn triangle_relation(&self, first: &str, second: &str) -> bool {
        self.triangle_relations.contains(&(first.to_owned(), second.to_owned()))
    }

    fn romb_relation(&self, first: &str, second: &str) -> bool {
        self.triangle_relation(first, second) && self.triangle_relation(second, first)
    }

    fn one_length_loop_transitions(&self) -> &HashSet<String> {
        self.one_length_loop_transitions
    }
}
