use crate::event_log::core::event_log::EventLog;

fn filter_log_by_name<T>(log: &T)
where
    T: EventLog,
{
    for trace in log.get_traces() {}
}
