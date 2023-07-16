use super::{
    reader::file_xes_log_reader::XesEventLogItem,
    shared::{XesClassifier, XesEventLogExtension},
    xes_event::XesEventImpl,
};

use crate::event_log::core::{event::EventPayloadValue, event_log::EventLog, trace::Trace};
use crate::utils::vec_utils;
use std::{collections::HashMap, rc::Rc, cell::RefCell};

pub struct XesEventLogImpl {
    traces: Vec<Rc<RefCell<XesTraceImpl>>>,
    globals: HashMap<String, HashMap<String, String>>,
    extensions: Vec<XesEventLogExtension>,
    classifiers: Vec<XesClassifier>,
    properties: HashMap<String, EventPayloadValue>,
}

impl XesEventLogImpl {
    pub fn get_globals_map(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.globals
    }

    pub fn get_extensions(&self) -> &Vec<XesEventLogExtension> {
        &self.extensions
    }

    pub fn get_classifiers(&self) -> &Vec<XesClassifier> {
        &self.classifiers
    }

    pub fn get_properties_map(&self) -> &HashMap<String, EventPayloadValue> {
        &self.properties
    }

    pub fn get_ordered_properties(&self) -> Vec<(&String, &EventPayloadValue)> {
        let mut properties = Vec::new();
        for (key, value) in self.get_properties_map() {
            properties.push((key, value));
        }

        vec_utils::sort_by_first(&mut properties);
        properties
    }

    pub fn get_ordered_globals(&self) -> Vec<(&String, Vec<(&String, &String)>)> {
        let mut globals = Vec::new();
        for (key, value) in self.get_globals_map() {
            let mut defaults = Vec::new();
            for (default_key, default_value) in value {
                defaults.push((default_key, default_value));
            }

            vec_utils::sort_by_first(&mut defaults);
            globals.push((key, defaults));
        }

        vec_utils::sort_by_first(&mut globals);
        globals
    }
}

impl XesEventLogImpl {
    pub fn new<TLogReader>(event_log_reader: TLogReader) -> Option<XesEventLogImpl>
    where
        TLogReader: Iterator<Item = XesEventLogItem>,
    {
        let mut extensions = Vec::new();
        let mut globals = HashMap::new();
        let mut traces = Vec::new();
        let mut classifiers = Vec::new();
        let mut properties = HashMap::new();

        for item in event_log_reader {
            match item {
                XesEventLogItem::Trace(trace_reader) => match XesTraceImpl::new(trace_reader) {
                    Some(trace) => traces.push(Rc::new(RefCell::new(trace))),
                    None => continue,
                },
                XesEventLogItem::Global(global) => _ = globals.insert(global.scope, global.default_values),
                XesEventLogItem::Extension(extension) => extensions.push(extension),
                XesEventLogItem::Classifier(classifier) => classifiers.push(classifier),
                XesEventLogItem::Property(property) => _ = properties.insert(property.name, property.value),
            }
        }

        let log = XesEventLogImpl {
            traces,
            globals,
            extensions,
            classifiers,
            properties,
        };

        Some(log)
    }
}

impl EventLog for XesEventLogImpl {
    type TEvent = XesEventImpl;
    type TTrace = XesTraceImpl;

    fn get_traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
        &self.traces
    }
}

pub struct XesTraceImpl {
    events: Vec<Rc<RefCell<XesEventImpl>>>,
}

impl XesTraceImpl {
    pub fn new<TTraceReader>(trace_reader: TTraceReader) -> Option<XesTraceImpl>
    where
        TTraceReader: Iterator<Item = XesEventImpl>,
    {
        let mut events: Vec<Rc<RefCell<XesEventImpl>>> = Vec::new();
        for event in trace_reader {
            events.push(Rc::new(RefCell::new(event)));
        }

        Some(XesTraceImpl { events })
    }
}

impl Trace for XesTraceImpl {
    type TEvent = XesEventImpl;

    fn get_events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>> {
        &self.events
    }
}
