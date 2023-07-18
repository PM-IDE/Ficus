use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    rc::Rc,
};

use crate::event_log::core::{event::Event, event_log::EventLog, trace::Trace};

pub fn split_by_traces<TLog, TTrace>(log: &TLog) -> Vec<Vec<Rc<RefCell<TTrace>>>>
where
    TLog: EventLog<TTrace = TTrace>,
    TTrace: Trace,
{
    let mut len_to_traces: HashMap<usize, Vec<Rc<RefCell<TTrace>>>> = HashMap::new();
    for trace in log.get_traces() {
        let len = trace.borrow().get_events().len();
        if len_to_traces.contains_key(&len) {
            (*len_to_traces.get_mut(&len).unwrap()).push(Rc::clone(trace));
        } else {
            len_to_traces.insert(len, vec![Rc::clone(trace)]);
        }
    }

    let mut result = Vec::new();
    for (_, traces) in len_to_traces {
        if traces.len() == 1 {
            result.push(traces);
            continue;
        }

        let mut groups = Vec::new();
        for trace in &traces {
            groups.push(Rc::clone(&trace));
        }

        let mut groups = vec![groups];

        let mut index = 0;
        loop {
            if index >= traces[0].borrow().get_events().len() {
                break;
            }

            let mut new_groups = Vec::new();
            let mut all_groups_have_one_trace = true;
            for group in &groups {
                if group.len() == 1 {
                    new_groups.push(group.to_vec());
                    continue;
                }

                all_groups_have_one_trace = false;
                let mut hashes_to_traces: HashMap<u64, Vec<Rc<RefCell<TTrace>>>> = HashMap::new();
                for trace in group {
                    let mut hasher = DefaultHasher::new();
                    trace.borrow().get_events()[index].borrow().get_name().hash(&mut hasher);
                    let hash_code = hasher.finish();
                    if hashes_to_traces.contains_key(&hash_code) {
                        (*hashes_to_traces.get_mut(&hash_code).unwrap()).push(Rc::clone(&trace));
                    } else {
                        hashes_to_traces.insert(hash_code, vec![Rc::clone(&trace)]);
                    }
                }

                for (_, new_group) in hashes_to_traces {
                    new_groups.push(new_group);
                }
            }

            if all_groups_have_one_trace {
                break;
            }

            index += 1;
            groups = new_groups;
        }

        for group in groups {
            result.push(group);
        }
    }

    result
}
