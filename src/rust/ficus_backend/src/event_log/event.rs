use std::collections::HashMap;

pub(crate) trait Event {
    fn get_name(&self) -> &str;
    fn get_timestamp(&self) -> i64;
}

pub(crate) struct EventImpl {
    event_class: String,
    timestamp: i64,
    payload: HashMap<String, String>
}

impl EventImpl {
    pub(crate) fn new(name: String, timestamp: i64) -> EventImpl {
        EventImpl { event_class: name.to_owned(), timestamp, payload: HashMap::new() }
    }
}

impl Event for EventImpl {
    fn get_name(&self) -> &str {
        self.event_class.as_str()
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}