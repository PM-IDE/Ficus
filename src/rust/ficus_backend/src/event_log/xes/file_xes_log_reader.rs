use super::{constants::*, xes_log_trace_reader::TraceXesEventLogIterator, shared::{XesEventLogExtension, XesGlobal}, utils};
use quick_xml::{Reader, events::BytesStart};
use std::{cell::RefCell, fs::File, io::BufReader, rc::Rc, collections::HashMap};

pub(crate) struct FromFileXesEventLogReader {
    storage: Rc<RefCell<Vec<u8>>>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>,
}

pub enum XesEventLogItem {
    Trace(TraceXesEventLogIterator),
    Global(XesGlobal),
    Extension(XesEventLogExtension)
}

impl Iterator for FromFileXesEventLogReader {
    type Item = XesEventLogItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut storage = self.storage.borrow_mut();
        let mut reader = self.reader.borrow_mut();

        loop {
            match reader.read_event_into(&mut storage) {
                Ok(quick_xml::events::Event::Start(e)) => match e.name().as_ref() {
                    TRACE_TAG_NAME => {
                        let copy_rc = Rc::clone(&self.reader);
                        return Some(XesEventLogItem::Trace(TraceXesEventLogIterator::new(copy_rc)));
                    },
                    GLOBAL_TAG_NAME => {
                        let mut scope_name: Option<String> = None;

                        for attr in e.attributes() {
                            match attr {
                                Ok(real_attr) => match real_attr.key.0 {
                                    SCOPE_ATTR_NAME => match String::from_utf8(real_attr.value.into_owned()) {
                                        Ok(string) => scope_name = Some(string),
                                        Err(_) => continue,
                                    },
                                    _ => continue
                                },
                                Err(_) => continue
                            }
                        }

                        if scope_name.is_none() { continue }

                        match Self::read_global(&mut reader, &mut storage) {
                            Some(default_values) => return Some(XesEventLogItem::Global(XesGlobal { scope: scope_name.unwrap(), default_values })),
                            None => continue,
                        }
                    },
                    EXTENSION_TAG_NAME => match Self::read_extension(&e) {
                        Some(extension) => return Some(XesEventLogItem::Extension(extension)),
                        None => continue
                    }
                    _ => continue,
                },
                Ok(quick_xml::events::Event::Eof) => return None,
                Err(error) => {
                    println!("Error: {}", error);
                    return None;
                }
                _ => continue,
            }
        }
    }
}

impl FromFileXesEventLogReader {
    pub fn new(file_path: &str) -> Option<FromFileXesEventLogReader> {
        match Reader::from_file(file_path) {
            Ok(reader) => Some(FromFileXesEventLogReader {
                reader: Rc::new(RefCell::new(reader)),
                storage: Rc::new(RefCell::new(Vec::new())),
            }),
            Err(_) => None,
        }
    }

    fn read_global(
        reader: &mut Reader<BufReader<File>>,
        storage: &mut Vec<u8>
    ) -> Option<HashMap<String, String>> {
        let mut map: Option<HashMap<String, String>> = None;

        loop {
            match reader.read_event_into(storage) {
                Err(_) => return None,
                Ok(quick_xml::events::Event::Empty(tag)) => {
                    let kv = utils::extract_key_value(&tag);
                    if kv.key.is_none() || kv.value.is_none() { return None }

                    match map {
                        Some(_) => {},
                        None => map = Some(HashMap::new())
                    }

                    map.as_mut().unwrap().insert(kv.key.unwrap(), kv.value.unwrap());
                },
                Ok(quick_xml::events::Event::End(tag)) => match tag.name().0 {
                    GLOBAL_TAG_NAME => break,
                    _ => continue,
                },
                _ => continue
            }
        }

        map
    }

    fn read_extension(tag: &BytesStart) -> Option<XesEventLogExtension> {
        let mut name: Option<String> = None;
        let mut prefix: Option<String> = None;
        let mut uri: Option<String> = None;

        for attr in tag.attributes() {
            match attr {
                Ok(real_attr) => match real_attr.key.0 {
                    PREFIX_ATTR_NAME => match String::from_utf8(real_attr.value.into_owned()) {
                        Ok(string) => prefix = Some(string),
                        Err(_) => return None
                    },
                    NAME_ATTR_NAME => match String::from_utf8(real_attr.value.into_owned()) {
                        Ok(string) => name = Some(string),
                        Err(_) => return None
                    },
                    URI_ATTR_NAME => match String::from_utf8(real_attr.value.into_owned()) {
                        Ok(string) => uri = Some(string),
                        Err(_) => return None
                    },
                    _ => continue,
                },
                Err(_) => return None
            }
        }

        if !name.is_some() || !prefix.is_some() || !uri.is_some() {
            return None;
        }

        Some(XesEventLogExtension { name: name.unwrap(), prefix: prefix.unwrap(), uri: uri.unwrap() })
    }
}
