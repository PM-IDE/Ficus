use crate::event_log::core::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::Trace;
use std::collections::{HashMap, HashSet};

pub struct EventLogInfo {
    events_count: usize,
    event_classes_counts: HashMap<String, usize>,
}

impl EventLogInfo {
    pub fn create_from<TLog>(log: &TLog) -> EventLogInfo
    where
        TLog: EventLog,
    {
        let mut events_count = 0;
        let mut map = HashMap::new();

        for trace in log.get_traces() {
            let trace = trace.borrow();
            let events = trace.get_events();
            events_count += events.len();

            for event in events {
                let event = event.borrow();
                if let Some(count) = map.get_mut(event.get_name()) {
                    *count += 1usize;
                } else {
                    map.insert(event.get_name().to_owned(), 1usize);
                }
            }
        }

        EventLogInfo {
            events_count,
            event_classes_counts: map,
        }
    }

    pub fn get_events_count(&self) -> usize {
        self.events_count
    }

    pub fn get_event_classes_names(&self) -> &HashMap<String, usize> {
        &self.event_classes_counts
    }
}

#[derive(Debug)]
pub struct DfgInfo {
    dfg_pairs: HashMap<(String, String), usize>,
    followed_events: HashMap<String, HashMap<String, usize>>,
    events_with_single_follower: HashSet<String>,
}

impl DfgInfo {
    pub fn new<TLog>(log: &TLog) -> DfgInfo
    where
        TLog: EventLog,
    {
        let mut dfg_pairs = HashMap::new();
        let mut followed_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut events_with_single_follower = HashSet::new();

        for trace in log.get_traces() {
            let mut prev_event_name = None;
            for event in trace.borrow().get_events() {
                let current_name = event.borrow().get_name().to_owned();
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

                prev_event_name = Some(event.borrow().get_name().to_owned());
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

        DfgInfo {
            dfg_pairs,
            followed_events,
            events_with_single_follower,
        }
    }
}
