use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    ops::{Deref, DerefMut},
    rc::Rc,
    str::FromStr,
};

use crate::{
    event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    pipelines::aliases::TracesActivities,
    utils::user_data::{keys::DefaultKey, user_data::UserData},
};

use super::repeat_sets::{ActivityNode, SubArrayWithTraceIndex};

#[derive(Debug, Clone)]
pub struct ActivityInTraceInfo {
    pub node: Rc<RefCell<ActivityNode>>,
    pub start_pos: usize,
    pub length: usize,
}

pub const UNATTACHED_SUB_TRACE_NAME: &str = "UndefinedActivity";

pub enum SubTraceKind<'a> {
    Attached(&'a ActivityInTraceInfo),
    Unattached(usize, usize),
}

impl ActivityInTraceInfo {
    pub fn dump(&self) -> (usize, usize) {
        (self.start_pos, self.start_pos + self.length)
    }
}

pub fn extract_activities_instances(
    log: &Vec<Vec<u64>>,
    activities: &mut Vec<Rc<RefCell<ActivityNode>>>,
    should_narrow: bool,
) -> Vec<Vec<ActivityInTraceInfo>> {
    let activities_by_size = split_activities_nodes_by_size(activities);
    let mut result = vec![];

    for trace in log {
        let mut trace_activities = vec![];
        let mut index = None;
        let mut current_activity = None;
        let mut last_activity_start_index = None;
        let mut current_event_classes = HashSet::new();

        while index.is_none() || index.unwrap() < trace.len() {
            if index.is_none() {
                index = Some(0);
            } else {
                *index.as_mut().unwrap() += 1;
            }

            if index.unwrap() >= trace.len() {
                break;
            }

            let event_hash = trace[index.unwrap()];
            if current_activity.is_none() {
                let mut found_activity = false;
                for activities in activities_by_size.borrow().iter() {
                    for activity in activities {
                        if activity.borrow().event_classes.contains(&event_hash) {
                            current_activity = Some(Rc::clone(activity));
                            last_activity_start_index = Some(index.unwrap());
                            found_activity = true;
                            break;
                        }
                    }

                    if found_activity {
                        current_event_classes.clear();
                        current_event_classes.insert(event_hash);
                        break;
                    }
                }

                continue;
            }

            if !current_activity
                .as_ref()
                .unwrap()
                .borrow()
                .event_classes
                .contains(&event_hash)
            {
                let mut new_set = current_event_classes.clone();
                new_set.insert(event_hash);

                let mut found_new_set = false;
                for activities_set in activities_by_size.borrow().iter() {
                    if activities_set.len() == 0
                        || activities_set[0].borrow().len() < current_activity.as_ref().unwrap().borrow().len()
                    {
                        continue;
                    }

                    for activity in activities_set {
                        if new_set.is_subset(&activity.borrow().event_classes) {
                            current_activity = Some(Rc::clone(activity));
                            found_new_set = true;
                            break;
                        }
                    }

                    if found_new_set {
                        current_event_classes.insert(event_hash);
                        break;
                    }
                }

                if !found_new_set {
                    if should_narrow {
                        let activity = narrow_activity(current_activity.as_ref().unwrap(), &current_event_classes);
                        current_activity = Some(activity);
                    }

                    let activity_instance = ActivityInTraceInfo {
                        node: Rc::clone(current_activity.as_ref().unwrap()),
                        start_pos: last_activity_start_index.unwrap(),
                        length: index.unwrap() - last_activity_start_index.unwrap(),
                    };

                    trace_activities.push(activity_instance);

                    current_activity = None;
                    current_event_classes.clear();
                    last_activity_start_index = None;
                    *index.as_mut().unwrap() -= 1;
                }
            } else {
                current_event_classes.insert(event_hash);
            }
        }

        if last_activity_start_index.is_some() {
            if should_narrow {
                let activity = narrow_activity(current_activity.as_ref().unwrap(), &current_event_classes);
                current_activity = Some(activity);
            }

            let activity_instance = ActivityInTraceInfo {
                node: Rc::clone(current_activity.as_ref().unwrap()),
                start_pos: last_activity_start_index.unwrap(),
                length: index.unwrap() - last_activity_start_index.unwrap(),
            };

            trace_activities.push(activity_instance);
        }

        result.push(trace_activities);
    }

    result
}

fn split_activities_nodes_by_size(
    activities: &mut Vec<Rc<RefCell<ActivityNode>>>,
) -> Rc<RefCell<Vec<Vec<Rc<RefCell<ActivityNode>>>>>> {
    if activities.is_empty() {
        return Rc::new(RefCell::new(vec![]));
    }

    activities.sort_by(|first, second| first.borrow().len().cmp(&second.borrow().len()));
    let mut current_length = activities[0].borrow().len();
    let result_ptr = Rc::new(RefCell::new(vec![vec![Rc::clone(activities.get(0).unwrap())]]));
    let result = &mut result_ptr.borrow_mut();

    for activity in activities.iter() {
        if activity.borrow().len() != current_length {
            result.push(vec![]);
            current_length = activity.borrow().len();
        }

        result.last_mut().unwrap().push(Rc::clone(activity));
    }

    Rc::clone(&result_ptr)
}

