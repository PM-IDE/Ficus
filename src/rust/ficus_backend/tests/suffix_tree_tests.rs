use std::{fs, rc::Rc};

use ficus_backend::utils::suffix_tree::{
    suffix_tree::SuffixTree,
    suffix_tree_slice::{MultipleWordsSuffixTreeSlice, SingleWordSuffixTreeSlice},
};
use test_core::{
    gold_based_test::execute_test_with_gold,
    test_paths::{create_suffix_tree_gold_file_path, get_paths_to_suffix_tree_string},
};

use crate::test_core::simple_events_logs_provider::{
    create_log_for_max_repeats1, create_max_repeats_trace_1, create_max_repeats_trace_2, create_max_repeats_trace_3,
    create_max_repeats_trace_4, create_max_repeats_trace_5,
};
mod test_core;

//ref impl: http://e-maxx.ru/algo/ukkonen
#[test]
fn test_suffix_tree_against_ref_impl() {
    for file_path in get_paths_to_suffix_tree_string() {
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();
        execute_test_with_gold(create_suffix_tree_gold_file_path(file_name), || {
            let mut file_string = fs::read_to_string(file_path).ok().unwrap();

            //remove last symbol as it is non-existing symbol for ref impl, but our impl
            //adds at implicitly
            file_string.remove(file_string.len() - 1);
            let slice = SingleWordSuffixTreeSlice::new(file_string.as_bytes());
            let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
            tree.build_tree();

            let mut test_value = String::new();
            for node in tree.dump_nodes() {
                let parent = match node.2 {
                    Some(value) => value as i64,
                    None => -1,
                };

                let link = match node.3 {
                    Some(value) => value as i64,
                    None => -1,
                };

                let serialized_node = format!("({} {} {} {})\n", node.0, node.1, parent, link);
                test_value.push_str(serialized_node.as_str());
            }

            test_value
        });
    }
}

#[test]
fn test_maximal_repeats() {
    let slice = SingleWordSuffixTreeSlice::new("djksadlasdjaslkdj".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.find_maximal_repeats(),
        [(0, 1), (0, 2), (2, 3), (3, 4), (4, 5), (6, 7), (7, 9)],
    )
}

#[test]
fn test_maximal_repeats2() {
    let slice = SingleWordSuffixTreeSlice::new("abcdxabcyabcz".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_maximal_repeats(), [(0, 3)])
}

#[test]
fn test_maximal_repeats3() {
    let slice = SingleWordSuffixTreeSlice::new("aaacdcdcbedbccbadbdebdc".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.find_maximal_repeats(),
        [
            (0, 1),
            (0, 2),
            (3, 4),
            (3, 6),
            (4, 5),
            (4, 6),
            (7, 9),
            (8, 9),
            (9, 10),
            (10, 12),
            (17, 19)
        ]
    )
}

#[test]
fn test_maximal_repeats4() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_1());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_maximal_repeats(), [(0, 1), (2, 3), (2, 5)])
}

#[test]
fn test_maximal_repeats5() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_2());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_maximal_repeats(), [(0, 4), (2, 3)])
}

#[test]
fn test_super_maximal_repeats() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_1());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_super_maximal_repeats(), [(0, 1), (2, 5)])
}

#[test]
fn test_near_super_maximal_repeats() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_1());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_near_super_maximal_repeats(), [(0, 1), (2, 3), (4, 7)])
}

#[test]
fn test_near_super_maximal_repeats2() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_2());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_near_super_maximal_repeats(), [(0, 4), (5, 6)])
}

#[test]
fn test_near_super_maximal_repeats3() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_3());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.find_near_super_maximal_repeats(),
        [(0, 1), (2, 4), (5, 9), (10, 11), (12, 13)]
    )
}

#[test]
fn test_near_super_maximal_repeats4() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_4());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.find_near_super_maximal_repeats(),
        [(0, 1), (2, 4), (5, 6), (7, 8), (9, 11)]
    )
}

#[test]
fn test_near_super_maximal_repeats6() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_5());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.find_near_super_maximal_repeats(),
        [
            (0, 1),
            (2, 4),
            (5, 6),
            (7, 10),
            (11, 12),
            (13, 15),
            (16, 18),
            (19, 20),
            (21, 22),
            (23, 25),
            (26, 28)
        ]
    )
}

#[test]
fn test_multiple_words_suffix_tree_slice() {
    let slices = vec!["abc".as_bytes(), "fsd".as_bytes()];
    let slice = MultipleWordsSuffixTreeSlice::new(slices);
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_patterns("abc".as_bytes()).unwrap(), [(0, 3)]);
    assert_eq!(tree.find_patterns("fsd".as_bytes()).unwrap(), [(4, 7)]);
    assert_eq!(tree.find_patterns("f".as_bytes()).unwrap(), [(4, 5)]);
}

#[test]
fn test_patterns_search() {
    let slice = SingleWordSuffixTreeSlice::new("abcdxabcyabcz".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_patterns("abc".as_bytes()).unwrap(), [(0, 3), (5, 8), (9, 12)]);
}

#[test]
fn test_patterns_search2() {
    let slice = SingleWordSuffixTreeSlice::new(create_max_repeats_trace_5());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_patterns("badb".as_bytes()).unwrap(), [(14, 18)]);
}

#[test]
fn test_patterns_search3() {
    let slice = SingleWordSuffixTreeSlice::new("abcdxabcyabcz".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_patterns("a".as_bytes()).unwrap(), [(0, 1), (5, 6), (9, 10)]);
}

