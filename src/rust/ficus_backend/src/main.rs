use std::{collections::HashMap, fs::File, io::BufReader, rc::Rc, cell::RefCell};
use quick_xml::reader::Reader;

fn main() {
    let path = "/Users/aero/Programming/pmide/Ficus/src/python/tests/test_data/source/example_logs/exercise1.xes";
    let reader = FromFileXesEventLogReader::new(path).unwrap();

    for trace in reader {
        println!("New trace");
        for event in trace {
            println!("{}", event.event_class);
        }
    }

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
    reader: Rc<RefCell<Reader<BufReader<File>>>>
}

struct TraceXesEventLogIterator {
    buffer: Vec<u8>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>
}

impl Iterator for TraceXesEventLogIterator {
    type Item = EventImpl;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.borrow_mut().read_event_into(&mut self.buffer) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    if let Ok(event_class) = String::from_utf8(e.name().0.to_vec()) {
                        return Some(EventImpl::new(event_class, 123));
                    } else {
                        continue;
                    }
                },
                Ok(quick_xml::events::Event::End(e)) => {
                    match e.name().as_ref() {
                        b"trace" => return None,
                        _ => continue
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                    return None;
                },
                _ => continue
            }
        }
    }
}

impl TraceXesEventLogIterator {
    fn new(reader: Rc<RefCell<Reader<BufReader<File>>>>) -> TraceXesEventLogIterator {
        TraceXesEventLogIterator { reader, buffer: Vec::new() }
    }
}

impl<'a> Iterator for FromFileXesEventLogReader {
    type Item = TraceXesEventLogIterator;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.borrow_mut().read_event_into(&mut self.storage) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"trace" => {
                            let copy_rc = Rc::clone(&self.reader);
                            return Some(TraceXesEventLogIterator::new(copy_rc))
                        },
                        _ => continue
                    }
                },
                Ok(quick_xml::events::Event::Eof) => return None,
                Err(error) => {
                    println!("Error: {}", error);
                    return None
                }
                _ => continue
            }
        }
    }
}

impl FromFileXesEventLogReader {
    fn new(file_path: &str) -> Option<FromFileXesEventLogReader> {
        match Reader::from_file(file_path) {
            Ok(reader) => Some(FromFileXesEventLogReader {
                reader: Rc::new(RefCell::new(reader)),
                storage: Vec::new()
            }),
            Err(_) => None
        }
    }
}