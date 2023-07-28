use std::{cell::RefCell, rc::Rc};

use crate::event_log::core::event::{event::Event, event_hasher::EventHasher};

use super::trace::Trace;


#[derive(Debug)]
pub struct TracesHolder<TTrace, TEvent>
where
    TTrace: Trace<TEvent = TEvent>,
    TEvent: Event,
{
    traces: Vec<Rc<RefCell<TTrace>>>,
}

impl<TTrace, TEvent> TracesHolder<TTrace, TEvent>
where
    TTrace: Trace<TEvent = TEvent>,
    TEvent: Event,
{
    pub fn new(traces: Vec<Rc<RefCell<TTrace>>>) -> TracesHolder<TTrace, TEvent> {
        TracesHolder { traces }
    }

    pub fn get_traces(&self) -> &Vec<Rc<RefCell<TTrace>>> {
        &self.traces
    }

    pub fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&TEvent) -> bool,
    {
        let traces = &mut self.traces;
        for index in (0..traces.len()).rev() {
            traces[index].borrow_mut().remove_events_by(&predicate);
            if traces[index].borrow().get_events().is_empty() {
                traces.remove(index);
            }
        }
    }

    pub fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut TEvent),
    {
        for trace in &self.traces {
            trace.borrow_mut().mutate_events(&mutator);
        }
    }

    pub fn to_hashes_vectors<THasher>(&self) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<TEvent>,
    {
        let mut hashes = Vec::new();
        for trace in &self.traces {
            let mut trace_hashes = Vec::new();
            for event in trace.borrow().get_events() {
                trace_hashes.push(THasher::hash(&event.borrow()));
            }

            hashes.push(trace_hashes);
        }

        hashes
    }
}
