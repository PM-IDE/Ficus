use crate::event_log::core::{event::event::Event, event_log::EventLog};

pub fn rename_events<TLog, TFilter>(log: &mut TLog, new_name: &str, filter: TFilter)
where
    TLog: EventLog,
    TFilter: Fn(&TLog::TEvent) -> bool,
{
    log.mutate_events(|event| {
        if filter(event) {
            event.set_name(new_name.to_owned())
        }
    })
}
