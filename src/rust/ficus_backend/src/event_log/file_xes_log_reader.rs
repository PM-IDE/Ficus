use std::{rc::Rc, cell::RefCell, io::BufReader, fs::File};
use quick_xml::{Reader};
use super::{constants::{*}, xes_log_trace_reader::TraceXesEventLogIterator};


pub(crate) struct FromFileXesEventLogReader {
    storage: Vec<u8>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>
}

impl<'a> Iterator for FromFileXesEventLogReader {
    type Item = TraceXesEventLogIterator;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.reader.borrow_mut().read_event_into(&mut self.storage) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    match e.name().as_ref() {
                        TRACE_TAG_NAME => {
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
    pub fn new(file_path: &str) -> Option<FromFileXesEventLogReader> {
        match Reader::from_file(file_path) {
            Ok(reader) => Some(FromFileXesEventLogReader {
                reader: Rc::new(RefCell::new(reader)),
                storage: Vec::new()
            }),
            Err(_) => None
        }
    }
}