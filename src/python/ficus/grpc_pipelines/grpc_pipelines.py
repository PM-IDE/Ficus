from dataclasses import dataclass
from enum import Enum
from typing import Optional

from ficus.pipelines.analysis.patterns.models import AdjustingMode

from ficus.analysis.event_log_analysis import draw_colors_event_log
from ficus.grpc_pipelines.constants import *
from ficus.grpc_pipelines.context_values import ContextValue, from_grpc_colors_log, \
    StringContextValue, Uint32ContextValue, BoolContextValue, EnumContextValue, from_grpc_event_log_info, \
    StringsContextValue
from ficus.grpc_pipelines.models.backend_service_pb2 import *
from ficus.grpc_pipelines.models.backend_service_pb2_grpc import *
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *
from ficus.grpc_pipelines.models.util_pb2 import *


class Pipeline2:
    def __init__(self, *parts):
        self.parts = parts

    def execute(self, initial_context: dict[str, ContextValue]) -> GrpcGuid:
        with grpc.insecure_channel('localhost:8080') as channel:
            stub = GrpcBackendServiceStub(channel)
            parts = list(self.parts)
            request = GrpcPipelineExecutionRequest(
                pipeline=self._create_grpc_pipeline(parts),
                initialContext=self._create_initial_context(initial_context)
            )

            callback_parts = self._find_pipeline_parts_with_callbacks(parts)
            last_result = None
            callback_part_index = 0

            for part_result in stub.ExecutePipeline(request):
                last_result = part_result

                if last_result.HasField('finalResult'):
                    break

                if last_result.HasField('pipelinePartResult') and callback_part_index < len(callback_parts):
                    callback_parts[callback_part_index].execute_callback(part_result.pipelinePartResult.contextValue)
                    callback_part_index += 1

                if last_result.HasField('logMessage'):
                    print(part_result.logMessage.message)

            return last_result

    def to_grpc_pipeline(self):
        return self._create_grpc_pipeline(list(self.parts))

    @staticmethod
    def _create_grpc_pipeline(parts) -> GrpcPipeline:
        pipeline = GrpcPipeline()
        for part in parts:
            if not isinstance(part, PipelinePart2):
                raise TypeError()

            pipeline.parts.append(part.to_grpc_part())

        return pipeline

    @staticmethod
    def _find_pipeline_parts_with_callbacks(parts) -> list["PipelinePart2WithCallback"]:
        result = []
        for part in parts:
            if isinstance(part, PipelinePart2WithCallback):
                result.append(part)

        return result

    @staticmethod
    def _create_initial_context(context: dict[str, ContextValue]) -> list[GrpcContextKeyValue]:
        result = []
        for key, value in context.items():
            result.append(GrpcContextKeyValue(
                key=GrpcContextKey(name=key),
                value=value.to_grpc_context_value()
            ))

        return result


class PipelinePart2:
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        raise NotImplementedError()


class PipelinePart2WithCallback(PipelinePart2):
    def execute_callback(self, context_value: GrpcContextValue):
        raise NotImplementedError()


class ReadLogFromXes2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_read_log_from_xes))


class UseNamesEventLog2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_use_names_event_log))


class PipelinePart2WithDrawColorsLogCallback(PipelinePart2WithCallback):
    def __init__(self,
                 title: str = None,
                 save_path: str = None,
                 plot_legend: bool = True,
                 height_scale: int = 1,
                 width_scale: int = 1):
        self.title = title
        self.save_path = save_path
        self.plot_legend = plot_legend
        self.height_scale = height_scale
        self.width_scale = width_scale

    def execute_callback(self, context_value: GrpcContextValue):
        colors_log = from_grpc_colors_log(context_value.colors_log)
        draw_colors_event_log(colors_log,
                              title=self.title,
                              save_path=self.save_path,
                              plot_legend=self.plot_legend,
                              height_scale=self.height_scale,
                              width_scale=self.width_scale)


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


class PatternsDiscoveryStrategy(Enum):
    FromAllTraces = 0
    FromSingleMergedTrace = 1


class FindTandemArrays2(PipelinePart2):
    def __init__(self,
                 part_type: str,
                 max_array_length: int,
                 class_extractor: Optional[str]):
        self.max_array_length = max_array_length
        self.part_type = part_type
        self.class_extractor = class_extractor

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_tandem_array_length, self.max_array_length)
        if self.class_extractor is not None:
            append_string_value(config, const_event_class_regex, self.class_extractor)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(self.part_type, config))


