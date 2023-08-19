use ficus_backend::pipelines::pipelines::PipelineParts;

#[test]
fn test_pipeline_parts() {
    let parts = PipelineParts::new();

    let names = [
        "ReadLogFromXes",
        "WriteLogToXes",
        "FindPrimitiveTandemArrays",
        "FindMaximalTandemArrays",
        "FindMaximalRepeats",
        "FindSuperMaximalRepeats",
        "FindNearSuperMaximalRepeats",
        "DiscoverActivities",
        "DiscoverActivitiesInstances",
    ];

    for name in names {
        assert!(parts.find_part(name).is_some())
    }
}
