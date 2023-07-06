use std::rc::Rc;

use super::event::{Event, EventImpl};

pub trait EventLog {
    type TEvent: Event;
    type TTrace: Trace<TEvent = Self::TEvent>;

    fn get_traces(&self) -> &Vec<Rc<Self::TTrace>>;
}

pub trait Trace {
    type TEvent: Event;
    fn get_events(&self) -> &Vec<Rc<Self::TEvent>>;
}

pub struct EventLogImpl {
    traces: Vec<Rc<TraceImpl>>,
}

impl EventLogImpl {
    pub fn new<TLogReader, TTraceReader>(event_log_reader: TLogReader) -> Option<EventLogImpl>
    where
        TLogReader: Iterator<Item = TTraceReader>,
        TTraceReader: Iterator<Item = EventImpl>,
    {
        let mut traces: Vec<Rc<TraceImpl>> = Vec::new();
        for trace_reader in event_log_reader {
            match TraceImpl::new(trace_reader) {
                Some(trace) => traces.push(Rc::new(trace)),
                None => return None,
            }
        }

        Some(EventLogImpl { traces })
    }
}

impl EventLog for EventLogImpl {
    type TEvent = EventImpl;
    type TTrace = TraceImpl;

    fn get_traces(&self) -> &Vec<Rc<Self::TTrace>> {
        &self.traces
    }
}

pub struct TraceImpl {
    events: Vec<Rc<EventImpl>>,
}

impl TraceImpl {
    pub fn new<TTraceReader>(trace_reader: TTraceReader) -> Option<TraceImpl>
    where
        TTraceReader: Iterator<Item = EventImpl>,
    {
        let mut events: Vec<Rc<EventImpl>> = Vec::new();
        for event in trace_reader {
            events.push(Rc::new(event))
        }

        Some(TraceImpl { events })
    }
}

impl Trace for TraceImpl {
    type TEvent = EventImpl;

    fn get_events(&self) -> &Vec<Rc<Self::TEvent>> {
        &self.events
    }
}
