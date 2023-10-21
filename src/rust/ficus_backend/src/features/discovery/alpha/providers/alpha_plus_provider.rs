use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::DfgInfo;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use std::collections::HashSet;

pub struct AlphaPlusRelationsProvider<'a> {
    dfg_info: &'a DfgInfo,
    triangle_relations: HashSet<(String, String)>,
}

impl<'a> AlphaPlusRelationsProvider<'a> {
    pub fn new(dfg_info: &'a DfgInfo, log: &'a impl EventLog) -> Self {
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
            dfg_info,
            triangle_relations,
        }
    }
}

impl<'a> AlphaRelationsProvider for AlphaPlusRelationsProvider<'a> {
    fn causal_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && (!self.direct_relation(second, first) || self.romb_relation(first, second))
    }

    fn parallel_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && self.direct_relation(second, first) && !self.romb_relation(first, second)
    }

    fn direct_relation(&self, first: &str, second: &str) -> bool {
        self.dfg_info.is_in_directly_follows_relation(first, second)
    }

    fn unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.direct_relation(first, second) && !self.direct_relation(second, first)
    }
}

impl<'a> AlphaPlusRelationsProvider<'a> {
    pub fn triangle_relation(&self, first: &str, second: &str) -> bool {
        self.triangle_relations.contains(&(first.to_owned(), second.to_owned()))
    }

    pub fn romb_relation(&self, first: &str, second: &str) -> bool {
        self.triangle_relation(first, second) && self.triangle_relation(second, first)
    }
}
