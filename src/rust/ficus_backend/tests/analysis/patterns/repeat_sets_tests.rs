use ficus_backend::{
    event_log::core::{event_hasher::NameEventHasher, event_log::EventLog},
    features::analysis::patterns::{
        entry_points::{find_repeats, PatternsKind},
        repeat_sets::SubArrayWithTraceIndex,
    },
};

use crate::test_core::simple_events_logs_provider::create_maximal_repeats_log;

#[test]
fn test_repeat_sets_primitive_tandem_arrays() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let repeats = find_repeats(&hashes, PatternsKind::PrimitiveTandemArrays(20));
    assert_eq!(get_first_trace_repeat(&repeats), [(0, 4, 1), (3, 2, 4)]);
}

fn get_first_trace_repeat(repeats: &Vec<SubArrayWithTraceIndex>) -> Vec<(usize, usize, usize)> {
    repeats.into_iter().map(|array| array.dump()).collect()
}

#[test]
fn test_repeat_sets_super_maximal_repeats() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let repeats = find_repeats(&hashes, PatternsKind::SuperMaximalRepeats);

    assert_eq!(
        get_first_trace_repeat(&repeats),
        [
            (0, 1, 0),
            (2, 3, 0),
            (0, 4, 1),
            (0, 4, 2),
            (5, 1, 3),
            (7, 2, 3),
            (3, 3, 4),
            (9, 1, 4),
            (10, 2, 4)
        ]
    );
}

#[test]
fn test_repeat_sets_near_super_maximal_repeats() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let repeats = find_repeats(&hashes, PatternsKind::NearSuperMaximalRepeats);

    assert_eq!(
        get_first_trace_repeat(&repeats),
        [
            (0, 1, 0),
            (2, 1, 0),
            (2, 3, 0),
            (0, 4, 1),
            (0, 4, 2),
            (3, 1, 2),
            (3, 3, 4),
            (4, 1, 4),
            (9, 1, 4),
            (10, 2, 4)
        ]
    );
}
