from IPython.core.display_functions import display

from ficus.analysis.event_log_analysis import DatasetVisualizationMethod, NComponents
from ficus.grpc_pipelines.activities_parts import ClusterizeLogTracesDbscan, DiscoverActivitiesFromPatterns2, \
    DiscoverActivitiesInstances2, ClusterizeActivitiesFromTracesDbscan, DiscoverActivitiesForSeveralLevels2
from ficus.grpc_pipelines.constants import const_labeled_log_traces_dataset, const_cluster_labels, \
    const_labeled_traces_activities_dataset
from ficus.grpc_pipelines.context_values import from_grpc_labeled_dataset
from ficus.grpc_pipelines.data_models import Distance, PatternsKind, PatternsDiscoveryStrategy, NarrowActivityKind, \
    ActivitiesRepresentationSource
from ficus.grpc_pipelines.grpc_pipelines import Pipeline2, PipelinePart2WithCallback, PipelinePart2
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcContextValue
from ficus.grpc_pipelines.util_parts import UseNamesEventLog2
from .test_grpc_pipelines import _execute_test_with_names_log, ResultAssertanceKind


def test_simple_dataset_1():
    execute_test_with_traces_dataset(
        [
            ['A', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
        ],
        ClusterizeLogTracesDbscan(
            after_clusterization_pipeline=Pipeline2(),
            min_events_count_in_cluster=2,
        ),
        [
            [1.0, 0.0, 1.0, 0.0],
            [1.0, 0.5, 1.0, 0.5],
            [1.0, 1.0, 1.0, 1.0]
        ]
    )


def test_simple_dataset_2():
    execute_test_with_traces_dataset(
        [
            ['A', 'C'],
            ['A', 'B', 'C'],
            ['A', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'B', 'C'],
        ],
        ClusterizeLogTracesDbscan(
            after_clusterization_pipeline=Pipeline2(),
            min_events_count_in_cluster=2,
        ),
        [
            [1.0, 0.0, 1.0],
            [1.0, 0.25, 1.0],
            [1.0, 0.5, 1.0],
            [1.0, 0.75, 1.0],
            [1.0, 1.0, 1.0]
        ]
    )


def test_simple_dataset_3():
    execute_test_with_traces_dataset(
        [
            ['A', 'B', 'B', 'D'],
            ['A', 'B', 'C', 'C', 'B', 'D'],
            ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
        ],
        ClusterizeLogTracesDbscan(
            after_clusterization_pipeline=Pipeline2(),
            min_events_count_in_cluster=2,
        ),
        [
            [1.0, 1.0, 0.0, 1.0],
            [1.0, 1.0, 0.5, 1.0],
            [1.0, 1.0, 1.0, 1.0]
        ]
    )


def test_simple_dataset_4():
    execute_test_with_traces_dataset(
        [
            ['A', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'C'],
            ['A', 'B', 'D', 'B', 'D', 'B', 'C'],
        ],
        ClusterizeLogTracesDbscan(
            after_clusterization_pipeline=Pipeline2(),
            min_events_count_in_cluster=2,
            distance=Distance.Levenshtein
        ),
        [
            [1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 2.0, 4.0, 2.0, 3.0, 0.0, 0.0, 0.0],
            [1.0, 2.0, 4.0, 2.0, 4.0, 2.0, 3.0, 0.0]
        ]
    )


def test_simple_dataset_5():
    execute_test_with_traces_dataset(
        [
            ['A', 'C'],
            ['A', 'B', 'C'],
            ['A', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'C'],
            ['A', 'B', 'B', 'B', 'B', 'C'],
        ],
        ClusterizeLogTracesDbscan(
            after_clusterization_pipeline=Pipeline2(),
            min_events_count_in_cluster=2,
            distance=Distance.Levenshtein
        ),
        [
            [1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 3.0, 3.0, 2.0, 0.0, 0.0, 0.0],
            [1.0, 3.0, 3.0, 3.0, 2.0, 0.0, 0.0],
            [1.0, 3.0, 3.0, 3.0, 3.0, 2.0, 0.0]
        ]
    )


def test_simple_dataset_6():
    execute_test_with_traces_dataset(
        [
            ['A', 'B', 'B', 'D'],
            ['A', 'B', 'C', 'C', 'B', 'D'],
            ['A', 'B', 'C', 'C', 'C', 'C', 'B', 'D'],
        ],
        ClusterizeLogTracesDbscan(
            after_clusterization_pipeline=Pipeline2(),
            min_events_count_in_cluster=2,
            distance=Distance.Levenshtein
        ),
        [
            [1.0, 2.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 2.0, 4.0, 4.0, 2.0, 3.0, 0.0, 0.0, 0.0],
            [1.0, 2.0, 4.0, 4.0, 4.0, 4.0, 2.0, 3.0, 0.0]
        ]
    )


def test_levenshtein_in_activities_clustering():
    _execute_test_with_names_log(
        [],
        Pipeline2(
            UseNamesEventLog2(),
            DiscoverActivitiesFromPatterns2(patterns_kind=PatternsKind.MaximalRepeats,
                                            strategy=PatternsDiscoveryStrategy.FromSingleMergedTrace),
            DiscoverActivitiesInstances2(narrow_activities=NarrowActivityKind.NarrowDown),
            ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                                 tolerance=0.1,
                                                 activities_repr_source=ActivitiesRepresentationSource.SubTracesUnderlyingEvents,
                                                 distance=Distance.Levenshtein,
                                                 activity_level=0,
                                                 view_params=(30, 60),
                                                 legend_cols=4,
                                                 visualization_method=DatasetVisualizationMethod.TSNE,
                                                 n_components=NComponents.Three),
        ),
        assertance_kind=ResultAssertanceKind.Error
    )


