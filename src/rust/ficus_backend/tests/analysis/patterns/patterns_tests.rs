use ficus_backend::{
    event_log::{
        core::{
            event::{event::Event, event_hasher::NameEventHasher},
            event_log::EventLog,
            trace::trace::Trace,
        },
        simple::simple_event_log::SimpleEventLog,
    },
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

#[test]
fn test_tandem_arrays_from_paper_string() {
    let log = create_log_from_taxonomy_of_patterns();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(dump_repeats_to_string(&to_sub_arrays(&tandem_arrays.borrow()), &log), ["abc", "bca", "cab", "abcabc", "bcabca"]);
}

fn to_sub_arrays(arrays: &Vec<Vec<TandemArrayInfo>>) -> Vec<Vec<SubArrayInTraceInfo>> {
    arrays.iter().map(|trace_arrays| {
        trace_arrays.iter().map(|arr| *arr.get_sub_array_info()).collect()
    }).collect()
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
fn test_no_tandem_arrays_string() {
    let log = create_no_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(dump_repeats_to_string(&to_sub_arrays(&tandem_arrays.borrow()), &log), Vec::<String>::new());
}

#[test]
fn test_one_tandem_array() {
    let log = create_one_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(&tandem_arrays.borrow()), [(0, 2, 2)]);
}

#[test]
fn test_one_tandem_array_string() {
    let log = create_one_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(dump_repeats_to_string(&to_sub_arrays(&tandem_arrays.borrow()), &log), ["ab"]);
}

#[test]
fn test_tandem_arrays2() {
    let log = create_log_for_max_repeats2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let tandem_arrays = find_primitive_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(&tandem_arrays.borrow()), [(0, 4, 2)]);
}

#[test]
fn test_tandem_arrays2_string() {
    let log = create_log_for_max_repeats2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let tandem_arrays = find_primitive_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(dump_repeats_to_string(&to_sub_arrays(&tandem_arrays.borrow()), &log), ["dabc"]);
}

#[test]
fn test_maximal_repeats_single_merged_trace1() {
    let log = create_single_trace_test_log1();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(dump_repeats(&repeats.borrow()), [(0, 0, 3)]);
}

#[test]
fn test_maximal_repeats_single_merged_trace1_string() {
    let log = create_single_trace_test_log1();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(dump_repeats_to_string(&repeats.borrow(), &log), ["abc"]);
}

#[test]
fn test_maximal_repeats_single_merged_trace2() {
    let log = create_single_trace_test_log2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(dump_repeats(&repeats.borrow()), [(0, 3, 6)]);
}

#[test]
fn test_maximal_repeats_single_merged_trace2_string() {
    let log = create_single_trace_test_log2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(dump_repeats_to_string(&repeats.borrow(), &log), ["abc"]);
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
            (1, 0, 3),
            (1, 0, 4),
            (1, 7, 9),
            (2, 0, 4),
            (2, 6, 10),
            (2, 7, 10),
            (2, 8, 10),
            (3, 2, 4),
            (4, 3, 6),
            (4, 4, 6),
            (4, 9, 10),
            (4, 17, 19)
        ]
    );
}

#[test]
fn test_maximal_repeats_single_merged_trace3_string() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(
        dump_repeats_to_string(&repeats.borrow(), &log),
        [
            "a", "aa", "ab", "abc", "abcd", "bcdbb", "cd", "d", "db", "bb", "bbc", "bbcd", "b", "bc", "bcda", "c",
            "da", "dab", "dabc", "cb", "bbbc", "bbcc", "bcc", "cc", "ad", "cdc", "dc", "e", "bd"
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
            (0, 1, 5),
            (0, 2, 7),
            (0, 5, 9),
            (0, 6, 10),
            (1, 0, 4),
            (1, 7, 9),
            (2, 0, 4),
            (2, 6, 10),
            (3, 2, 4),
            (4, 3, 6),
            (4, 9, 10),
            (4, 17, 19)
        ]
    );
}

#[test]
fn test_super_maximal_repeats_single_merged_trace_string() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_super_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(
        dump_repeats_to_string(&repeats.borrow(), &log),
        ["abcd", "bcdbb", "bbcd", "bcda", "dabc", "cb", "bbbc", "bbcc", "ad", "cdc", "e", "bd"]
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
            (0, 0, 2),
            (0, 1, 5),
            (0, 2, 7),
            (0, 4, 6),
            (0, 5, 7),
            (0, 5, 9),
            (0, 6, 10),
            (1, 0, 3),
            (1, 0, 4),
            (1, 7, 9),
            (2, 0, 4),
            (2, 6, 10),
            (2, 7, 10),
            (2, 8, 10),
            (3, 2, 4),
            (4, 3, 6),
            (4, 4, 6),
            (4, 9, 10),
            (4, 17, 19)
        ]
    );
}

#[test]
fn test_near_super_maximal_repeats_single_merged_trace_string() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let repeats = find_near_super_maximal_repeats(&hashes, &PatternsDiscoveryStrategy::FromSingleMergedTrace);

    assert_eq!(
        dump_repeats_to_string(&repeats.borrow(), &log),
        [
            "aa", "abcd", "bcdbb", "db", "bb", "bbcd", "bcda", "dab", "dabc", "cb", "bbbc", "bbcc", "bcc", "cc", "ad",
            "cdc", "dc", "e", "bd"
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

fn dump_repeats_to_string(repeats: &Vec<Vec<SubArrayInTraceInfo>>, log: &SimpleEventLog) -> Vec<String> {
    let mut result = vec![];
    let mut index = 0;

    for trace_repeats in repeats {
        for repeat in trace_repeats {
            let trace = log.get_traces().get(index).unwrap().borrow();
            let events = trace.get_events();
            let mut string = String::new();

            for event in &events[repeat.start_index..(repeat.start_index + repeat.length)] {
                string.push_str(event.borrow().get_name());
            }

            result.push(string);
        }

        index += 1;
    }

    result
}
