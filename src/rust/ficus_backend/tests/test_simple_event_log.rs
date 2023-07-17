use chrono::Utc;
use core::fmt::Debug;
use ficus_backend::event_log::{
    core::{event::Event, event_log::EventLog, trace::Trace},
    simple::simple_event_log::{SimpleEvent, SimpleEventLog},
};

#[test]
fn test_simple_event_log_creation() {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    let simple_event_log = SimpleEventLog::new(&raw_log);
    assert_eq!(raw_log, simple_event_log.to_raw_vector())
}

#[test]
fn test_set_name() {
    let log = create_simple_event_log();
    let value = String::from_utf8("ASDASD".into()).ok().unwrap();

    execute_test_set_test(
        &log,
        &value,
        |event| event.get_name(),
        |event, value| event.set_name(value),
    )
}

#[test]
fn test_set_date() {
    let log = create_simple_event_log();
    let value = Utc::now();

    execute_test_set_test(
        &log,
        &value,
        |event| event.get_timestamp(),
        |event, value| event.set_timestamp(*value),
    )
}

fn create_simple_event_log() -> SimpleEventLog {
    let raw_log = vec![vec!["A", "B", "C"], vec!["A", "B", "C"]];
    SimpleEventLog::new(&raw_log)
}

fn execute_test_set_test<TValue, TGet, TSet>(
    log: &SimpleEventLog,
    value: &TValue,
    get_property: TGet,
    set_property: TSet,
) where
    TGet: Fn(&SimpleEvent) -> &TValue,
    TSet: Fn(&mut SimpleEvent, &TValue) -> (),
    TValue: PartialEq + Debug,
{
    for trace in log.get_traces() {
        for event in trace.borrow().get_events() {
            set_property(&mut event.borrow_mut(), &value);
        }
    }

    for trace in log.get_traces() {
        for event in trace.borrow().get_events() {
            let event = &event.borrow();
            assert_eq!(get_property(event), value);
        }
    }
}
