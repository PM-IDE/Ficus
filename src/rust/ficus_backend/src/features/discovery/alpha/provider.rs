use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::{DfgInfo, EventLogInfo};
use std::collections::HashSet;

pub trait AlphaRelationsProvider {
    fn is_in_causal_relation(&self, first: &str, second: &str) -> bool;
    fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool;
    fn is_in_direct_relation(&self, first: &str, second: &str) -> bool;
    fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool;
}

pub struct DefaultAlphaRelationsProvider<'a> {
    dfg_info: &'a DfgInfo,
}

impl<'a> DefaultAlphaRelationsProvider<'a> {
    pub fn new(dfg_info: &'a DfgInfo) -> Self {
        Self { dfg_info }
    }
}

impl<'a> AlphaRelationsProvider for DefaultAlphaRelationsProvider<'a> {
    fn is_in_causal_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }

    fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second) && self.is_in_direct_relation(second, first)
    }

    fn is_in_direct_relation(&self, first: &str, second: &str) -> bool {
        self.dfg_info.is_in_directly_follows_relation(first, second)
    }

    fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }
}

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
    fn is_in_causal_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second)
            && (!self.is_in_direct_relation(second, first) || self.is_in_romb_relation(first, second))
    }

    fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second)
            && self.is_in_direct_relation(second, first)
            && !self.is_in_romb_relation(first, second)
    }

    fn is_in_direct_relation(&self, first: &str, second: &str) -> bool {
        self.dfg_info.is_in_directly_follows_relation(first, second)
    }

    fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }
}

impl<'a> AlphaPlusRelationsProvider<'a> {
    pub fn is_in_triangle_relation(&self, first: &str, second: &str) -> bool {
        self.triangle_relations.contains(&(first.to_owned(), second.to_owned()))
    }

    pub fn is_in_romb_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_triangle_relation(first, second) && self.is_in_triangle_relation(second, first)
    }
}

pub struct AlphaSharpRelationsProvider<'a> {
    alpha_plus_provider: &'a AlphaPlusRelationsProvider<'a>,
    info: &'a EventLogInfo
}

impl<'a> AlphaRelationsProvider for AlphaSharpRelationsProvider<'a> {
    fn is_in_causal_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_causal_relation(first, second)
    }

    fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_parallel_relation(first, second)
    }

    fn is_in_direct_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_direct_relation(first, second)
    }

    fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_unrelated_relation(first, second)
    }
}

impl<'a> AlphaSharpRelationsProvider<'a> {
    pub fn is_in_triangle_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_triangle_relation(first, second)
    }

    pub fn is_in_romb_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_romb_relation(first, second)
    }

    pub fn is_in_advanced_ordering_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_causal_relation(first, second) && self.check_advanced_ordering_relation_second_part(first, second)
    }

    fn check_advanced_ordering_relation_second_part(&self, first: &str, second: & str) -> bool {
        let classes = self.info.all_event_classes();
        for x_class in &classes {
            for y_class in &classes {
                let first_causal_x = self.is_in_causal_relation(first, x_class);
                let y_causal_second = self.is_in_causal_relation(y_class, second);
                let x_following_y = self.is_in_direct_relation(y_class, x_class);
                let x_parallel_second = self.is_in_parallel_relation(x_class, second);
                let first_parallel_y = self.is_in_parallel_relation(first, y_class);

                if first_causal_x && y_causal_second && !x_following_y && !x_parallel_second && !first_parallel_y {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_in_real_causal_dependency(&self, first: &str, second: &str) -> bool {
        self.is_in_causal_relation(first, second) && !self.is_in_advanced_ordering_relation(first, second)
    }

    pub fn is_in_redundant_advanced_ordering_relation(&self, first: &str, second: &str) -> bool {
        let classes = self.info.all_event_classes();
        for c_class in &classes {
            for d_class in &classes {
                let c_causal_d = self.is_in_causal_relation(c_class, d_class);
                let first_advanced_d = self.is_in_advanced_ordering_relation(first, d_class);
                let c_advanced_second = self.is_in_advanced_ordering_relation(c_class, second);

                if c_causal_d && first_advanced_d && c_advanced_second {
                    return true;
                }
            }
        }

        false
    }
}