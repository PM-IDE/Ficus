use std::{cell::RefCell, rc::Rc, vec};

use crate::utils::suffix_tree::{
    suffix_tree::SuffixTree,
    suffix_tree_slice::{MultipleWordsSuffixTreeSlice, SingleWordSuffixTreeSlice},
};

use super::{contexts::PatternsDiscoveryStrategy, tandem_arrays::SubArrayInTraceInfo};

pub fn find_maximal_repeats(
    log: &Vec<Vec<u64>>,
    strategy: &PatternsDiscoveryStrategy,
) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    find_repeats(log, strategy, |tree| tree.find_maximal_repeats())
}

fn find_repeats<TRepeatsFinder>(
    log: &Vec<Vec<u64>>,
    strategy: &PatternsDiscoveryStrategy,
    finder: TRepeatsFinder,
) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>>
where
    TRepeatsFinder: Fn(&SuffixTree<u64>) -> Vec<(usize, usize)>,
{
    let repeats_ptr = Rc::new(RefCell::new(vec![]));
    let repeats = &mut repeats_ptr.borrow_mut();

    let mut push_repeats = |patterns: &[(usize, usize)]| {
        repeats.push(
            patterns
                .into_iter()
                .map(|repeat| SubArrayInTraceInfo::new(repeat.0, repeat.1 - repeat.0))
                .collect(),
        );
    };

    match strategy {
        PatternsDiscoveryStrategy::FromAllTraces => {
            for trace in log {
                let slice = SingleWordSuffixTreeSlice::new(trace.as_slice());
                let mut tree = SuffixTree::new(&slice);
                tree.build_tree();
                push_repeats(finder(&tree).as_slice());
            }
        }
        PatternsDiscoveryStrategy::FromSingleMergedTrace => {
            let mut single_trace = vec![];
            for trace in log {
                single_trace.push(trace.as_slice());
            }

            let slice = MultipleWordsSuffixTreeSlice::new(single_trace.clone());
            let mut tree = SuffixTree::new(&slice);

            tree.build_tree();

            let mut patterns = finder(&tree);
            adjust_patterns_from_single_merged_trace(&mut patterns, &slice);

            let mut upper_bound = single_trace[0].len() + 1;
            let mut trace_index = 1;
            let mut pattern_index = 0;
            let mut prev_pattern_index = 0;

            while pattern_index <= patterns.len() {
                if pattern_index >= patterns.len() {
                    push_repeats(&patterns[prev_pattern_index..pattern_index]);
                    break;
                }

                if pattern_index < patterns.len() {
                    let pattern = &patterns[pattern_index];
                    if pattern.1 < upper_bound {
                        pattern_index += 1;
                        continue;
                    }
                }

                push_repeats(&patterns[prev_pattern_index..pattern_index]);

                if trace_index >= single_trace.len() {
                    break;
                }

                upper_bound += single_trace[trace_index].len() + 1;
                trace_index += 1;
                prev_pattern_index = pattern_index;
            }
        }
    }

    Rc::clone(&repeats_ptr)
}

fn adjust_patterns_from_single_merged_trace(
    patterns: &mut Vec<(usize, usize)>, 
    slice: &MultipleWordsSuffixTreeSlice<u64>
) {
    for pattern in patterns {
        let first_index_info = slice.get_slice_info_for(pattern.0).unwrap();
        let trace_index = first_index_info.0;

        let first_index = first_index_info.1.unwrap();
        let second_index = match slice.get_slice_info_for(pattern.1).unwrap().1 {
            Some(index) => index,
            None => slice.get_slice_part_len(trace_index),
        };

        pattern.0 = first_index;
        pattern.1 = second_index;
    }
}

pub fn find_super_maximal_repeats(
    log: &Vec<Vec<u64>>,
    strategy: &PatternsDiscoveryStrategy,
) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    find_repeats(log, strategy, |tree| tree.find_super_maximal_repeats())
}

pub fn find_near_super_maximal_repeats(
    log: &Vec<Vec<u64>>,
    strategy: &PatternsDiscoveryStrategy,
) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    find_repeats(log, strategy, |tree| tree.find_near_super_maximal_repeats())
}
