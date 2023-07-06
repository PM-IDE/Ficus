use std::rc::Rc;
use crate::event_log::core::event_log::{Trace, EventLog};
use super::xes_event::XesEventImpl;


pub struct XesEventLogImpl {
    traces: Vec<Rc<XesTraceImpl>>,
}

impl XesEventLogImpl {
    pub fn new<TLogReader, TTraceReader>(event_log_reader: TLogReader) -> Option<XesEventLogImpl>
    where
        TLogReader: Iterator<Item = TTraceReader>,
        TTraceReader: Iterator<Item = XesEventImpl>,
    {
        let mut traces: Vec<Rc<XesTraceImpl>> = Vec::new();
        for trace_reader in event_log_reader {
            match XesTraceImpl::new(trace_reader) {
                Some(trace) => traces.push(Rc::new(trace)),
                None => return None,
            }
        }

        Some(XesEventLogImpl { traces })
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
