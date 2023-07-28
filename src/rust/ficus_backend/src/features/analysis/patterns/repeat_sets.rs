use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::utils::hash_utils::calculate_poly_hash_for_collection;

use super::tandem_arrays::SubArrayInTraceInfo;

#[derive(Clone, Copy, Debug)]
pub struct SubArrayWithTraceIndex {
    pub sub_array: SubArrayInTraceInfo,
    pub trace_index: usize,
}

impl SubArrayWithTraceIndex {
    pub fn new(sub_array: SubArrayInTraceInfo, trace_index: usize) -> Self {
        Self { sub_array, trace_index }
    }

    pub fn dump(&self) -> (usize, usize, usize) {
        (self.sub_array.start_index, self.sub_array.length, self.trace_index)
    }
}

pub fn build_repeat_sets(
    log: &Vec<Vec<u64>>,
    patterns: &Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>>,
) -> Rc<RefCell<Vec<SubArrayWithTraceIndex>>> {
    let mut repeat_sets = HashMap::new();
    let mut set = HashSet::new();
    let mut vec: Vec<u64> = vec![];
    let mut trace_index = 0;

    for (trace, trace_patterns) in log.into_iter().zip(patterns.borrow().iter()) {
        for pattern in trace_patterns {
            let start = pattern.start_index;
            let end = start + pattern.length;

            set.clear();
            for element in &trace[start..end] {
                set.insert(*element);
            }

            vec.clear();
            vec.extend(&set);
            vec.sort();

            let hash = calculate_poly_hash_for_collection(vec.as_slice());

            if !repeat_sets.contains_key(&hash) {
                repeat_sets.insert(hash, SubArrayWithTraceIndex::new(*pattern, trace_index));
            }
        }

        trace_index += 1;
    }

    let mut result = vec![];
    for repeat_set in repeat_sets.values().into_iter() {
        result.push(*repeat_set);
    }

    result.sort_by(|first, second| {
        if first.trace_index == second.trace_index {
            if first.sub_array.start_index != second.sub_array.start_index {
                first.sub_array.start_index.cmp(&second.sub_array.start_index)
            } else {
                first.sub_array.length.cmp(&second.sub_array.length)
            }
        } else {
            first.trace_index.cmp(&second.trace_index)
        }
    });

    Rc::new(RefCell::new(result))
}

#[derive(Debug)]
pub struct ActivityNode {
    pub repeat_set: SubArrayWithTraceIndex,
    pub event_classes: HashSet<u64>,
    pub children: Vec<Rc<RefCell<ActivityNode>>>,
    pub level: usize,
    pub name: String,
}

impl ActivityNode {
    fn len(&self) -> usize {
        self.event_classes.len()
    }

    fn contains_other(&self, other_node: &ActivityNode) -> bool {
        self.event_classes.is_superset(&other_node.event_classes)
    }
}

pub struct ActivitiesDiscoveryContext<TNameCreator>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub activity_level: usize,
    pub name_creator: TNameCreator,
}

impl<TNameCreator> ActivitiesDiscoveryContext<TNameCreator>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub fn new(activity_level: usize, name_creator: TNameCreator) -> Self {
        Self {
            activity_level,
            name_creator,
        }
    }
}

pub fn build_repeat_set_tree_from_repeats<TNameCreator>(
    log: &Vec<Vec<u64>>,
    repeats: &Rc<RefCell<Vec<SubArrayWithTraceIndex>>>,
    context: ActivitiesDiscoveryContext<TNameCreator>,
) -> Rc<RefCell<Vec<Rc<RefCell<ActivityNode>>>>>
where
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    let repeats = repeats.borrow();
    if repeats.len() == 0 {
        return Rc::new(RefCell::new(vec![]));
    }

    let extract_events_set = |repeat_set: &SubArrayWithTraceIndex| -> HashSet<u64> {
        let trace = log.get(repeat_set.trace_index).unwrap();
        let mut set = HashSet::new();
        let array = repeat_set.sub_array;
        for index in array.start_index..(array.start_index + array.length) {
            set.insert(trace[index]);
        }

        set
    };

    let create_activity_node = |repeat_set: &SubArrayWithTraceIndex| {
        let events_set = extract_events_set(repeat_set);
        Rc::new(RefCell::new(ActivityNode {
            repeat_set: *repeat_set,
            event_classes: events_set,
            children: vec![],
            level: context.activity_level,
            name: (&context.name_creator)(repeat_set),
        }))
    };

    let mut activity_nodes = repeats
        .iter()
        .map(|repeat| create_activity_node(&repeat))
        .collect::<Vec<Rc<RefCell<ActivityNode>>>>();

    activity_nodes.sort_by(|first, second| second.borrow().len().cmp(&first.borrow().len()));
    let max_length = activity_nodes[0].borrow().len();
    let top_level_nodes_ptr = Rc::new(RefCell::new(vec![Rc::clone(&activity_nodes[0])]));
    let top_level_nodes = &mut top_level_nodes_ptr.borrow_mut();
    let mut next_length_index = 1;
    let mut current_length = max_length;

    for i in 1..activity_nodes.len() {
        let node_ptr = &activity_nodes[i];
        if node_ptr.borrow().len() != max_length {
            next_length_index = i;
            current_length = node_ptr.borrow().len();
            break;
        }

        top_level_nodes.push(Rc::clone(node_ptr));
    }

    if top_level_nodes.len() == activity_nodes.len() {
        return Rc::clone(&top_level_nodes_ptr);
    }

    let mut nodes_by_level: Vec<Vec<Rc<RefCell<ActivityNode>>>> = vec![vec![]];

    for i in next_length_index..activity_nodes.len() {
        let current_node_ptr = activity_nodes.get(i).unwrap();
        let current_node = current_node_ptr.borrow();

        if current_node.len() < current_length {
            current_length = current_node.len();
            nodes_by_level.push(vec![]);
        }

        let mut found_any_match = false;

        'this_loop: for level_index in (0..(nodes_by_level.len() - 1)).rev() {
            for activity_node in nodes_by_level.get(level_index).unwrap() {
                let mut activity_node = activity_node.borrow_mut();
                if activity_node.contains_other(&current_node) {
                    activity_node.children.push(Rc::clone(current_node_ptr));
                    found_any_match = true;
                    break 'this_loop;
                }
            }
        }

        if !found_any_match {
            for top_level_node_ptr in top_level_nodes.iter() {
                let mut top_level_node = top_level_node_ptr.borrow_mut();
                if top_level_node.contains_other(&current_node) && !Rc::ptr_eq(top_level_node_ptr, current_node_ptr) {
                    top_level_node.children.push(Rc::clone(current_node_ptr));
                    found_any_match = true;
                    break;
                }
            }
        }

        nodes_by_level.last_mut().unwrap().push(Rc::clone(current_node_ptr));
        if !found_any_match {
            top_level_nodes.push(Rc::clone(current_node_ptr));
        }
    }

    Rc::clone(&top_level_nodes_ptr)
}

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

            if current_activity
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
