from ficus.grpc_pipelines.activities_parts import ClusterizeLogTracesDbscan
from ficus.grpc_pipelines.constants import const_labeled_log_traces_dataset, const_cluster_labels
from ficus.grpc_pipelines.context_values import from_grpc_labeled_dataset
from ficus.grpc_pipelines.grpc_pipelines import Pipeline2, PipelinePart2WithCallback, PipelinePart2
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcContextValue
from ficus.grpc_pipelines.util_parts import UseNamesEventLog2
from tests.grpc_pipelines.test_grpc_pipelines import _execute_test_with_names_log


def test_simple_dataset_1():
    execute_test_with_dataset(
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
            [1.0,  0.0,  1.0,  0.0],
            [1.0,  0.5,  1.0,  0.5],
            [1.0,  1.0,  1.0,  1.0]
        ]
    )


class TestDatasetPipelinePart(PipelinePart2WithCallback):
    def __init__(self, original_part: PipelinePart2, expected_dataset: list[list[float]]):
        super().__init__()
        self.uuid = original_part.uuid
        self.original_part = original_part
        self.expected_dataset = expected_dataset

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return self.original_part.to_grpc_part()

    def execute_callback(self, values: dict[str, GrpcContextValue], labeled_log_traces_dataset=None):
        dataset = values[const_labeled_log_traces_dataset].labeled_dataset
        df = from_grpc_labeled_dataset(dataset).drop([const_cluster_labels], axis=1)
        assert df.values.tolist() == self.expected_dataset


def execute_test_with_dataset(names_log, clusterization_pipeline, expected_raw_dataset):
    _execute_test_with_names_log(
        names_log,
        Pipeline2(
            UseNamesEventLog2(),
            TestDatasetPipelinePart(clusterization_pipeline, expected_raw_dataset),
        )
    )
