use ficus_backend::{
    event_log::core::{event::event_hasher::NameEventHasher, event_log::EventLog},
    features::analysis::patterns::{
        contexts::PatternsDiscoveryStrategy,
        repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
        tandem_arrays::{
            find_maximal_tandem_arrays_with_length, find_primitive_tandem_arrays_with_length, SubArrayInTraceInfo,
            TandemArrayInfo,
        },
    },
};

use crate::test_core::simple_events_logs_provider::{
    create_log_for_max_repeats2, create_log_from_taxonomy_of_patterns, create_maximal_repeats_log,
    create_no_tandem_array_log, create_one_tandem_array_log, create_single_trace_test_log1,
    create_single_trace_test_log2,
};

#[test]
fn test_tandem_arrays_from_paper() {
    let log = create_log_from_taxonomy_of_patterns();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(
        get_first_trace_tuples(&tandem_arrays.borrow()),
        [(2, 3, 4), (3, 3, 4), (4, 3, 3), (2, 6, 2), (3, 6, 2)]
    );
}

fn get_first_trace_tuples(tandem_arrays: &Vec<Vec<TandemArrayInfo>>) -> Vec<(usize, usize, usize)> {
    tandem_arrays[0].iter().map(|array| array.dump()).collect()
}

#[test]
fn test_no_tandem_arrays() {
    let log = create_no_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(&tandem_arrays.borrow()), []);
}

#[test]
fn test_one_tandem_array() {
    let log = create_one_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(&tandem_arrays.borrow()), [(0, 2, 2)]);
}

#[test]
fn test_tandem_arrays2() {
    let log = create_log_for_max_repeats2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let tandem_arrays = find_primitive_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(&tandem_arrays.borrow()), [(0, 4, 2)]);
}

#[test]
fn test_maximal_repeats_single_merged_trace1() {
    let log = create_single_trace_test_log1();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(dump_repeats(&repeats.borrow()), [(0, 0, 3)]);
}

#[test]
fn test_maximal_repeats_single_merged_trace2() {
    let log = create_single_trace_test_log2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(dump_repeats(&repeats.borrow()), [(0, 3, 6)]);
}

#[test]
fn test_maximal_repeats_single_merged_trace3() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(
        dump_repeats(&repeats.borrow()),
        [
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 3),
            (0, 1, 4),
            (0, 1, 5),
            (0, 2, 7),
            (0, 3, 5),
            (0, 4, 5),
            (0, 4, 6),
            (0, 5, 7),
            (0, 5, 8),
            (0, 5, 9),
            (0, 6, 7),
            (0, 6, 8),
            (0, 6, 10),
            (0, 7, 8),
            (0, 8, 10),
            (0, 0, 3),
            (0, 0, 4),
            (0, 7, 9),
            (0, 0, 4),
            (0, 6, 10),
            (0, 7, 10),
            (0, 8, 10),
            (0, 2, 4),
            (0, 3, 6),
            (0, 4, 6),
            (0, 9, 10),
            (1, 17, 19)
        ]
    );
}

#[test]
fn test_super_maximal_repeats_single_merged_trace() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_super_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(
        dump_repeats(&repeats.borrow()),
        [
            (0, 0, 2),
            (0, 1, 5),
            (0, 2, 7),
            (0, 5, 9),
            (0, 6, 10),
            (0, 0, 4),
            (0, 7, 9),
            (0, 0, 4),
            (0, 6, 10),
            (0, 2, 4),
            (0, 3, 6),
            (0, 9, 10),
            (1, 17, 19)
        ]
    );
}

#[test]
fn test_near_super_maximal_repeats_single_merged_trace() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_near_super_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(
        dump_repeats(&repeats.borrow()),
        [
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 3),
            (0, 1, 4),
            (0, 1, 5),
            (0, 2, 7),
            (0, 3, 5),
            (0, 4, 5),
            (0, 4, 6),
            (0, 5, 7),
            (0, 5, 8),
            (0, 5, 9),
            (0, 6, 7),
            (0, 6, 8),
            (0, 6, 10),
            (0, 7, 8),
            (0, 8, 10),
            (0, 0, 3),
            (0, 0, 4),
            (0, 7, 9),
            (0, 0, 4),
            (0, 6, 10),
            (0, 7, 10),
            (0, 8, 10),
            (0, 2, 4),
            (0, 3, 6),
            (0, 4, 6),
            (0, 9, 10),
            (1, 17, 19)
        ]
    );
}

fn dump_repeats(repeats: &Vec<Vec<SubArrayInTraceInfo>>) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    let mut index = 0;

    for trace_repeats in repeats {
        for repeat in trace_repeats {
            result.push((index, repeat.start_index, repeat.start_index + repeat.length));
        }

        index += 1;
    }

    result
}
