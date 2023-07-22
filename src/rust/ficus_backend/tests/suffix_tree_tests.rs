use std::fs;

use ficus_backend::utils::suffix_tree::SuffixTree;
use test_core::{test_paths::{get_paths_to_suffix_tree_string, create_suffix_tree_gold_file_path}, gold_based_test::execute_test_with_gold, simple_events_logs_provider::create_log_from_filter_out_chaotic_events_with_noise};
mod test_core;

#[test]
pub fn test_suffix_tree_nodes() {
    let mut tree = SuffixTree::new("xabxac".as_bytes());
    tree.build_tree();

    assert_eq!(
        tree.dump_nodes(),
        [
            (0, 0, None, None),
            (2, 6, Some(4), None),
            (2, 6, Some(6), None),
            (2, 6, Some(0), None),
            (0, 2, Some(0), Some(6)),
            (5, 6, Some(4), None),
            (1, 2, Some(0), Some(0)),
            (5, 6, Some(6), None),
            (5, 6, Some(0), None)
        ]
    );
}

#[test]
pub fn test_suffix_tree_nodes2() {
    let mut tree = SuffixTree::new("dasdasdasasasdasdasasd".as_bytes());
    tree.build_tree();

    assert_eq!(
        tree.dump_nodes(),
        [
            (0, 0, None, None),
            (6, 22, Some(4), None),
            (11, 22, Some(20), None),
            (11, 22, Some(22), None),
            (3, 6, Some(10), Some(6)),
            (11, 22, Some(24), None),
            (3, 6, Some(12), Some(8)),
            (11, 22, Some(26), None),
            (3, 6, Some(14), Some(10)),
            (11, 22, Some(28), None),
            (0, 3, Some(0), Some(12)),
            (11, 22, Some(30), None),
            (1, 3, Some(0), Some(14)),
            (11, 22, Some(16), None),
            (2, 3, Some(0), Some(0)),
            (11, 22, Some(18), None),
            (9, 11, Some(12), Some(18)),
            (13, 22, Some(16), None),
            (9, 11, Some(14), Some(12)),
            (13, 22, Some(18), None),
            (6, 11, Some(6), Some(22)),
            (21, 22, Some(20), None),
            (6, 11, Some(8), Some(24)),
            (21, 22, Some(22), None),
            (9, 11, Some(4), Some(26)),
            (21, 22, Some(24), None),
            (9, 11, Some(6), Some(28)),
            (21, 22, Some(26), None),
            (9, 11, Some(8), Some(30)),
            (21, 22, Some(28), None),
            (9, 11, Some(10), Some(16)),
            (21, 22, Some(30), None)
        ]
    );
}

#[test]
pub fn test_suffix_tree_nodes3() {
    let mut tree = SuffixTree::new("asjkldhoiufjaksdjkasfgahabvasdrfaoasdfuabjikdu".as_bytes());
    tree.build_tree();

    assert_eq!(
        tree.dump_nodes(),
        [
            (0, 0, None, None),
            (2, 46, Some(25), None),
            (2, 46, Some(18), None),
            (4, 46, Some(22), None),
            (4, 46, Some(16), None),
            (4, 46, Some(0), None),
            (6, 46, Some(20), None),
            (7, 46, Some(32), None),
            (8, 46, Some(44), None),
            (9, 46, Some(58), None),
            (10, 46, Some(51), None),
            (11, 46, Some(28), None),
            (2, 3, Some(0), Some(0)),
            (12, 46, Some(12), None),
            (0, 1, Some(0), Some(0)),
            (13, 46, Some(14), None),
            (3, 4, Some(0), Some(0)),
            (14, 46, Some(16), None),
            (1, 2, Some(0), Some(0)),
            (16, 46, Some(38), None),
            (5, 6, Some(0), Some(0)),
            (16, 46, Some(20), None),
            (3, 4, Some(12), Some(16)),
            (18, 46, Some(22), None),
            (18, 46, Some(16), None),
            (1, 2, Some(14), Some(18)),
            (20, 46, Some(25), None),
            (20, 46, Some(18), None),
            (10, 11, Some(0), Some(0)),
            (21, 46, Some(28), None),
            (21, 46, Some(0), None),
            (23, 46, Some(14), None),
            (6, 7, Some(0), Some(0)),
            (24, 46, Some(32), None),
            (26, 46, Some(53), None),
            (26, 46, Some(55), None),
            (26, 46, Some(0), None),
            (30, 46, Some(46), None),
            (15, 16, Some(18), Some(20)),
            (30, 46, Some(38), None),
            (30, 46, Some(20), None),
            (30, 46, Some(0), None),
            (32, 46, Some(28), None),
            (33, 46, Some(14), None),
            (7, 8, Some(0), Some(0)),
            (34, 46, Some(44), None),
            (29, 30, Some(25), Some(38)),
            (37, 46, Some(46), None),
            (37, 46, Some(38), None),
            (37, 46, Some(20), None),
            (38, 46, Some(28), None),
            (9, 10, Some(0), Some(0)),
            (39, 46, Some(51), None),
            (25, 26, Some(14), Some(55)),
            (41, 46, Some(53), None),
            (25, 26, Some(0), Some(0)),
            (41, 46, Some(55), None),
            (42, 46, Some(12), None),
            (8, 9, Some(0), Some(0)),
            (43, 46, Some(58), None),
            (44, 46, Some(16), None),
            (45, 46, Some(20), None)
        ]
    );
}

#[test]
fn test_suffix_tree_against_ref_impl() {
    for file_path in get_paths_to_suffix_tree_string() {
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();
        execute_test_with_gold(create_suffix_tree_gold_file_path(file_name), || {
            let file_string = fs::read_to_string(file_path).ok().unwrap();
            let mut tree = SuffixTree::new(file_string.as_bytes());
            tree.build_tree();

            let mut test_value = String::new();
            for node in tree.dump_nodes() {
                let parent = match node.2 {
                    Some(value) => value as i64,
                    None => -1
                };

                let link = match node.3 {
                    Some(value) => value as i64,
                    None => -1
                };

                let serialized_node = format!("({} {} {} {})\n", node.0, node.1, parent, link);
                test_value.push_str(serialized_node.as_str());
            }

            test_value
        });
    }
}