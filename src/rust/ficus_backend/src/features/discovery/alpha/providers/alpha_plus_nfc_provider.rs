use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::event_log_info::EventLogInfo;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use std::collections::HashSet;

enum PrePostSet {
    PreSet,
    PostSet,
}

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
                            if self.is_in_concave_arrow_relation(name, first)
                                || self.is_in_parallel_relation(name, first)
                            {
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

    pub fn is_in_w21_relation(&self, first: &str, second: &str, petri_net: &DefaultPetriNet) -> bool {
        if !self.is_in_right_double_arrow_relation(first, second) {
            return false;
        }

        if let Some(first_transition) = petri_net.find_transition_by_name(first) {
            let first_outgoing_arcs = first_transition.outgoing_arcs();
            if first_outgoing_arcs.len() <= 1 {
                return false;
            }

            for second_streak in self.info.all_event_classes() {
                if !self.is_in_left_triangle_relation(second, second_streak) {
                    continue;
                }

                for first_outgoing_arc in first_outgoing_arcs {
                    let place_id = first_outgoing_arc.place_id();
                    let post_set = petri_net.get_outgoing_transitions(&place_id);

                    let mut first_condition = false;
                    let mut second_condition = false;

                    for t in &post_set {
                        if !first_condition {
                            first_condition = self.is_in_concave_arrow_relation(t.name(), second);
                            first_condition |= self.is_in_parallel_relation(t.name(), second);
                        }

                        if !second_condition {
                            second_condition = self.is_in_concave_arrow_relation(t.name(), second_streak);
                            second_condition |= self.is_in_parallel_relation(t.name(), second_streak);
                        }
                    }

                    if !first_condition && second_condition {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_in_w22_relation(&self, first: &str, second: &str, petri_net: &DefaultPetriNet) -> bool {
        if !self.is_in_right_double_arrow_relation(first, second) {
            return false;
        }

        if let Some(second_transition) = petri_net.find_transition_by_name(second) {
            let second_preset = second_transition.incoming_arcs();
            if second_preset.len() <= 1 {
                return false;
            }

            for first_streak in self.info.all_event_classes() {
                if !self.is_in_right_triangle_relation(first, first_streak) {
                    continue;
                }

                for preset_arc in second_preset {
                    let preset_place_id = preset_arc.place_id();

                    let preset_tasks = petri_net.get_incoming_transitions(&preset_place_id);
                    let mut first_condition = false;
                    let mut second_condition = false;
                    for preset_task in preset_tasks {
                        if !first_condition {
                            first_condition = self.is_in_concave_arrow_relation(first, preset_task.name());
                            first_condition |= self.is_in_parallel_relation(first, preset_task.name());
                        }

                        if !second_condition {
                            second_condition = self.is_in_concave_arrow_relation(first_streak, preset_task.name());
                            second_condition |= self.is_in_parallel_relation(first_streak, preset_task.name());
                        }
                    }

                    if !first_condition && second_condition {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn is_in_w2_relation(&self, first: &str, second: &str, petri_net: &DefaultPetriNet) -> bool {
        self.is_in_w21_relation(first, second, petri_net) || self.is_in_w22_relation(first, second, petri_net)
    }

    pub fn is_in_w3_relation(&self, first: &str, second: &str, petri_net: &DefaultPetriNet) -> bool {
        if !self.is_in_right_double_arrow_relation(first, second) {
            return false;
        }

        let first_post_set = Self::get_pre_or_post_set(petri_net, first, PrePostSet::PostSet);
        let second_pre_set = Self::get_pre_or_post_set(petri_net, second, PrePostSet::PreSet);

        for first_streak in self.info.all_event_classes() {
            for second_streak in self.info.all_event_classes() {
                if first_streak == second_streak {
                    continue;
                }

                let first_streak_post_set = Self::get_pre_or_post_set(petri_net, first_streak, PrePostSet::PostSet);
                let second_streak_pre_set = Self::get_pre_or_post_set(petri_net, second_streak, PrePostSet::PreSet);

                let first_intersection: HashSet<&u64> = first_post_set.intersection(&first_streak_post_set).collect();
                if first_intersection.len() == 0 {
                    continue;
                }

                let second_intersection: HashSet<&u64> = second_pre_set.intersection(&second_streak_pre_set).collect();
                if second_intersection.len() == 0 {
                    continue;
                }

                if self.is_in_right_double_arrow_relation(first, second_streak) {
                    continue;
                }

                if self.is_in_right_double_arrow_relation(first_streak, second) {
                    continue;
                }

                if !self.is_in_right_double_arrow_relation(first_streak, second_streak) {
                    continue;
                }

                let mut right_set = HashSet::new();
                for task in self.info.all_event_classes() {
                    if self.is_in_right_double_arrow_relation(first, task) {
                        continue;
                    }

                    if !self.is_in_right_double_arrow_relation(first_streak, task) {
                        continue;
                    }

                    if !(self.is_in_parallel_relation(second_streak, task)
                        || self.is_in_concave_arrow_relation(second_streak, task))
                    {
                        continue;
                    }

                    let task_pre_set = Self::get_pre_or_post_set(petri_net, task, PrePostSet::PreSet);
                    let intersection: HashSet<&u64> = second_streak_pre_set.intersection(&task_pre_set).collect();
                    if intersection.len() == 0 {
                        continue;
                    }

                    for place_id in &task_pre_set {
                        right_set.insert(*place_id);
                    }
                }

                for place_id in second_streak_pre_set {
                    right_set.insert(place_id);
                }

                if second_pre_set.is_subset(&right_set) {
                    return true;
                }
            }
        }

        false
    }

    fn get_pre_or_post_set(petri_net: &DefaultPetriNet, transition_name: &str, pre_set: PrePostSet) -> HashSet<u64> {
        let transition = petri_net.find_transition_by_name(transition_name).unwrap();
        let arcs = match pre_set {
            PrePostSet::PreSet => transition.incoming_arcs(),
            PrePostSet::PostSet => transition.outgoing_arcs(),
        };

        return arcs.iter().map(|arc| arc.place_id()).collect();
    }
}
