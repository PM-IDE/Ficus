from ficus.grpc_pipelines.constants import const_colors_event_log, const_traces_diversity_diagram, const_event_name, \
    const_draw_placement_of_event_by_name, const_regex, const_draw_placement_of_event_by_regex, \
    const_draw_full_activities_diagram, const_draw_short_activities_diagram
from ficus.grpc_pipelines.context_values import StringContextValue
from ficus.grpc_pipelines.grpc_pipelines import PipelinePart2WithDrawColorsLogCallback, \
    _create_complex_get_context_part, append_string_value
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
    GrpcContextKeyValue, GrpcContextKey


class TracesDiversityDiagram2(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(const_colors_event_log, const_traces_diversity_diagram, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawPlacementsOfEventByName2(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 event_name: str,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.event_name = event_name

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        config.configurationParameters.append(GrpcContextKeyValue(
            key=GrpcContextKey(name=const_event_name),
            value=StringContextValue(self.event_name).to_grpc_context_value()
        ))

        part = _create_complex_get_context_part(const_colors_event_log, const_draw_placement_of_event_by_name, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawPlacementOfEventsByRegex2(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 regex: str,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.regex = regex

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_regex, self.regex)

        part = _create_complex_get_context_part(const_colors_event_log, const_draw_placement_of_event_by_regex, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawActivitiesDiagramBase2(PipelinePart2WithDrawColorsLogCallback):
    def __init__(self,
                 diagram_kind: str,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)

        self.diagram_kind = diagram_kind

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(const_colors_event_log, self.diagram_kind, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)


class DrawFullActivitiesDiagram2(DrawActivitiesDiagramBase2):
    def __init__(self,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(const_draw_full_activities_diagram,
                         title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)


class DrawShortActivitiesDiagram2(DrawActivitiesDiagramBase2):
    def __init__(self,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        super().__init__(const_draw_short_activities_diagram,
                         title=title,
                         save_path=save_path,
                         plot_legend=plot_legend,
                         height_scale=height_scale,
                         width_scale=width_scale)
