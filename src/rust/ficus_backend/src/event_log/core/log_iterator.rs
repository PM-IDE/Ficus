use std::{cell::RefCell, rc::Rc};

use super::{event::event::Event, event_log::EventLog, trace::trace::Trace};

pub struct LogIterator<TLog, TEvent, TEventProcessor, TResult>
where
    TEvent: Event,
    TLog: EventLog<TEvent = TEvent>,
    TEventProcessor: Fn(&TEvent) -> TResult,
{
    log: Rc<RefCell<TLog>>,
    event_processor: Rc<TEventProcessor>,
    trace_index: usize,
}

impl<TLog, TEvent, TEventProcessor, TResult> LogIterator<TLog, TEvent, TEventProcessor, TResult>
where
    TEvent: Event,
    TLog: EventLog<TEvent = TEvent>,
    TEventProcessor: Fn(&TEvent) -> TResult,
{
    pub fn new(log: Rc<RefCell<TLog>>, event_processor: Rc<TEventProcessor>) -> Self {
        Self {
            log,
            event_processor,
            trace_index: 0,
        }
    }
}

impl<TLog, TTrace, TEvent, TEventProcessor, TResult> Iterator for LogIterator<TLog, TEvent, TEventProcessor, TResult>
where
    TEvent: Event,
    TTrace: Trace<TEvent = TEvent>,
    TLog: EventLog<TTrace = TTrace, TEvent = TEvent>,
    TEventProcessor: Fn(&TEvent) -> TResult,
{
    type Item = TraceIterator<TTrace, TEvent, TEventProcessor, TResult>;

    fn next(&mut self) -> Option<Self::Item> {
        let log = self.log.borrow();
        let traces = log.get_traces();

        if self.trace_index >= traces.len() {
            None
        } else {
            let item = Some(TraceIterator {
                trace: Rc::clone(&traces[self.trace_index]),
                event_processor: Rc::clone(&self.event_processor),
                event_index: 0,
            });

            self.trace_index += 1;

            item
        }
    }
}

pub struct TraceIterator<TTrace, TEvent, TEventProcessor, TResult>
where
    TTrace: Trace<TEvent = TEvent>,
    TEventProcessor: Fn(&TEvent) -> TResult,
{
    trace: Rc<RefCell<TTrace>>,
    event_processor: Rc<TEventProcessor>,
    event_index: usize,
}

impl<TTrace, TEvent, TEventProcessor, TResult> Iterator for TraceIterator<TTrace, TEvent, TEventProcessor, TResult>
where
    TTrace: Trace<TEvent = TEvent>,
    TEventProcessor: Fn(&TEvent) -> TResult,
{
    type Item = TResult;

    fn next(&mut self) -> Option<Self::Item> {
        let trace = self.trace.borrow();
        let events = trace.get_events();
        if self.event_index >= events.len() {
            None
        } else {
            let item = (&self.event_processor)(&events[self.event_index].borrow());
            self.event_index += 1;

            Some(item)
        }
    }
}

impl<TTrace, TEvent, TEventProcessor, TResult> TraceIterator<TTrace, TEvent, TEventProcessor, TResult>
where
    TTrace: Trace<TEvent = TEvent>,
    TEventProcessor: Fn(&TEvent) -> TResult,
{
    pub fn new(trace: Rc<RefCell<TTrace>>, event_processor: Rc<TEventProcessor>) -> Self {
        Self {
            trace,
            event_processor,
            event_index: 0,
        }
    }

    pub fn step_back(&mut self) {
        if self.event_index > 0 {
            self.event_index -= 1;
        }
    }
}