#[test]
fn test_patterns_search4() {
    let slice = SingleWordSuffixTreeSlice::new("xabxac".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(tree.find_patterns("xa".as_bytes()).unwrap(), [(0, 2), (3, 5)]);
}

#[test]
pub fn test_suffix_tree_nodes() {
    let slice = SingleWordSuffixTreeSlice::new("xabxac".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.dump_nodes(),
        [
            (0, 0, None, None),
            (2, 7, Some(4), None),
            (2, 7, Some(6), None),
            (2, 7, Some(0), None),
            (0, 2, Some(0), Some(6)),
            (5, 7, Some(4), None),
            (1, 2, Some(0), Some(0)),
            (5, 7, Some(6), None),
            (5, 7, Some(0), None),
            (6, 7, Some(0), None)
        ]
    );
}

#[test]
pub fn test_suffix_tree_nodes2() {
    let slice = SingleWordSuffixTreeSlice::new("dasdasdasasasdasdasasd".as_bytes());
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.dump_nodes(),
        [
            (0, 0, None, None),
            (6, 23, Some(4), None),
            (11, 23, Some(20), None),
            (11, 23, Some(22), None),
            (3, 6, Some(10), Some(6)),
            (11, 23, Some(24), None),
            (4, 6, Some(36), Some(8)),
            (11, 23, Some(26), None),
            (4, 6, Some(38), Some(10)),
            (11, 23, Some(28), None),
            (1, 3, Some(40), Some(12)),
            (11, 23, Some(30), None),
            (1, 3, Some(0), Some(14)),
            (11, 23, Some(16), None),
            (2, 3, Some(0), Some(0)),
            (11, 23, Some(18), None),
            (9, 11, Some(12), Some(18)),
            (14, 23, Some(32), None),
            (9, 11, Some(14), Some(12)),
            (14, 23, Some(34), None),
            (6, 11, Some(6), Some(22)),
            (21, 23, Some(20), None),
            (6, 11, Some(8), Some(24)),
            (21, 23, Some(22), None),
            (9, 11, Some(4), Some(26)),
            (21, 23, Some(24), None),
            (9, 11, Some(6), Some(28)),
            (21, 23, Some(26), None),
            (9, 11, Some(8), Some(30)),
            (21, 23, Some(28), None),
            (9, 11, Some(10), Some(16)),
            (21, 23, Some(30), None),
            (13, 14, Some(16), Some(34)),
            (22, 23, Some(32), None),
            (13, 14, Some(18), Some(36)),
            (22, 23, Some(34), None),
            (3, 4, Some(12), Some(38)),
            (22, 23, Some(36), None),
            (3, 4, Some(14), Some(40)),
            (22, 23, Some(38), None),
            (0, 1, Some(0), Some(0)),
            (22, 23, Some(40), None),
            (22, 23, Some(0), None)
        ]
    );
}

#[test]
pub fn test_suffix_tree_nodes3() {
    let string = "asjkldhoiufjaksdjkasfgahabvasdrfaoasdfuabjikdu".as_bytes();
    let slice = SingleWordSuffixTreeSlice::new(string);
    let mut tree = SuffixTree::new(Rc::new(Box::new(slice)));
    tree.build_tree();

    assert_eq!(
        tree.dump_nodes(),
        [
            (0, 0, None, None),
            (2, 47, Some(25), None),
            (2, 47, Some(18), None),
            (4, 47, Some(22), None),
            (4, 47, Some(16), None),
            (4, 47, Some(0), None),
            (6, 47, Some(20), None),
            (7, 47, Some(32), None),
            (8, 47, Some(44), None),
            (9, 47, Some(58), None),
            (10, 47, Some(51), None),
            (11, 47, Some(28), None),
            (2, 3, Some(0), Some(0)),
            (12, 47, Some(12), None),
            (0, 1, Some(0), Some(0)),
            (13, 47, Some(14), None),
            (3, 4, Some(0), Some(0)),
            (14, 47, Some(16), None),
            (1, 2, Some(0), Some(0)),
            (16, 47, Some(38), None),
            (5, 6, Some(0), Some(0)),
            (16, 47, Some(20), None),
            (3, 4, Some(12), Some(16)),
            (18, 47, Some(22), None),
            (18, 47, Some(16), None),
            (1, 2, Some(14), Some(18)),
            (20, 47, Some(25), None),
            (20, 47, Some(18), None),
            (10, 11, Some(0), Some(0)),
            (21, 47, Some(28), None),
            (21, 47, Some(0), None),
            (23, 47, Some(14), None),
            (6, 7, Some(0), Some(0)),
            (24, 47, Some(32), None),
            (26, 47, Some(53), None),
            (26, 47, Some(55), None),
            (26, 47, Some(0), None),
            (30, 47, Some(46), None),
            (15, 16, Some(18), Some(20)),
            (30, 47, Some(38), None),
            (30, 47, Some(20), None),
            (30, 47, Some(0), None),
            (32, 47, Some(28), None),
            (33, 47, Some(14), None),
            (7, 8, Some(0), Some(0)),
            (34, 47, Some(44), None),
            (29, 30, Some(25), Some(38)),
            (37, 47, Some(46), None),
            (37, 47, Some(38), None),
            (37, 47, Some(20), None),
            (38, 47, Some(28), None),
            (9, 10, Some(0), Some(0)),
            (39, 47, Some(51), None),
            (25, 26, Some(14), Some(55)),
            (41, 47, Some(53), None),
            (25, 26, Some(0), Some(0)),
            (41, 47, Some(55), None),
            (42, 47, Some(12), None),
            (8, 9, Some(0), Some(0)),
            (43, 47, Some(58), None),
            (44, 47, Some(16), None),
            (45, 47, Some(20), None),
            (46, 47, Some(51), None),
            (46, 47, Some(0), None)
        ]
    );
}
