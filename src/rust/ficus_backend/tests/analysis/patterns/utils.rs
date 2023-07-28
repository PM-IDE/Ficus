use ficus_backend::{
    event_log::{
        core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
        simple::simple_event_log::SimpleEventLog,
    },
    features::analysis::patterns::repeat_sets::SubArrayWithTraceIndex,
};

pub fn create_activity_name(log: &SimpleEventLog, sub_array: &SubArrayWithTraceIndex) -> String {
    let mut name = String::new();

    let left = sub_array.sub_array.start_index;
    let right = left + sub_array.sub_array.length;
    let trace = log.get_traces().get(sub_array.trace_index).unwrap().borrow();
    let events = trace.get_events();
    for event in &events[left..right] {
        name.push_str(event.borrow().get_name());
    }

    name
}
