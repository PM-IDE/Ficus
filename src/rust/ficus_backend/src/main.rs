use crate::event_log::{
    core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    xes::{reader::file_xes_log_reader::FromFileXesEventLogReader, xes_event_log::XesEventLogImpl},
};

mod event_log;
mod utils;

fn main() {
    let path = r"/Users/aero/Programming/pmide/PhdDocsAndExperiments/Experiments/exp5/fixed_data/data/inline_merge/SystemArrayPooling/SystemArrayPooling.Program.Main[void...xes";
    let reader = FromFileXesEventLogReader::new(path).unwrap();

    let log = XesEventLogImpl::new(reader).unwrap();
}
