use std::{cell::RefCell, rc::Rc, vec};

use crate::utils::suffix_tree::{suffix_tree::SuffixTree, suffix_tree_slice::SingleWordSuffixTreeSlice};

use super::tandem_arrays::SubArrayInTraceInfo;

pub fn find_maximal_repeats(log: &Vec<Vec<u64>>) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    find_repeats(log, |tree| tree.find_maximal_repeats())
}

fn find_repeats<TRepeatsFinder>(
    log: &Vec<Vec<u64>>,
    finder: TRepeatsFinder,
) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>>
where
    TRepeatsFinder: Fn(&SuffixTree<u64, SingleWordSuffixTreeSlice<u64>>) -> Vec<(usize, usize)>,
{
    let repeats_ptr = Rc::new(RefCell::new(vec![]));
    let repeats = &mut repeats_ptr.borrow_mut();

    for trace in log {
        let slice = Rc::new(Box::new(SingleWordSuffixTreeSlice::new(trace.as_slice())));
        let mut tree = SuffixTree::new(slice);
        tree.build_tree();

        repeats.push(
            finder(&tree)
                .into_iter()
                .map(|repeat| SubArrayInTraceInfo::new(repeat.0, repeat.1 - repeat.0))
                .collect(),
        );
    }

    Rc::clone(&repeats_ptr)
}

pub fn find_super_maximal_repeats(log: &Vec<Vec<u64>>) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    find_repeats(log, |tree| tree.find_super_maximal_repeats())
}

pub fn find_near_super_maximal_repeats(log: &Vec<Vec<u64>>) -> Rc<RefCell<Vec<Vec<SubArrayInTraceInfo>>>> {
    find_repeats(log, |tree| tree.find_near_super_maximal_repeats())
}
