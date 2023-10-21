use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::EventLogInfo;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;

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
                        let mut actual_length = 0;

                        for j in first_index..i {
                            let event = events[j].borrow();
                            let event_name = event.name();
                            if self.info.event_count(event_name) == 0 {
                                continue;
                            }

                            actual_length += 1;
                            if self.is_in_left_triangle_relation(event_name, first)
                                || self.is_in_right_triangle_relation(event_name, first)
                            {
                                all_suitable = false;
                                break;
                            }
                        }

                        if all_suitable && actual_length > 0 {
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

    pub fn is_in_w1_relation(&self, first: &str, second: &str, petri_net: &DefaultPetriNet) -> bool {
        if self.is_in_direct_relation(first, second) {
            return false;
        }

        for event_class in self.info.all_event_classes() {
            if let Some(transition_id) = petri_net.find_place_id_by_name(event_class) {
                let transition = petri_net.transition(&transition_id);
                for first_incoming_arc in transition.incoming_arcs() {
                    'second_loop: for second_incoming_arc in transition.incoming_arcs() {
                        let first_place_id = first_incoming_arc.place_id();
                        let second_place_id = second_incoming_arc.place_id();

                        if first_place_id == second_place_id {
                            continue 'second_loop;
                        }

                        let first_place_preset = petri_net.get_incoming_transitions(&first_place_id);
                        let second_place_preset = petri_net.get_incoming_transitions(&second_place_id);

                        let mut first_in_first_place_preset = false;
                        for first_pre_transition in &first_place_preset {
                            if first_pre_transition.name() == first {
                                first_in_first_place_preset = true;
                                break;
                            }
                        }

                        if !first_in_first_place_preset {
                            continue 'second_loop;
                        }

                        for second_pre_transition in &second_place_preset {
                            if second_pre_transition.name() == first {
                                continue 'second_loop;
                            }
                        }

                        let second_place_postset = petri_net.get_outgoing_transitions(&second_place_id);

                        let mut second_in_second_place_postset = false;
                        for second_post_transition in &second_place_postset {
                            if second_post_transition.name() == second {
                                second_in_second_place_postset = true;
                                break;
                            }
                        }

                        if !second_in_second_place_postset {
                            continue 'second_loop;
                        }

                        for second_pre_transition in &second_place_preset {
                            let name = second_pre_transition.name();
                            if self.is_in_concave_arrow_relation(name, first) || self.is_in_parallel_relation(name, first) {
                                continue 'second_loop;
                            }
                        }

                        return true;
                    }
                }
            }
        }

        false
    }
}
