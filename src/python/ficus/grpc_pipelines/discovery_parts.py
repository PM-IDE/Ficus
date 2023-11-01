from ficus.discovery.graph import draw_graph, from_grpc_graph
from ficus.discovery.petri_net import draw_petri_net
from ficus.grpc_pipelines.context_values import from_grpc_petri_net
from ficus.grpc_pipelines.grpc_pipelines import *
from ficus.grpc_pipelines.grpc_pipelines import _create_default_pipeline_part, _create_simple_get_context_value_part, \
    _create_complex_get_context_part
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *


class DiscoverPetriNetAlpha2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_default_discovery_part(const_discover_petri_net_alpha)


def _create_default_discovery_part(algo_name: str) -> GrpcPipelinePartBase:
    config = GrpcPipelinePartConfiguration()
    return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(algo_name, config))


class DiscoverPetriNetAlphaPlus2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_default_discovery_part(const_discover_petri_net_alpha_plus)


class DiscoverPetriNetAlphaPlusPlus2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_default_discovery_part(const_discover_petri_net_alpha_plus_plus)


class DiscoverPetriNetAlphaPlusPlusNfc2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return _create_default_discovery_part(const_discover_petri_net_alpha_plus_plus_nfc)


class DiscoverPetriNetHeuristic2(PipelinePart2):
    def __init__(self, dependency_threshold: float, positive_observations_threshold: int, relative_to_best_threshold: float):
        super().__init__()
        self.dependency_threshold = dependency_threshold
        self.positive_observations_threshold = positive_observations_threshold
        self.relative_to_best_threshold = relative_to_best_threshold

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_float_value(config, const_dependency_relation_threshold, self.dependency_threshold)
        append_uint32_value(config, const_positive_observations_threshold, self.positive_observations_threshold)
        append_float_value(config, const_relative_to_best_threshold, self.relative_to_best_threshold)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_discover_petri_net_heuristic, config))


class SerializePetriNetToPNML2(PipelinePart2):
    def __init__(self, save_path, use_names_as_ids: bool = False):
        super().__init__()
        self.save_path = save_path
        self.use_names_as_ids = use_names_as_ids

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_path, self.save_path)
        append_bool_value(config, const_pnml_use_names_as_ids, self.use_names_as_ids)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_serialize_petri_net_to_pnml, config))


class ViewGraphLikeFormalismPart2(PipelinePart2WithCallback):
    def __init__(self,
                 name: str = 'petri_net',
                 background_color: str = 'white',
                 engine='dot',
                 export_path: Optional[str] = None,
                 rankdir: str = 'LR'):
        super().__init__()
        self.export_path = export_path
        self.name = name
        self.background_color = background_color
        self.engine = engine
        self.rankdir = rankdir


class ViewPetriNet2(ViewGraphLikeFormalismPart2):
    def __init__(self,
                 show_places_names: bool = False,
                 name: str = 'petri_net',
                 background_color: str = 'white',
                 engine='dot',
                 export_path: Optional[str] = None,
                 rankdir: str = 'LR'):
        super().__init__(name, background_color, engine, export_path, rankdir)
        self.show_places_names = show_places_names

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


class ViewDirectlyFollowsGraph(ViewGraphLikeFormalismPart2):
    def __init__(self,
                 name: str = 'dfg_graph',
                 background_color: str = 'white',
                 engine='dot',
                 export_path: Optional[str] = None,
                 rankdir: str = 'LR'):
        super().__init__(name, background_color, engine, export_path, rankdir)

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        part = _create_complex_get_context_part(self.uuid, const_graph, const_discover_directly_follows_graph, GrpcPipelinePartConfiguration())
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        draw_graph(from_grpc_graph(context_value.graph),
                   name=self.name,
                   background_color=self.background_color,
                   engine=self.engine,
                   rankdir=self.rankdir,
                   export_path=self.export_path)
