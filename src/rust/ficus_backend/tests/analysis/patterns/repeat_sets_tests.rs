use ficus_backend::{
    event_log::core::{event_hasher::NameEventHasher, event_log::EventLog},
    features::analysis::patterns::{
        patterns::{find_repeats, PatternsKind},
        repeat_sets::SubArrayWithTraceIndex,
    },
};

use crate::test_core::simple_events_logs_provider::create_maximal_repeats_log;

#[test]
fn test_repeat_sets() {
    let log = create_maximal_repeats_log();
    let hashes = log.to_hashes_event_log::<NameEventHasher>();
    let repeats = find_repeats(&hashes, PatternsKind::PrimitiveTandemArrays(20));
    assert_eq!(get_first_trace_repeat(&repeats), [(0, 4, 1), (3, 2, 4)]);
}

fn get_first_trace_repeat(repeats: &Vec<SubArrayWithTraceIndex>) -> Vec<(usize, usize, usize)> {
    repeats.into_iter().map(|array| array.dump()).collect()
}
