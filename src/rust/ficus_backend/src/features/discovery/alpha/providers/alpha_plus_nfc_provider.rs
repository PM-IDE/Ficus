use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::EventLogInfo;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;

pub struct AlphaPlusNfcRelationsProvider<'a, TLog>
where
    TLog: EventLog,
{
    alpha_plus_provider: AlphaPlusRelationsProvider<'a>,
    info: &'a EventLogInfo,
    log: &'a TLog,
}

impl<'a, TLog> AlphaPlusNfcRelationsProvider<'a, TLog>
where
    TLog: EventLog,
{
    pub fn new(info: &'a EventLogInfo, log: &'a TLog) -> Self {
        Self {
            alpha_plus_provider: AlphaPlusRelationsProvider::new(info.dfg_info(), log),
            info,
            log,
        }
    }

    pub fn is_in_triangle_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_triangle_relation(first, second)
    }

    pub fn is_in_direct_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_direct_relation(first, second)
    }

    pub fn is_in_causal_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_direct_relation(first, second)
            && (!self.alpha_plus_provider.is_in_direct_relation(second, first)
                || self.is_in_triangle_relation(first, second)
                || self.is_in_triangle_relation(second, first))
    }

    pub fn is_in_unrelated_relation(&self, first: &str, second: &str) -> bool {
        self.alpha_plus_provider.is_in_unrelated_relation(first, second)
    }

    pub fn is_in_parallel_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_direct_relation(first, second)
            && self.is_in_direct_relation(second, first)
            && !(self.is_in_triangle_relation(first, second) || self.is_in_triangle_relation(second, first))
    }

    pub fn is_in_left_triangle_relation(&self, first: &str, second: &str) -> bool {
        if !self.is_in_unrelated_relation(first, second) {
            return false;
        }

        for class in self.info.all_event_classes() {
            if self.is_in_causal_relation(class, first) && self.is_in_causal_relation(class, second) {
                return true;
            }
        }

        false
    }

    pub fn is_in_right_triangle_relation(&self, first: &str, second: &str) -> bool {
        if !self.is_in_unrelated_relation(first, second) {
            return false;
        }

        for class in self.info.all_event_classes() {
            if self.is_in_causal_relation(first, class) && self.is_in_causal_relation(second, class) {
                return true;
            }
        }

        false
    }

    pub fn is_in_right_double_arrow_relation(&self, first: &str, second: &str) -> bool {
        if self.is_in_direct_relation(first, second) {
            return false;
        }

        for trace in self.log.traces() {
            let trace = trace.borrow();
            let events = trace.events();
            let mut last_first_index = None;
            for i in 0..events.len() {
                if events[i].borrow().name() == first {
                    last_first_index = Some(i);
                    continue;
                }

                if events[i].borrow().name() == second {
                    if let Some(first_index) = last_first_index {
                        let mut all_suitable = true;
                        for j in first_index..i {
                            let event = events[j].borrow();
                            let event_name = event.name();
                            if self.is_in_left_triangle_relation(event_name, first)
                                || self.is_in_right_triangle_relation(event_name, first)
                            {
                                all_suitable = false;
                                break;
                            }
                        }

                        if all_suitable {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_in_concave_arrow_relation(&self, first: &str, second: &str) -> bool {
        self.is_in_causal_relation(first, second) || self.is_in_right_double_arrow_relation(first, second)
    }
}
