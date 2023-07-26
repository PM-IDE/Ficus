use ficus_backend::{
    event_log::core::{event_hasher::NameEventHasher, event_log::EventLog},
    features::analysis::patterns::tandem_arrays::{
        find_maximal_tandem_arrays_with_length, find_primitive_tandem_arrays_with_length, TandemArrayInfo,
    },
};
use test_core::simple_events_logs_provider::create_log_from_taxonomy_of_patterns;

use crate::test_core::simple_events_logs_provider::{
    create_log_for_max_repeats2, create_no_tandem_array_log, create_one_tandem_array_log,
};

mod test_core;

#[test]
fn test_tandem_arrays_from_paper() {
    let log = create_log_from_taxonomy_of_patterns();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(
        get_first_trace_tuples(tandem_arrays),
        [(2, 3, 4), (3, 3, 4), (4, 3, 3), (2, 6, 2), (3, 6, 2)]
    );
}

fn get_first_trace_tuples(tandem_arrays: Vec<Vec<TandemArrayInfo>>) -> Vec<(usize, usize, usize)> {
    tandem_arrays[0].iter().map(|array| array.dump()).collect()
}

#[test]
fn test_no_tandem_arrays() {
    let log = create_no_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(tandem_arrays), []);
}

#[test]
fn test_one_tandem_array() {
    let log = create_one_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(tandem_arrays), [(0, 2, 2)]);
}

#[test]
fn test_tandem_arrays2() {
    let log = create_log_for_max_repeats2();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();

    let tandem_arrays = find_primitive_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(tandem_arrays), [(0, 4, 2)]);
}
