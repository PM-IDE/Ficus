use ficus_backend::utils::interval_tree::interval_tree::{Interval, IntervalTree};

#[test]
fn interval_tree_test() {
    let intervals = vec![
        Interval::new(1, 4),
        Interval::new(5, 6),
        Interval::new(9, 10),
        Interval::new(2, 3),
        Interval::new(-1, 3),
        Interval::new(-5, 10),
    ];

    let tree = IntervalTree::new(intervals);

    assert_eq!(
        tree.search_point(5),
        [Interval { left: -5, right: 10 }, Interval { left: 5, right: 6 }]
    );

    assert_eq!(
        tree.search_point(2),
        [
            Interval { left: -5, right: 10 },
            Interval { left: -1, right: 3 },
            Interval { left: 1, right: 4 },
            Interval { left: 2, right: 3 }
        ]
    );

    assert_eq!(
        tree.search_interval(1..3),
        [
            Interval { left: -5, right: 10 },
            Interval { left: -1, right: 3 },
            Interval { left: 1, right: 4 },
            Interval { left: 2, right: 3 }
        ]
    );

    assert_eq!(
        tree.search_interval(1..10),
        [
            Interval { left: -5, right: 10 },
            Interval { left: -1, right: 3 },
            Interval { left: 1, right: 4 },
            Interval { left: 2, right: 3 },
            Interval { left: 5, right: 6 },
            Interval { left: 9, right: 10 }
        ]
    );
}
