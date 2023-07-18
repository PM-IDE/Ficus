use std::collections::HashSet;

use crate::event_log::core::{event::Event, event_log::EventLog};

pub fn filter_log_by_name<TLog>(log: &mut TLog, name: &str)
where
    TLog: EventLog,
{
    log.filter_events_by(|event| event.get_name() == name);
}

pub fn filter_log_by_names<TLog>(log: &mut TLog, names: &HashSet<String>)
where
    TLog: EventLog,
{
    log.filter_events_by(|event| names.contains(event.get_name()));
}
