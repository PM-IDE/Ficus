use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{
    event_log::{
        core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
        simple::simple_event_log::{SimpleEvent, SimpleEventLog, SimpleTrace},
    },
    utils::user_data::Key,
};

use super::repeat_sets::ActivityNode;

#[derive(Debug)]
pub struct ActivityInTraceInfo {
    pub node: Rc<RefCell<ActivityNode>>,
    pub start_pos: usize,
    pub length: usize,
}

impl ActivityInTraceInfo {
    pub fn dump(&self) -> (usize, usize) {
        (self.start_pos, self.start_pos + self.length)
    }
}

pub fn extract_activities_instances(
    log: &Vec<Vec<u64>>,
    activities: Rc<RefCell<Vec<Rc<RefCell<ActivityNode>>>>>,
    should_narrow: bool,
) -> Rc<RefCell<Vec<Vec<ActivityInTraceInfo>>>> {
    let activities_by_size = split_activities_nodes_by_size(&mut activities.borrow_mut());
    let result_ptr = Rc::new(RefCell::new(vec![]));
    let result = &mut result_ptr.borrow_mut();

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

    Rc::clone(&result_ptr)
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

pub enum UndefActivityHandlingStrategy {
    DontInsert,
    InsertAsSingleEvent,
    InsertAllEvents,
}

pub const UNDEF_ACTIVITY_NAME: &str = "UNDEFINED_ACTIVITY";

pub fn underlying_events_key<TEvent>() -> Key<Vec<Rc<RefCell<TEvent>>>>
where
    TEvent: Event + 'static,
{
    Key::new(&"UNDERLYING_EVENTS".to_string())
}

pub fn create_new_log_from_activities_instances<TLog>(
    log: &Rc<RefCell<TLog>>,
    instances: &Vec<Vec<ActivityInTraceInfo>>,
    strategy: &UndefActivityHandlingStrategy,
) -> Rc<RefCell<SimpleEventLog>>
where
    TLog: EventLog,
    TLog::TEvent: 'static,
{
    let new_log_ptr = Rc::new(RefCell::new(SimpleEventLog::empty()));
    let new_log = &mut new_log_ptr.borrow_mut();

    for (instances, trace) in instances.iter().zip(log.borrow().get_traces()) {
        let trace = trace.borrow();
        let new_trace_ptr = Rc::new(RefCell::new(SimpleTrace::empty()));

        let undef_activity_func = |start_index: usize, end_index: usize| match strategy {
            UndefActivityHandlingStrategy::DontInsert => (),
            UndefActivityHandlingStrategy::InsertAsSingleEvent => {
                let event = SimpleEvent::new_with_min_date(UNDEF_ACTIVITY_NAME);
                new_trace_ptr.borrow_mut().push(Rc::new(RefCell::new(event)));
            }
            UndefActivityHandlingStrategy::InsertAllEvents => {
                for i in start_index..end_index {
                    let event = SimpleEvent::new_with_min_date(&trace.get_events()[i].borrow().get_name());
                    new_trace_ptr.borrow_mut().push(Rc::new(RefCell::new(event)));
                }
            }
        };

        let activity_func = |activity: &ActivityInTraceInfo| {
            let ptr = Rc::new(RefCell::new(SimpleEvent::new_with_min_date(
                &activity.node.borrow().name,
            )));

            new_trace_ptr.borrow_mut().push(Rc::clone(&ptr));

            let mut underlying_events = vec![];
            for i in activity.start_pos..(activity.start_pos + activity.length) {
                underlying_events.push(Rc::clone(&trace.get_events()[i]));
            }

            let mut event = ptr.borrow_mut();
            let user_data = event.get_user_data();
            user_data.put(&underlying_events_key(), Box::new(underlying_events));
        };

        process_activities_in_trace(trace.get_events().len(), &instances, undef_activity_func, activity_func);

        new_log.push(new_trace_ptr)
    }

    Rc::clone(&new_log_ptr)
}