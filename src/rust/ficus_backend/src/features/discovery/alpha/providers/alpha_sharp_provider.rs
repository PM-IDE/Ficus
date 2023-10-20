use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;

pub struct AlphaSharpRelationsProvider<'a> {
    alpha_plus_provider: AlphaPlusRelationsProvider<'a>,
    info: &'a EventLogInfo,
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
        self.alpha_plus_provider.is_in_causal_relation(first, second)
            && self.check_advanced_ordering_relation_second_part(first, second)
    }

    fn check_advanced_ordering_relation_second_part(&self, first: &str, second: &str) -> bool {
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
                if c_class.as_str() != first && d_class.as_str() != second {
                    let c_causal_d = self.is_in_causal_relation(c_class, d_class);
                    let first_advanced_d = self.is_in_advanced_ordering_relation(first, d_class);
                    let c_advanced_second = self.is_in_advanced_ordering_relation(c_class, second);

                    if c_causal_d && first_advanced_d && c_advanced_second {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl<'a> AlphaSharpRelationsProvider<'a> {
    pub fn new(log: &'a impl EventLog, info: &'a EventLogInfo) -> Self {
        Self {
            alpha_plus_provider: AlphaPlusRelationsProvider::new(info.dfg_info(), log),
            info,
        }
    }
}
