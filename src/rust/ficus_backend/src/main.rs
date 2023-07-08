use crate::event_log::{
    core::{event::Event, event_log::EventLog, trace::Trace},
    xes::{reader::file_xes_log_reader::FromFileXesEventLogReader, xes_event_log::XesEventLogImpl},
};

mod event_log;

fn main() {
    let path = r"C:\Users\aeroo\Desktop\Programming\CSharp\pmide\Ficus\src\python\tests\test_data\source\example_logs\exercise1.xes";
    let reader = FromFileXesEventLogReader::new(path).unwrap();

    let log = XesEventLogImpl::new(reader).unwrap();

    println!("GLobals: ");
    for global in log.get_globals() {
        println!("{:?}", global)
    }

    println!("Classifiers: ");
    for classifier in log.get_classifiers() {
        println!("{:?}", classifier);
    }

    println!("Extensions: ");
    for extension in log.get_extensions() {
        println!("{:?}", extension);
    }

    println!("Traces: ");
    for trace in log.get_traces() {
        println!("New trace");
        for event in trace.get_events() {
            println!("{}", event.get_name());
        }
    }

    println!("Hello, world!");
}
