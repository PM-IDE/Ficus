from ficus.discovery.petri_net import draw_petri_net
from ficus.grpc_pipelines.context_values import from_grpc_petri_net
from ficus.grpc_pipelines.grpc_pipelines import *
from ficus.grpc_pipelines.grpc_pipelines import _create_default_pipeline_part, _create_simple_get_context_value_part
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *


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
    def __init__(self,
                 show_places_names: bool = False,
                 name: str = 'petri_net',
                 background_color: str = 'white',
                 engine='dot',
                 export_path: Optional[str] = None,
                 rankdir: str = 'LR'):
        super().__init__()
        self.export_path = export_path
        self.show_places_names = show_places_names
        self.name = name
        self.background_color = background_color
        self.engine = engine
        self.rankdir = rankdir

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        part = _create_simple_get_context_value_part(self.uuid, const_petri_net)
        return GrpcPipelinePartBase(simpleContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        draw_petri_net(from_grpc_petri_net(context_value.petriNet),
                       show_places_names=self.show_places_names,
                       name=self.name,
                       background_color=self.background_color,
                       engine=self.engine,
                       rankdir=self.rankdir,
                       export_path=self.export_path)
