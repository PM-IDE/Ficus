use crate::event_log::xes_event_log::XesEventLogImpl;

mod event_log;

fn main() {
    let path = "/Users/aero/Programming/pmide/Ficus/src/python/tests/test_data/source/example_logs/exercise1.xes";
    let reader = event_log::file_xes_log_reader::FromFileXesEventLogReader::new(path).unwrap();

    let log = XesEventLogImpl::new(reader);

    println!("Hello, world!");
}
