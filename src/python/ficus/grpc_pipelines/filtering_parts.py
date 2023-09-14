from ficus.grpc_pipelines.constants import const_events_count, const_filter_traces_by_events_count
from ficus.grpc_pipelines.grpc_pipelines import PipelinePart2, _create_default_pipeline_part, append_uint32_value
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration


class FilterTracesByEventsCount2(PipelinePart2):
    def __init__(self, min_events_in_trace: int):
        self.min_events_in_trace = min_events_in_trace

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_events_count, self.min_events_in_trace)
        part = _create_default_pipeline_part(const_filter_traces_by_events_count, config)
        return GrpcPipelinePartBase(defaultPart=part)
