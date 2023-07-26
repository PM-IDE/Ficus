use std::{cell::RefCell, rc::Rc};

use super::{
    repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets, ActivityNode, SubArrayWithTraceIndex},
    repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
    tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
};

pub enum PatternsKind {
    PrimitiveTandemArrays(usize),
    MaximalTandemArrays(usize),

    MaximalRepeats,
    SuperMaximalRepeats,
    NearSuperMaximalRepeats,
}

pub fn find_patterns(log: &Vec<Vec<u64>>, patterns_kind: PatternsKind) -> Vec<Vec<SubArrayInTraceInfo>> {
    match patterns_kind {
        PatternsKind::MaximalRepeats => find_maximal_repeats(log),
        PatternsKind::SuperMaximalRepeats => find_super_maximal_repeats(log),
        PatternsKind::NearSuperMaximalRepeats => find_near_super_maximal_repeats(log),
        PatternsKind::PrimitiveTandemArrays(length) => find_primitive_tandem_arrays(log, length),
        PatternsKind::MaximalTandemArrays(length) => find_maximal_tandem_arrays(log, length),
    }
}

pub fn find_repeats(log: &Vec<Vec<u64>>, patterns_kind: PatternsKind) -> Vec<SubArrayWithTraceIndex> {
    let patterns = find_patterns(log, patterns_kind);
    build_repeat_sets(log, &patterns)
}

pub fn build_repeat_set_tree(log: &Vec<Vec<u64>>, patterns_kind: PatternsKind) -> Vec<Rc<RefCell<ActivityNode>>> {
    let repeats = find_repeats(log, patterns_kind);
    build_repeat_set_tree_from_repeats(log, repeats)
}
