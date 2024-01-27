use super::{
    reader::file_xes_log_reader::XesEventLogItem,
    shared::{XesClassifier, XesEventLogExtension},
    xes_event::XesEventImpl,
    xes_trace::XesTraceImpl,
};

use crate::event_log::core::{
    event::{event::EventPayloadValue, event_hasher::EventHasher, events_holder::EventSequenceInfo},
    event_log::EventLog,
    trace::traces_holder::TracesHolder,
};
use crate::utils::vec_utils;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct XesEventLogImpl {
    traces_holder: TracesHolder<XesTraceImpl>,
    globals: HashMap<String, HashMap<String, EventPayloadValue>>,
    extensions: Vec<XesEventLogExtension>,
    classifiers: Vec<XesClassifier>,
    properties: HashMap<String, EventPayloadValue>,
}

impl XesEventLogImpl {
    pub fn globals_map(&self) -> &HashMap<String, HashMap<String, EventPayloadValue>> {
        &self.globals
    }

    pub fn extensions(&self) -> &Vec<XesEventLogExtension> {
        &self.extensions
    }

    pub fn classifiers(&self) -> &Vec<XesClassifier> {
        &self.classifiers
    }

    pub fn properties_map(&self) -> &HashMap<String, EventPayloadValue> {
        &self.properties
    }

    pub fn ordered_properties(&self) -> Vec<(&String, &EventPayloadValue)> {
        let mut properties = Vec::new();
        for (key, value) in self.properties_map() {
            properties.push((key, value));
        }

        vec_utils::sort_by_first(&mut properties);
        properties
    }

    pub fn ordered_globals(&self) -> Vec<(&String, Vec<(&String, &EventPayloadValue)>)> {
        let mut globals = Vec::new();
        for (key, value) in self.globals_map() {
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
            traces_holder: TracesHolder::new(traces),
            globals,
            extensions,
            classifiers,
            properties,
        };

        Some(log)
    }
}

impl Clone for XesEventLogImpl {
    fn clone(&self) -> Self {
        Self {
            traces_holder: self.traces_holder.clone(),
            globals: self.globals.clone(),
            extensions: self.extensions.clone(),
            classifiers: self.classifiers.clone(),
            properties: self.properties.clone(),
        }
    }
}

impl EventLog for XesEventLogImpl {
    type TEvent = XesEventImpl;
    type TTraceInfo = EventSequenceInfo;
    type TTrace = XesTraceImpl;

    fn empty() -> Self {
        Self {
            traces_holder: TracesHolder::empty(),
            globals: HashMap::new(),
            extensions: Vec::new(),
            classifiers: Vec::new(),
            properties: HashMap::new(),
        }
    }

    fn traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
        &self.traces_holder.get_traces()
    }

    fn push(&mut self, trace: Rc<RefCell<Self::TTrace>>) {
        self.traces_holder.push(trace);
    }

    fn to_raw_vector(&self) -> Vec<Vec<String>> {
        self.traces_holder.to_raw_vector()
    }

    fn to_hashes_event_log<THasher>(&self, hasher: &THasher) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<Self::TEvent>,
    {
        self.traces_holder.to_hashes_vectors(hasher)
    }

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool,
    {
        self.traces_holder.filter_events_by(predicate);
    }

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent),
    {
        self.traces_holder.mutate_events(mutator);
    }

    fn filter_traces(&mut self, predicate: &impl Fn(&Self::TTrace, &usize) -> bool) {
        self.traces_holder.filter_traces(predicate);
    }
}
