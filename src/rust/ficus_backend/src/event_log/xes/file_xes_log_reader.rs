use super::{constants::*, xes_log_trace_reader::TraceXesEventLogIterator, shared::{XesEventLogExtension, XesGlobal, XesClassifier}, utils};
use quick_xml::{Reader, events::BytesStart};
use std::{cell::{RefCell, RefMut}, fs::File, io::BufReader, rc::Rc, collections::HashMap};

pub(crate) struct FromFileXesEventLogReader {
    storage: Rc<RefCell<Vec<u8>>>,
    reader: Rc<RefCell<Reader<BufReader<File>>>>,
}

pub enum XesEventLogItem {
    Trace(TraceXesEventLogIterator),
    Global(XesGlobal),
    Extension(XesEventLogExtension),
    Classifier(XesClassifier),
}

impl Iterator for FromFileXesEventLogReader {
    type Item = XesEventLogItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut storage = self.storage.borrow_mut();
        let mut reader = self.reader.borrow_mut();

        loop {
            match reader.read_event_into(&mut storage) {
                Ok(quick_xml::events::Event::Start(tag)) => match tag.name().as_ref() {
                    TRACE_TAG_NAME => {
                        let copy_rc = Rc::clone(&self.reader);
                        return Some(XesEventLogItem::Trace(TraceXesEventLogIterator::new(copy_rc)));
                    },
                    GLOBAL_TAG_NAME => match Self::read_scope_name(&tag) {
                        Some(scope_name) => match Self::read_global_internal(&mut reader, &mut storage) {
                            Some(default_values) => {
                                let global = XesGlobal { scope: scope_name, default_values };
                                return Some(XesEventLogItem::Global(global))
                            },
                            None => continue,
                        },
                        None => continue
                    },
                    EXTENSION_TAG_NAME | CLASSIFIER_TAG_NAME => match Self::read_extension_or_classifier(&tag) {
                        Some(item) => return Some(item),
                        None => continue,
                    },
                    _ => continue,
                },
                Ok(quick_xml::events::Event::Empty(tag)) => match Self::read_extension_or_classifier(&tag) {
                    Some(item) => return Some(item),
                    None => continue
                },
                Ok(quick_xml::events::Event::Eof) => return None,
                Err(_) => return None,
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

    fn read_scope_name(tag: &BytesStart) -> Option<String> {
        let mut scope_name: Option<String> = None;

        for attr in tag.attributes() {
            match attr {
                Ok(real_attr) => match real_attr.key.0 {
                    SCOPE_ATTR_NAME => if !utils::read_attr_value(&real_attr, &mut scope_name) { continue },
                    _ => continue
                },
                Err(_) => continue
            }
        }

        scope_name
    }

    fn read_extension_or_classifier(tag: &BytesStart) -> Option<XesEventLogItem> {
        match tag.name().as_ref() {
            EXTENSION_TAG_NAME => match Self::read_extension(&tag) {
                Some(extension) => Some(XesEventLogItem::Extension(extension)),
                None => None
            },
            CLASSIFIER_TAG_NAME => match Self::read_classifier(&tag) {
                Some(classifier) => Some(XesEventLogItem::Classifier(classifier)),
                None => None,
            },
            _ => None
        }
    }

    fn read_global_internal(
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

                    if let None = map {
                        map = Some(HashMap::new())
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

    fn read_classifier(tag: &BytesStart) -> Option<XesClassifier> {
        let mut name: Option<String> = None;
        let mut keys: Option<Vec<String>> = None;

        for attr in tag.attributes() {
            match attr {
                Ok(real_attr) => match real_attr.key.0 {
                    NAME_ATTR_NAME => if !utils::read_attr_value(&real_attr, &mut name) { return None },
                    KEYS_ATTR_NAME => match String::from_utf8(real_attr.value.into_owned()) {
                        Ok(keys_string) => keys = Some(keys_string.split(" ").map(|s| s.to_owned()).collect()),
                        Err(_) => return None,
                    },
                    _ => continue
                },
                Err(_) => continue,
            }
        }

        if name.is_none() || keys.is_none() { return None }

        Some(XesClassifier { name: name.unwrap(), keys: keys.unwrap() })
    }

    fn read_extension(tag: &BytesStart) -> Option<XesEventLogExtension> {
        let mut name: Option<String> = None;
        let mut prefix: Option<String> = None;
        let mut uri: Option<String> = None;

        for attr in tag.attributes() {
            match attr {
                Ok(real_attr) => match real_attr.key.0 {
                    PREFIX_ATTR_NAME => if !utils::read_attr_value(&real_attr, &mut prefix) { return None },
                    NAME_ATTR_NAME => if !utils::read_attr_value(&real_attr, &mut name) { return None },
                    URI_ATTR_NAME => if !utils::read_attr_value(&real_attr, &mut uri) { return None },
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
