use crate::features::analysis::event_log_info::{DfgInfo, EventLogInfo};

pub trait AlphaRelationsProvider {
    fn causal_relation(&self, first: &str, second: &str) -> bool;
    fn parallel_relation(&self, first: &str, second: &str) -> bool;
    fn direct_relation(&self, first: &str, second: &str) -> bool;
    fn unrelated_relation(&self, first: &str, second: &str) -> bool;
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
    fn causal_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && !self.direct_relation(second, first)
    }

    fn parallel_relation(&self, first: &str, second: &str) -> bool {
        self.direct_relation(first, second) && self.direct_relation(second, first)
    }

    fn direct_relation(&self, first: &str, second: &str) -> bool {
        self.dfg_info.is_in_directly_follows_relation(first, second)
    }

    fn unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.direct_relation(first, second) && !self.direct_relation(second, first)
    }
}
