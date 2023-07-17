use ficus_backend::event_log::{simple::simple_event_log::{SimpleEventLog, SimpleEvent}, core::{event_log::EventLog, trace::Trace, event::Event}};
use core::fmt::Debug;

#[test]
fn test_simple_event_log_creation() {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    let simple_event_log = SimpleEventLog::new(&raw_log);
    assert_eq!(raw_log, simple_event_log.to_raw_vector())
}

#[test]
fn test_set_name() {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    let log = SimpleEventLog::new(&raw_log);
    const NEW_NAME: &str = "ASDASD";
    execute_test_set_test(&log, NEW_NAME, |event, &mut mut value| { value = Some(event.get_name()) }, |event, value| { event.set_name(value) })
}

fn execute_test_set_test<TValue, TGet, TSet>(
    log: &SimpleEventLog,
    value: TValue,
    get_property: TGet,
    set_property: TSet)
where
    TGet: Fn(&SimpleEvent, &mut Option<TValue>) -> (),
    TSet: Fn(&mut SimpleEvent, &TValue) -> (),
    TValue: PartialEq + Debug
{
    for trace in log.get_traces() {
        for event in trace.borrow().get_events() {
            set_property(&mut event.borrow_mut(), &value);
        }
    }

    for trace in log.get_traces() {
        for event in trace.borrow().get_events() {
            let event = &event.borrow();
            let mut new_value = None;
            get_property(event, &mut new_value);
            assert_eq!(new_value.unwrap(), value);
        }
    }
}