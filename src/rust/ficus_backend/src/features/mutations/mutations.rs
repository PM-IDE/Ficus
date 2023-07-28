use crate::event_log::core::{event::event::Event, event_log::EventLog};

pub fn rename_events<TLog, TEvent, TFilter>(log: &mut TLog, new_name: &str, filter: TFilter)
where
    TLog: EventLog<TEvent = TEvent>,
    TEvent: Event,
    TFilter: Fn(&TEvent) -> bool,
{
    log.mutate_events(|event| {
        if filter(event) {
            event.set_name(&new_name.to_owned())
        }
    })
}