fn narrow_activity(node_ptr: &Rc<RefCell<ActivityNode>>, activities_set: &HashSet<u64>) -> Rc<RefCell<ActivityNode>> {
    let mut q = VecDeque::new();
    let node = node_ptr.borrow();
    for child in &node.children {
        q.push_back(Rc::clone(child));
    }

    let mut result = vec![];
    while !q.is_empty() {
        let current_activity_ptr = q.pop_front().unwrap();
        let current_activity = current_activity_ptr.borrow();

        if current_activity.event_classes.is_superset(&activities_set) {
            result.push(Rc::clone(&current_activity_ptr));
            for child_node in &current_activity.children {
                q.push_back(Rc::clone(child_node));
            }
        }
    }

    if result.is_empty() {
        return Rc::clone(&node_ptr);
    }

    let result = result
        .iter()
        .max_by(|first, second| first.borrow().len().cmp(&second.borrow().len()));

    Rc::clone(result.unwrap())
}

pub fn process_activities_in_trace<TUndefActivityHandleFunc, TActivityHandleFunc>(
    trace_length: usize,
    activities_instances: &Vec<ActivityInTraceInfo>,
    mut undefined_activity_func: TUndefActivityHandleFunc,
    mut activity_func: TActivityHandleFunc,
) where
    TUndefActivityHandleFunc: FnMut(usize, usize) -> (),
    TActivityHandleFunc: FnMut(&ActivityInTraceInfo) -> (),
{
    let mut index = 0;
    for instance in activities_instances {
        if index < instance.start_pos {
            undefined_activity_func(index, instance.start_pos);
        }

        activity_func(instance);
        index = instance.start_pos + instance.length;
    }

    if index < trace_length {
        undefined_activity_func(index, trace_length);
    }
}

pub enum UndefActivityHandlingStrategy<TEvent> {
    DontInsert,
    InsertAsSingleEvent(Box<dyn Fn() -> Rc<RefCell<TEvent>>>),
    InsertAllEvents,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AdjustingMode {
    FromAllLog,
    FromUnattachedSubTraces,
}

impl FromStr for AdjustingMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FromAllLog" => Ok(AdjustingMode::FromAllLog),
            "FromUnattachedSubTraces" => Ok(AdjustingMode::FromUnattachedSubTraces),
            _ => Err(()),
        }
    }
}

pub const UNDEF_ACTIVITY_NAME: &str = "UNDEFINED_ACTIVITY";

pub fn underlying_events_key<TEvent>() -> DefaultKey<Vec<Rc<RefCell<TEvent>>>>
where
    TEvent: Event + 'static,
{
    DefaultKey::new("UNDERLYING_EVENTS".to_string())
}

pub fn create_new_log_from_activities_instances<TLog, TEventFactory>(
    log: &TLog,
    instances: &Vec<Vec<ActivityInTraceInfo>>,
    strategy: &UndefActivityHandlingStrategy<TLog::TEvent>,
    event_from_activity_factory: &TEventFactory,
) -> TLog
where
    TLog: EventLog,
    TLog::TEvent: 'static,
    TEventFactory: Fn(&ActivityInTraceInfo) -> Rc<RefCell<TLog::TEvent>>,
{
    let mut new_log = TLog::empty();

    for (instances, trace) in instances.iter().zip(log.get_traces()) {
        let trace = trace.borrow();
        let new_trace_ptr = Rc::new(RefCell::new(TLog::TTrace::empty()));

        let undef_activity_func = |start_index: usize, end_index: usize| match strategy {
            UndefActivityHandlingStrategy::DontInsert => (),
            UndefActivityHandlingStrategy::InsertAsSingleEvent(factory) => {
                new_trace_ptr.borrow_mut().push(factory());
            }
            UndefActivityHandlingStrategy::InsertAllEvents => {
                for i in start_index..end_index {
                    let event = trace.get_events()[i].borrow().clone();
                    new_trace_ptr.borrow_mut().push(Rc::new(RefCell::new(event)));
                }
            }
        };

        let activity_func = |activity: &ActivityInTraceInfo| {
            let ptr = event_from_activity_factory(activity);

            new_trace_ptr.borrow_mut().push(Rc::clone(&ptr));

            let mut underlying_events = vec![];
            for i in activity.start_pos..(activity.start_pos + activity.length) {
                underlying_events.push(Rc::clone(&trace.get_events()[i]));
            }

            let mut event = ptr.borrow_mut();
            let user_data = event.get_user_data();
            user_data.put_any(&underlying_events_key::<TLog::TEvent>(), underlying_events);
        };

        process_activities_in_trace(trace.get_events().len(), &instances, undef_activity_func, activity_func);

        new_log.push(new_trace_ptr)
    }

    new_log
}

