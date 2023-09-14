from ficus.grpc_pipelines.constants import const_read_log_from_xes
from ficus.grpc_pipelines.grpc_pipelines import PipelinePart2, _create_default_pipeline_part
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase


class ReadLogFromXes2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_log_from_xes))
