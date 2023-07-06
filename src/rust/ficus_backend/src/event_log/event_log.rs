use std::rc::Rc;

use super::event::Event;

pub struct EventLog<TEvent>
where
    TEvent: Event,
{
    traces: Vec<Rc<Trace<TEvent>>>,
}

impl<TEvent> EventLog<TEvent>
where
    TEvent: Event,
{
    pub fn new<TLogReader, TTraceReader>(event_log_reader: TLogReader) -> Option<EventLog<TEvent>>
    where
        TLogReader: Iterator<Item = TTraceReader>,
        TTraceReader: Iterator<Item = TEvent>,
    {
        let mut traces: Vec<Rc<Trace<TEvent>>> = Vec::new();
        for trace_reader in event_log_reader {
            match Trace::new(trace_reader) {
                Some(trace) => traces.push(Rc::new(trace)),
                None => return None,
            }
        }

        Some(EventLog { traces })
    }
}

pub struct Trace<TEvent>
where
    TEvent: Event,
{
    events: Vec<Rc<TEvent>>,
}

impl<TEvent> Trace<TEvent>
where
    TEvent: Event,
{
    pub fn new<TTraceReader>(trace_reader: TTraceReader) -> Option<Trace<TEvent>>
    where
        TTraceReader: Iterator<Item = TEvent>,
    {
        let mut events: Vec<Rc<TEvent>> = Vec::new();
        for event in trace_reader {
            events.push(Rc::new(event))
        }

        Some(Trace { events })
    }
}
