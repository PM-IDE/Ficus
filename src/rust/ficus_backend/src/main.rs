use std::{collections::HashMap, fs::File, io::BufReader};
use quick_xml::reader::Reader;

fn main() {
    println!("Hello, world!");
}

trait Event {
    fn get_name(&self) -> &str;
    fn get_timestamp(&self) -> i64;
}

struct EventImpl {
    event_class: String,
    timestamp: i64,
    payload: HashMap<String, String>
}

impl EventImpl {
    fn new(name: String, timestamp: i64) -> EventImpl {
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

trait EventLogReader : Iterator {
}

struct FromFileXesEventLogReader {
    storage: Vec<u8>,
    reader: Reader<BufReader<File>>
}

impl Iterator for FromFileXesEventLogReader {
    type Item = EventImpl;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_event_into(&mut self.storage) {
            Ok(quick_xml::events::Event::Start(e)) => {
                if let Ok(event_class) = String::from_utf8(e.name().0.to_vec()) {
                    Some(EventImpl::new(event_class, 123))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

impl FromFileXesEventLogReader {
    fn new(file_path: &str) -> Option<FromFileXesEventLogReader> {
        match Reader::from_file(file_path) {
            Ok(reader) => Some(FromFileXesEventLogReader { reader, storage: Vec::new() }),
            Err(_) => None
        }
    }
}