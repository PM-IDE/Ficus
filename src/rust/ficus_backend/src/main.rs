mod event_log;

use crate::event_log::event::Event;

fn main() {
    let path = "/Users/aero/Programming/pmide/Ficus/src/python/tests/test_data/source/example_logs/exercise1.xes";
    let reader = event_log::file_xes_log_reader::FromFileXesEventLogReader::new(path).unwrap();

    for trace in reader {
        println!("New trace");
        for event in trace {
            println!("{},{},{:?},{:?}", event.get_name(), event.get_timestamp(), event.get_lifecycle(), event.get_payload());
        }
    }

    println!("Hello, world!");
}
