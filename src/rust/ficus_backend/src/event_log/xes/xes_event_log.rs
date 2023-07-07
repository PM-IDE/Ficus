use std::{rc::Rc, collections::HashMap};
use crate::event_log::core::{event_log::EventLog, trace::Trace};
use super::{xes_event::XesEventImpl, shared::XesEventLogExtension, file_xes_log_reader::XesEventLogItem};


pub struct XesEventLogImpl {
    traces: Vec<Rc<XesTraceImpl>>,
    globals: HashMap<String, HashMap<String, String>>,
    extensions: Vec<XesEventLogExtension>
}

impl XesEventLogImpl {
    pub fn new<TLogReader>(event_log_reader: TLogReader) -> Option<XesEventLogImpl>
    where
        TLogReader: Iterator<Item = XesEventLogItem>
    {
        let mut extensions = Vec::new();
        let mut globals = HashMap::new();
        let mut traces: Vec<Rc<XesTraceImpl>> = Vec::new();

        for item in event_log_reader {
            match item {
                XesEventLogItem::Trace(trace_reader) => match XesTraceImpl::new(trace_reader) {
                    Some(trace) => traces.push(Rc::new(trace)),
                    None => continue,
                },
                XesEventLogItem::Global(global) => _ = globals.insert(global.scope, global.default_values),
                XesEventLogItem::Extension(extension) => extensions.push(extension),
            }
        }

        Some(XesEventLogImpl { traces, globals, extensions })
    }
}

impl EventLog for XesEventLogImpl {
    type TEvent = XesEventImpl;
    type TTrace = XesTraceImpl;

    fn get_traces(&self) -> &Vec<Rc<Self::TTrace>> {
        &self.traces
    }
}

pub struct XesTraceImpl {
    events: Vec<Rc<XesEventImpl>>,
}

impl XesTraceImpl {
    pub fn new<TTraceReader>(trace_reader: TTraceReader) -> Option<XesTraceImpl>
    where
        TTraceReader: Iterator<Item = XesEventImpl>,
    {
        let mut events: Vec<Rc<XesEventImpl>> = Vec::new();
        for event in trace_reader {
            events.push(Rc::new(event))
        }

        Some(XesTraceImpl { events })
    }
}

impl Trace for XesTraceImpl {
    type TEvent = XesEventImpl;

    fn get_events(&self) -> &Vec<Rc<Self::TEvent>> {
        &self.events
    }
}