def test_activities_dataset_1():
    execute_test_with_activities_dataset(
        [
            ['A', 'B', 'C', 'x', 'A', 'B', 'C'],
            ['A', 'D', 'C', 'y', 'A', 'D', 'C'],

            ['X', 'Y', 'Z', 'x', 'X', 'Y', 'Z'],
            ['X', 'Q', 'Z', 'y', 'X', 'Q', 'Z'],
        ],
        ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                             tolerance=0.1,
                                             activities_repr_source=ActivitiesRepresentationSource.EventClasses,
                                             distance=Distance.Cosine,
                                             show_visualization=False),
        [
            [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
        ]
    )


def test_activities_dataset_2():
    execute_test_with_activities_dataset(
        [
            ['A', 'A', 'B', 'C', 'x', 'A', 'A', 'B', 'C'],
            ['A', 'D', 'C', 'C', 'y', 'A', 'D', 'C', 'C'],

            ['X', 'X', 'Y', 'Z', 'x', 'X', 'X', 'Y', 'Z'],
            ['X', 'Q', 'Z', 'Z', 'y', 'X', 'Q', 'Z', 'Z'],
        ],
        ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                             tolerance=0.1,
                                             activities_repr_source=ActivitiesRepresentationSource.EventClasses,
                                             distance=Distance.Cosine,
                                             show_visualization=False),
        [
            [1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]
        ]
    )

def test_activities_dataset_3():
    execute_test_with_activities_dataset(
        [
            ['A', 'A', 'B', 'C', 'x', 'A', 'A', 'B', 'C'],
            ['A', 'D', 'C', 'C', 'y', 'A', 'D', 'C', 'C'],

            ['X', 'X', 'Y', 'Z', 'x', 'X', 'X', 'Y', 'Z'],
            ['X', 'Q', 'Z', 'Z', 'y', 'X', 'Q', 'Z', 'Z'],
        ],
        ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                             tolerance=0.1,
                                             activities_repr_source=ActivitiesRepresentationSource.SubTraces,
                                             distance=Distance.Cosine,
                                             show_visualization=False),
        [
            [1.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5]
        ]
    )


def test_activities_dataset_4():
    execute_test_with_activities_dataset(
        [
            ['A', 'A', 'B', 'C', 'x', 'A', 'A', 'B', 'C'],
            ['A', 'D', 'C', 'C', 'y', 'A', 'D', 'C', 'C'],

            ['X', 'X', 'Y', 'Z', 'x', 'X', 'X', 'Y', 'Z'],
            ['X', 'Q', 'Z', 'Z', 'y', 'X', 'Q', 'Z', 'Z'],
        ],
        ClusterizeActivitiesFromTracesDbscan(min_events_count_in_cluster=2,
                                             tolerance=0.1,
                                             activities_repr_source=ActivitiesRepresentationSource.SubTracesUnderlyingEvents,
                                             distance=Distance.Cosine,
                                             show_visualization=False),
        [
            [1.0, 1.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5]
        ]
    )


class TestDatasetPipelinePart(PipelinePart2WithCallback):
    def __init__(self,
                 original_part: PipelinePart2,
                 expected_dataset: list[list[float]],
                 labeled_dataset_key: str):
        super().__init__()
        self.uuid = original_part.uuid
        self.original_part = original_part
        self.expected_dataset = expected_dataset
        self.labeled_dataset_key = labeled_dataset_key

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return self.original_part.to_grpc_part()

    def execute_callback(self, values: dict[str, GrpcContextValue], labeled_log_traces_dataset=None):
        dataset = values[self.labeled_dataset_key].labeled_dataset
        df = from_grpc_labeled_dataset(dataset).drop([const_cluster_labels], axis=1)
        print(df)
        assert df.values.tolist() == self.expected_dataset


def execute_test_with_activities_dataset(names_log,
                                         clusterization_pipeline,
                                         expected_raw_dataset,
                                         assertance_kind=ResultAssertanceKind.Success):
    _execute_test_with_names_log(
        names_log,
        Pipeline2(
            UseNamesEventLog2(),
            DiscoverActivitiesForSeveralLevels2(event_classes=['.*'],
                                                patterns_kind=PatternsKind.MaximalRepeats),
            TestDatasetPipelinePart(clusterization_pipeline,
                                    expected_raw_dataset,
                                    const_labeled_traces_activities_dataset)
        ),
        assertance_kind
    )


def execute_test_with_traces_dataset(names_log,
                                     clusterization_pipeline,
                                     expected_raw_dataset,
                                     assertance_kind=ResultAssertanceKind.Success):
    _execute_test_with_names_log(
        names_log,
        Pipeline2(
            UseNamesEventLog2(),
            TestDatasetPipelinePart(clusterization_pipeline, expected_raw_dataset, const_labeled_log_traces_dataset),
        ),
        assertance_kind
    )
