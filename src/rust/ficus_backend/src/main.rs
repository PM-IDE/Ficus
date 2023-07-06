mod event_log;

use event_log::event_log::EventLog;

use crate::event_log::event::Event;

fn main() {
    let path = "/Users/aero/Programming/pmide/Ficus/src/python/tests/test_data/source/example_logs/exercise1.xes";
    let reader = event_log::file_xes_log_reader::FromFileXesEventLogReader::new(path).unwrap();

    let log = EventLog::new(reader);

    println!("Hello, world!");
}
