use ficus_backend::{
    event_log::core::{event_hasher::NameEventHasher, event_log::EventLog},
    features::analysis::patterns::tandem_arrays::{find_maximal_tandem_arrays, TandemArrayInfo},
};
use test_core::simple_events_logs_provider::create_log_from_taxonomy_of_patterns;

use crate::test_core::simple_events_logs_provider::{create_no_tandem_array_log, create_one_tandem_array_log};

mod test_core;

#[test]
fn test_tandem_arrays_from_paper() {
    let log = create_log_from_taxonomy_of_patterns();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays(&hashes, 10);
    let tuples = tandem_arrays[0]
        .iter()
        .map(|array| to_tuple(array))
        .collect::<Vec<(usize, usize, usize)>>();

    assert_eq!(tuples, [(2, 3, 4), (3, 3, 4), (4, 3, 3), (2, 6, 2), (3, 6, 2)]);
}

fn to_tuple(array: &TandemArrayInfo) -> (usize, usize, usize) {
    (
        *array.get_sub_array_info().get_start_index(),
        *array.get_sub_array_info().get_length(),
        *array.get_repeat_count(),
    )
}

#[test]
fn test_no_tandem_arrays() {
    let log = create_no_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays(&hashes, 10);
    let tuples = tandem_arrays[0]
        .iter()
        .map(|array| to_tuple(array))
        .collect::<Vec<(usize, usize, usize)>>();

    assert_eq!(tuples, []);
}

#[test]
fn test_one_tandem_array() {
    let log = create_one_tandem_array_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let tandem_arrays = find_maximal_tandem_arrays(&hashes, 10);
    let tuples = tandem_arrays[0]
        .iter()
        .map(|array| to_tuple(array))
        .collect::<Vec<(usize, usize, usize)>>();

    assert_eq!(tuples, [(0, 2, 2)]);
}
