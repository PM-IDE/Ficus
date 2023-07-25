use std::collections::HashMap;

use crate::utils::hash_utils::calculate_poly_hash_for_collection;

use super::tandem_arrays::SubArrayInTraceInfo;

#[derive(Clone, Copy)]
pub struct SubArrayWithTraceIndex {
    pub sub_array: SubArrayInTraceInfo,
    pub trace_index: usize,
}

impl SubArrayWithTraceIndex {
    pub fn new(sub_array: SubArrayInTraceInfo, trace_index: usize) -> Self {
        Self { sub_array, trace_index }
    }
}

pub fn build_repeat_sets(log: &Vec<Vec<u64>>, patterns: &Vec<Vec<SubArrayInTraceInfo>>) -> Vec<SubArrayWithTraceIndex> {
    let mut repeat_sets = HashMap::new();
    let mut trace_index = 0;

    for (trace, trace_patterns) in log.into_iter().zip(patterns) {
        for pattern in trace_patterns {
            let start = pattern.start_index;
            let end = start + pattern.length;
            let mut sub_trace = trace[start..end].to_vec();
            sub_trace.sort();

            let hash = calculate_poly_hash_for_collection(sub_trace.as_slice());

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

    result
}
