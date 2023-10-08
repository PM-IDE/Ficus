from ficus.discovery.petri_net import draw_petri_net
from ficus.grpc_pipelines.context_values import from_grpc_petri_net
from ficus.grpc_pipelines.grpc_pipelines import *
from ficus.grpc_pipelines.grpc_pipelines import _create_default_pipeline_part, _create_simple_get_context_value_part
from ficus.grpc_pipelines.models.backend_service_pb2 import *
from ficus.grpc_pipelines.models.backend_service_pb2_grpc import *
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *
from ficus.grpc_pipelines.models.util_pb2 import *


class DiscoverPetriNetAlpha2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_discover_petri_net_alpha, config))


class SerializePetriNetToPNML2(PipelinePart2):
    def __init__(self, save_path):
        super().__init__()
        self.save_path = save_path

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_path, self.save_path)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_serialize_petri_net_to_pnml, config))


class ViewPetriNet2(PipelinePart2WithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        part = _create_simple_get_context_value_part(self.uuid, const_petri_net)
        return GrpcPipelinePartBase(simpleContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        petri_net = from_grpc_petri_net(context_value.petriNet)
        draw_petri_net(petri_net)