class FindPrimitiveTandemArrays2(FindTandemArrays2):
    def __init__(self, max_array_length: int, class_extractor: Optional[str] = None):
        super().__init__(part_type=const_find_primitive_tandem_arrays,
                         max_array_length=max_array_length,
                         class_extractor=class_extractor)


class FindMaximalTandemArrays2(FindTandemArrays2):
    def __init__(self, max_array_length: int, class_extractor: Optional[str] = None):
        super().__init__(part_type=const_find_maximal_tandem_arrays,
                         max_array_length=max_array_length,
                         class_extractor=class_extractor)


class FindRepeats2(PipelinePart2):
    def __init__(self,
                 part_name: str,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        self.strategy = strategy
        self.part_name = part_name
        self.class_extractor = class_extractor

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)
        if self.class_extractor is not None:
            append_string_value(config, const_event_class_regex, self.class_extractor)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(self.part_name, config))


class FindMaximalRepeats2(FindRepeats2):
    def __init__(self,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        super().__init__(part_name=const_find_maximal_repeats,
                         strategy=strategy,
                         class_extractor=class_extractor)


class FindSuperMaximalRepeats2(FindRepeats2):
    def __init__(self,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        super().__init__(part_name=const_find_super_maximal_repeats,
                         strategy=strategy,
                         class_extractor=class_extractor)


class FindNearSuperMaximalRepeats2(FindRepeats2):
    def __init__(self,
                 strategy: PatternsDiscoveryStrategy,
                 class_extractor: Optional[str] = None):
        super().__init__(part_name=const_find_near_super_maximal_repeats,
                         strategy=strategy,
                         class_extractor=class_extractor)


class DiscoverActivities2(PipelinePart2):
    def __init__(self, activity_level: int):
        self.activity_level = activity_level

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_activity_level, self.activity_level)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_discover_activities, config))


class DiscoverActivitiesInstances2(PipelinePart2):
    def __init__(self, narrow_activities: bool):
        self.narrow_activities = narrow_activities

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_bool_value(config, const_narrow_activities, self.narrow_activities)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_discover_activities_instances, config))


class PatternsKind(Enum):
    PrimitiveTandemArrays = 0,
    MaximalTandemArrays = 1,
    MaximalRepeats = 2,
    SuperMaximalRepeats = 3,
    NearSuperMaximalRepeats = 4,


class DiscoverActivitiesForSeveralLevels2(PipelinePart2):
    def __init__(self,
                 event_classes: list[str],
                 patterns_kind: PatternsKind,
                 narrow_activities: bool = True,
                 activity_level: int = 0,
                 strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
                 max_array_length: int = 20,
                 adjusting_mode: AdjustingMode = AdjustingMode.FromAllLog,
                 min_events_count: int = 0):
        self.event_classes = event_classes
        self.narrow_activities = narrow_activities
        self.patterns_kind = patterns_kind
        self.activity_level = activity_level
        self.strategy = strategy
        self.max_array_length = max_array_length
        self.adjusting_mode = adjusting_mode
        self.min_events_count = min_events_count

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()

        append_bool_value(config, const_narrow_activities, self.narrow_activities)
        append_strings_context_value(config, const_event_classes_regexes, self.event_classes)
        append_adjusting_mode(config, const_adjusting_mode, self.adjusting_mode)
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_uint32_value(config, const_events_count, self.min_events_count)
        append_patterns_kind(config, const_patterns_kind, self.patterns_kind)
        append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)

        default_part = _create_default_pipeline_part(const_discover_activities_for_several_levels, config)
        return GrpcPipelinePartBase(defaultPart=default_part)


