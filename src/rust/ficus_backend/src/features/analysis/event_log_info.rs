use crate::event_log::core::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::Trace;
use std::collections::{HashMap, HashSet};

pub struct EventLogInfo {
    events_count: usize,
    event_classes_counts: HashMap<String, usize>,
    dfg_info: DfgInfo
}

impl EventLogInfo {
    pub fn create_from<TLog>(log: &TLog) -> EventLogInfo
    where
        TLog: EventLog,
    {
        let mut dfg_pairs = HashMap::new();
        let mut followed_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut events_with_single_follower = HashSet::new();
        let mut events_count = 0;
        let mut events_counts = HashMap::new();

        for trace in log.get_traces() {
            let trace = trace.borrow();
            let events = trace.get_events();
            events_count += events.len();
            let mut prev_event_name = None;

            for event in events {
                let event = event.borrow();
                if let Some(count) = events_counts.get_mut(event.get_name()) {
                    *count += 1usize;
                } else {
                    events_counts.insert(event.get_name().to_owned(), 1usize);
                }

                let current_name = event.get_name().to_owned();
                if prev_event_name.is_none() {
                    prev_event_name = Some(current_name.to_owned());
                    continue;
                }

                let prev_name = prev_event_name.unwrap();
                let pair = (prev_name.to_owned(), current_name.to_owned());
                if dfg_pairs.contains_key(&pair) {
                    (*dfg_pairs.get_mut(&pair).unwrap()) += 1;
                } else {
                    dfg_pairs.insert(pair, 1usize);
                }

                prev_event_name = Some(event.get_name().to_owned());
            }
        }

        for ((first, second), count) in &dfg_pairs {
            if followed_events.contains_key(first) {
                if events_with_single_follower.contains(first) {
                    events_with_single_follower.remove(first);
                }

                if !followed_events.get(first).unwrap().contains_key(second) {
                    let followers_map = followed_events.get_mut(first).unwrap();
                    followers_map.insert(second.to_owned(), count.to_owned());
                }
            } else {
                let map = HashMap::from_iter(vec![(second.to_owned(), count.to_owned())]);
                followed_events.insert(first.to_owned(), map);
                events_with_single_follower.insert(first.to_owned());
            }
        }

        EventLogInfo {
            events_count,
            event_classes_counts: events_counts,
            dfg_info: DfgInfo {
                dfg_pairs,
                followed_events,
                events_with_single_follower,
            }
        }
    }

    pub fn get_events_count(&self) -> usize {
        self.events_count
    }

    pub fn get_event_classes_names(&self) -> &HashMap<String, usize> {
        &self.event_classes_counts
    }

    pub fn get_dfg_info(&self) -> &DfgInfo {
        &self.dfg_info
    }
}

#[derive(Debug)]
pub struct DfgInfo {
    dfg_pairs: HashMap<(String, String), usize>,
    followed_events: HashMap<String, HashMap<String, usize>>,
    events_with_single_follower: HashSet<String>,
}

impl DfgInfo {
    pub fn get_directly_follows_count(&self, pair: &(String, String)) -> usize {
        match self.dfg_pairs.get(pair) {
            Some(count) => count.to_owned(),
            None => 0
        }
    }

    pub fn get_followed_events(&self, event_class: &String) -> Option<&HashMap<String, usize>> {
        match self.followed_events.get(event_class) {
            Some(followers_counts) => Some(followers_counts),
            None => None
        }
    }

    pub fn is_event_with_single_follower(&self, event_class: &String) -> bool {
        self.events_with_single_follower.contains(event_class)
    }
}
