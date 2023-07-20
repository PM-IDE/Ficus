use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::Trace;
use crate::{event_log::core::event::Event, utils::hash_map_utils::increase_in_map};
use std::collections::{HashMap, HashSet};

use super::constants::{FAKE_EVENT_END_NAME, FAKE_EVENT_START_NAME};

pub struct EventLogInfo {
    events_count: usize,
    event_classes_counts: HashMap<String, usize>,
    dfg_info: DfgInfo,
}

pub struct EventLogInfoCreationDto<'a, TLog>
where
    TLog: EventLog,
{
    log: &'a TLog,
    add_fake_start_end_events: bool,
    ignored_events: Option<HashSet<String>>,
}

impl<'a, TLog> EventLogInfoCreationDto<'a, TLog>
where
    TLog: EventLog,
{
    pub fn default(log: &'a TLog) -> Self {
        EventLogInfoCreationDto {
            log,
            add_fake_start_end_events: false,
            ignored_events: None,
        }
    }

    pub fn default_fake_events(log: &'a TLog) -> Self {
        EventLogInfoCreationDto {
            log,
            add_fake_start_end_events: true,
            ignored_events: None,
        }
    }

    pub fn default_fake_ignored(log: &'a TLog, ignored_events: Option<HashSet<String>>) -> Self {
        EventLogInfoCreationDto {
            log,
            add_fake_start_end_events: true,
            ignored_events: ignored_events,
        }
    }
}

impl EventLogInfo {
    pub fn create_from<'a, TLog>(creation_dto: EventLogInfoCreationDto<'a, TLog>) -> EventLogInfo
    where
        TLog: EventLog,
    {
        let EventLogInfoCreationDto {
            log,
            add_fake_start_end_events,
            ignored_events,
        } = creation_dto;

        let mut dfg_pairs = HashMap::new();
        let mut followed_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut events_with_single_follower = HashSet::new();
        let mut events_count = 0;
        let mut events_counts = HashMap::new();

        let mut update_events_counts = |event_name: &String| {
            increase_in_map(&mut events_counts, event_name);
        };

        let mut update_pairs_count = |first_name: &String, second_name: &String| {
            let pair = (first_name.to_owned(), second_name.to_owned());
            increase_in_map(&mut dfg_pairs, &pair);
        };

        for trace in log.get_traces() {
            let trace = trace.borrow();
            let events = trace.get_events();
            events_count += events.len();
            let mut prev_event_name = None;

            for event in events {
                let event = event.borrow();
                let current_name = event.get_name().to_owned();

                if let Some(ignored_events) = &ignored_events {
                    if ignored_events.contains(&current_name) {
                        continue;
                    }
                }

                update_events_counts(&current_name);

                if prev_event_name.is_none() {
                    prev_event_name = Some(current_name.to_owned());
                    if add_fake_start_end_events {
                        update_pairs_count(&FAKE_EVENT_START_NAME.to_string(), &current_name);
                    }

                    continue;
                }

                let prev_name = prev_event_name.unwrap();
                update_pairs_count(&prev_name, &current_name);
                prev_event_name = Some(event.get_name().to_owned());
            }

            if add_fake_start_end_events && prev_event_name.is_some() {
                update_pairs_count(&prev_event_name.unwrap(), &FAKE_EVENT_END_NAME.to_string());
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
            },
        }
    }

    pub fn get_events_count(&self) -> usize {
        self.events_count
    }

    pub fn get_event_classes_count(&self) -> usize {
        self.event_classes_counts.len()
    }

    pub fn get_event_count(&self, event_class: &String) -> usize {
        match self.event_classes_counts.get(event_class) {
            Some(value) => value.to_owned(),
            None => 0,
        }
    }

    pub fn get_dfg_info(&self) -> &DfgInfo {
        &self.dfg_info
    }

    pub fn get_all_event_classes(&self) -> Vec<&String> {
        self.event_classes_counts.keys().into_iter().collect()
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
            None => 0,
        }
    }

    pub fn get_followed_events(&self, event_class: &String) -> Option<&HashMap<String, usize>> {
        match self.followed_events.get(event_class) {
            Some(followers_counts) => Some(followers_counts),
            None => None,
        }
    }

    pub fn is_event_with_single_follower(&self, event_class: &String) -> bool {
        self.events_with_single_follower.contains(event_class)
    }
}