class DiscoverActivitiesFromPatterns2(PipelinePart2):
    def __init__(self,
                 patterns_kind: PatternsKind,
                 strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
                 max_array_length: int = 20,
                 activity_level: int = 0):
        self.patterns_kind = patterns_kind
        self.strategy = strategy
        self.max_array_length = max_array_length
        self.activity_level = activity_level

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        match self.patterns_kind:
            case PatternsKind.MaximalRepeats:
                patterns_part = FindMaximalRepeats2(strategy=self.strategy)
            case PatternsKind.SuperMaximalRepeats:
                patterns_part = FindSuperMaximalRepeats2(strategy=self.strategy)
            case PatternsKind.NearSuperMaximalRepeats:
                patterns_part = FindNearSuperMaximalRepeats2(strategy=self.strategy)
            case PatternsKind.PrimitiveTandemArrays:
                patterns_part = FindPrimitiveTandemArrays2(max_array_length=self.max_array_length)
            case PatternsKind.MaximalTandemArrays:
                patterns_part = FindMaximalTandemArrays2(max_array_length=self.max_array_length)
            case _:
                print(f"Unknown patterns_kind: {self.patterns_kind}")
                raise ValueError()

        pipeline = Pipeline2(
            patterns_part,
            DiscoverActivities2(activity_level=self.activity_level),
        )

        config = GrpcPipelinePartConfiguration()
        append_pipeline_value(config, const_pipeline, pipeline)

        default_part = _create_default_pipeline_part(const_discover_activities_from_patterns, config)
        return GrpcPipelinePartBase(defaultPart=default_part)


class PrintEventLogInfo2(PipelinePart2WithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(const_event_log_info, const_get_event_log_info, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        log_info = from_grpc_event_log_info(context_value.event_log_info)
        print(log_info)


class FilterTracesByEventsCount2(PipelinePart2):
    def __init__(self, min_events_in_trace: int):
        self.min_events_in_trace = min_events_in_trace

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_events_count, self.min_events_in_trace)
        part = _create_default_pipeline_part(const_filter_traces_by_events_count, config)
        return GrpcPipelinePartBase(defaultPart=part)


def _create_simple_get_context_value_part(key_name: str):
    return GrpcSimpleContextRequestPipelinePart(key=GrpcContextKey(name=key_name))


def _create_complex_get_context_part(key_name: str, before_part_name: str, config: GrpcPipelinePartConfiguration):
    return GrpcComplexContextRequestPipelinePart(
        key=GrpcContextKey(name=key_name),
        beforePipelinePart=GrpcPipelinePart(
            name=before_part_name,
            configuration=config
        )
    )


def _create_default_pipeline_part(name: str, config=GrpcPipelinePartConfiguration()):
    return GrpcPipelinePart(configuration=config, name=name)


def append_string_value(config: GrpcPipelinePartConfiguration, key: str, value: str):
    _append_context_value(config, key, StringContextValue(value))


def _append_context_value(config: GrpcPipelinePartConfiguration, key: str, value: ContextValue):
    config.configurationParameters.append(GrpcContextKeyValue(
        key=GrpcContextKey(name=key),
        value=value.to_grpc_context_value()
    ))


def append_uint32_value(config: GrpcPipelinePartConfiguration, key: str, value: int):
    _append_context_value(config, key, Uint32ContextValue(value))


def append_bool_value(config: GrpcPipelinePartConfiguration, key: str, value: bool):
    _append_context_value(config, key, BoolContextValue(value))


def append_enum_value(config: GrpcPipelinePartConfiguration, key: str, enum_name: str, value: str):
    _append_context_value(config, key, EnumContextValue(enum_name, value))


def append_patterns_discovery_strategy(config: GrpcPipelinePartConfiguration, key: str, value: PatternsDiscoveryStrategy):
    append_enum_value(config, key, const_pattern_discovery_strategy_enum_name, value.name)

def append_strings_context_value(config: GrpcPipelinePartConfiguration, key: str, value: list[str]):
    _append_context_value(config, key, StringsContextValue(value))

def append_patterns_kind(config: GrpcPipelinePartConfiguration, key: str, value: PatternsKind):
    append_enum_value(config, key, const_patterns_kind_enum_name, value.name)

def append_adjusting_mode(config: GrpcPipelinePartConfiguration, key: str, value: AdjustingMode):
    append_enum_value(config, key, const_adjusting_mode_enum_name, value.name)


@dataclass
class PipelineContextValue(ContextValue):
    pipeline: Pipeline2


    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(pipeline=self.pipeline.to_grpc_pipeline())


def append_pipeline_value(config: GrpcPipelinePartConfiguration, key: str, value: Pipeline2):
    _append_context_value(config, key, PipelineContextValue(value))
