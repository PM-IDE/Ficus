use crate::event_log::core::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::Trace;
use std::collections::HashMap;

pub struct EventLogInfo {
    events_count: usize,
    event_classes_counts: HashMap<String, usize>,
}

impl EventLogInfo {
    pub fn new<TLog>(log: &TLog) -> EventLogInfo
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