pub fn add_unattached_activities(
    log: &Vec<Vec<u64>>,
    activities: &mut Vec<Rc<RefCell<ActivityNode>>>,
    existing_instances: &Vec<Vec<ActivityInTraceInfo>>,
    min_numbers_of_events: usize,
    should_narrow: bool,
) -> Vec<Vec<ActivityInTraceInfo>> {
    let mut new_activities = vec![];

    for (trace_activities, trace) in existing_instances.iter().zip(log) {
        let mut new_trace_activities = vec![];

        let handle_unattached_events = |start_index: usize, end_index: usize| {
            if end_index - start_index < min_numbers_of_events {
                return;
            }

            let activities = extract_activities_instances(&vec![trace.clone()], activities, should_narrow);
            new_trace_activities.extend(
                activities[0]
                    .iter()
                    .map(|instance| ActivityInTraceInfo {
                        node: Rc::clone(&instance.node),
                        start_pos: start_index + instance.start_pos,
                        length: instance.length,
                    })
                    .collect::<Vec<ActivityInTraceInfo>>(),
            );
        };

        let length = trace.len();
        process_activities_in_trace(length, trace_activities, handle_unattached_events, |_| {});

        new_trace_activities.extend(trace_activities.iter().map(|instance| instance.clone()));
        new_trace_activities.sort_by(|first, second| first.start_pos.cmp(&second.start_pos));

        new_activities.push(new_trace_activities);
    }

    new_activities
}

pub fn create_logs_for_activities<TLog>(
    log: &TLog,
    activities: &Vec<Vec<ActivityInTraceInfo>>,
    activity_level: usize,
) -> HashMap<String, Rc<RefCell<TLog>>>
where
    TLog: EventLog,
{
    let mut activities_to_logs: HashMap<String, Rc<RefCell<TLog>>> = HashMap::new();
    for (trace_activities, trace) in activities.iter().zip(log.get_traces()) {
        let activity_handler = |activity_info: &ActivityInTraceInfo| {
            if activity_level != activity_info.node.borrow().level {
                return;
            }

            let new_trace_ptr = Rc::new(RefCell::new(TLog::TTrace::empty()));
            let mut new_trace = new_trace_ptr.borrow_mut();

            let start = activity_info.start_pos;
            let end = start + activity_info.length;

            let trace = trace.borrow();
            let events = trace.get_events();

            for i in start..end {
                new_trace.push(Rc::new(RefCell::new(events[i].borrow().clone())));
            }

            let name = &activity_info.node.borrow().name;
            if let Some(activity_log) = activities_to_logs.get_mut(name) {
                activity_log.borrow_mut().push(Rc::clone(&new_trace_ptr));
            } else {
                let log = Rc::new(RefCell::new(TLog::empty()));
                log.borrow_mut().push(Rc::clone(&new_trace_ptr));

                activities_to_logs.insert(name.to_owned(), log);
            }
        };

        let length = trace.borrow().get_events().len();
        process_activities_in_trace(length, trace_activities, |_, _| {}, activity_handler);
    }

    activities_to_logs
}

pub fn create_activity_name<TLog>(log: &TLog, sub_array: &SubArrayWithTraceIndex) -> String
where
    TLog: EventLog,
{
    let mut name = String::new();

    let left = sub_array.sub_array.start_index;
    let right = left + sub_array.sub_array.length;
    let trace = log.get_traces().get(sub_array.trace_index).unwrap().borrow();
    let events = trace.get_events();
    for event in &events[left..right] {
        name.push_str(event.borrow().get_name());
    }

    name
}

pub fn count_underlying_events<TLog>(log: &TLog) -> usize
where
    TLog: EventLog,
{
    let mut count = 0usize;
    for trace in log.get_traces() {
        let mut trace_count = 0usize;
        for event in trace.borrow().get_events() {
            trace_count += count_underlying_events_for_event(event.borrow_mut().deref_mut());
        }

        count += trace_count;
    }

    count
}

fn count_underlying_events_for_event<TEvent>(event: &mut TEvent) -> usize
where
    TEvent: Event + 'static,
{
    let key = underlying_events_key::<TEvent>();

    if let Some(underlying_events) = event.get_user_data().get_concrete_mut(&key) {
        let mut result = 0usize;
        for underlying_event in underlying_events {
            result += count_underlying_events_for_event(underlying_event.borrow_mut().deref_mut())
        }

        result
    } else {
        1
    }
}

pub fn create_log_from_unattached_events<TLog>(log: &TLog, activities: &TracesActivities) -> TLog
where
    TLog: EventLog,
{
    let mut new_log = TLog::empty();

    for (trace, trace_activities) in log.get_traces().into_iter().zip(activities) {
        let trace = trace.borrow();
        let mut new_trace = TLog::TTrace::empty();

        let process_undef_activity = |start, end| {
            for event in &trace.get_events()[start..end] {
                new_trace.push(event.clone());
            }
        };

        process_activities_in_trace(
            trace.get_events().len(),
            trace_activities,
            process_undef_activity,
            |_| {},
        );

        new_log.push(Rc::new(RefCell::new(new_trace)));
    }

    new_log
}
