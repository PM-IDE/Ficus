use crate::features::analysis::event_log_info::DfgInfo;

pub struct DefaultAlphaRelationsProvider<'a> {
    dfg_info: &'a DfgInfo,
}

impl<'a> DefaultAlphaRelationsProvider<'a> {
    pub fn new(dfg_info: &'a DfgInfo) -> Self {
        Self { dfg_info }
    }

    pub fn is_in_casual_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }

    pub fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second) && self.is_in_direct_relation(second, first)
    }

    pub fn is_in_direct_relation(&self, first: &str, second: &str) -> bool {
        self.dfg_info.is_in_directly_follows_relation(first, second)
    }

    pub fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool {
        !self.is_in_direct_relation(first, second) && !self.is_in_direct_relation(second, first)
    }
}
