from ficus.grpc_pipelines.constants import *
from ficus.grpc_pipelines.grpc_pipelines import PipelinePart2, _create_default_pipeline_part
from ficus.grpc_pipelines.models.backend_service_pb2 import *
from ficus.grpc_pipelines.models.backend_service_pb2_grpc import *
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *
from ficus.grpc_pipelines.models.util_pb2 import *


class AddStartEndArtificialEvents2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_add_start_end_artificial_events, config))


class AddStartArtificialEvents2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_add_start_artificial_events, config))


class AddEndArtificialEvents2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_add_end_artificial_events, config))
