use std::collections::HashSet;

use regex::Regex;

use crate::event_log::core::{event::event::Event, event_log::EventLog};

pub fn filter_log_by_name(log: &mut impl EventLog, name: &str)
{
    log.filter_events_by(|event| event.get_name() == name);
}

pub fn filter_log_by_names(log: &mut impl EventLog, names: &HashSet<String>)
{
    log.filter_events_by(|event| names.contains(event.get_name()));
}

pub fn filter_log_by_regex(log: &mut impl EventLog, regex: &Regex) {
    log.filter_events_by(|event| regex.is_match(event.get_name()));
}