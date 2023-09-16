from ficus.analysis.patterns.patterns_models import UndefinedActivityHandlingStrategy
from ficus.grpc_pipelines.constants import *
from ficus.grpc_pipelines.data_models import PatternsKind, PatternsDiscoveryStrategy, NarrowActivityKind
from ficus.grpc_pipelines.grpc_pipelines import *
from ficus.grpc_pipelines.grpc_pipelines import _create_default_pipeline_part, _create_complex_get_context_part
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
    GrpcContextValue
from ficus.grpc_pipelines.patterns_parts import FindMaximalRepeats2, \
    FindSuperMaximalRepeats2, FindNearSuperMaximalRepeats2, FindPrimitiveTandemArrays2, FindMaximalTandemArrays2
from ficus.pipelines.analysis.patterns.models import AdjustingMode


class DiscoverActivities2(PipelinePart2):
    def __init__(self, activity_level: int):
        super().__init__()
        self.activity_level = activity_level

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_activity_level, self.activity_level)
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_discover_activities, config))


class DiscoverActivitiesInstances2(PipelinePart2):
    def __init__(self, narrow_activities: NarrowActivityKind, min_events_in_activity: int = 0):
        super().__init__()
        self.narrow_activities = narrow_activities
        self.min_events_in_activity = min_events_in_activity

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
        append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity)

        return GrpcPipelinePartBase(
            defaultPart=_create_default_pipeline_part(const_discover_activities_instances, config))


class CreateLogFromActivitiesInstances2(PipelinePart2):
    def __init__(self,
                 strategy: UndefinedActivityHandlingStrategy = UndefinedActivityHandlingStrategy.DontInsert):
        super().__init__()
        self.strategy = strategy

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_undef_activity_handling_strat(config, const_undef_activity_handling_strategy, self.strategy)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_create_log_from_activities, config))


class DiscoverActivitiesForSeveralLevels2(PipelinePart2):
    def __init__(self,
                 event_classes: list[str],
                 patterns_kind: PatternsKind,
                 narrow_activities: NarrowActivityKind = NarrowActivityKind.NarrowDown,
                 activity_level: int = 0,
                 strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
                 max_array_length: int = 20,
                 adjusting_mode: AdjustingMode = AdjustingMode.FromAllLog,
                 min_events_in_unattached_subtrace_count: int = 0,
                 min_events_in_activity_count: int = 0):
        super().__init__()
        self.event_classes = event_classes
        self.narrow_activities = narrow_activities
        self.patterns_kind = patterns_kind
        self.activity_level = activity_level
        self.strategy = strategy
        self.max_array_length = max_array_length
        self.adjusting_mode = adjusting_mode
        self.min_events_in_unattached_subtrace_count = min_events_in_unattached_subtrace_count
        self.min_events_in_activity_count = min_events_in_activity_count

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()

        append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
        append_strings_context_value(config, const_event_classes_regexes, self.event_classes)
        append_adjusting_mode(config, const_adjusting_mode, self.adjusting_mode)
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_uint32_value(config, const_events_count, self.min_events_in_unattached_subtrace_count)
        append_patterns_kind(config, const_patterns_kind, self.patterns_kind)
        append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)
        append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity_count)

        default_part = _create_default_pipeline_part(const_discover_activities_for_several_levels, config)
        return GrpcPipelinePartBase(defaultPart=default_part)


class DiscoverActivitiesFromPatterns2(PipelinePart2):
    def __init__(self,
                 patterns_kind: PatternsKind,
                 strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
                 max_array_length: int = 20,
                 activity_level: int = 0):
        super().__init__()
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


class DiscoverActivitiesUntilNoMore2(PipelinePart2):
    def __init__(self,
                 event_class: str = None,
                 patterns_kind: PatternsKind = PatternsKind.MaximalRepeats,
                 narrow_activities: NarrowActivityKind = NarrowActivityKind.NarrowDown,
                 activity_level: int = 0,
                 strategy: PatternsDiscoveryStrategy = PatternsDiscoveryStrategy.FromAllTraces,
                 undef_strategy: UndefinedActivityHandlingStrategy = UndefinedActivityHandlingStrategy.DontInsert,
                 max_array_length: int = 20,
                 adjusting_mode: AdjustingMode = AdjustingMode.FromAllLog,
                 min_events_in_unattached_subtrace_count: int = 0,
                 min_events_in_activity_count: int = 0):
        super().__init__()
        self.event_class = event_class
        self.narrow_activities = narrow_activities
        self.patterns_kind = patterns_kind
        self.activity_level = activity_level
        self.strategy = strategy
        self.undef_strategy = undef_strategy
        self.max_array_length = max_array_length
        self.adjusting_mode = adjusting_mode
        self.min_events_count = min_events_in_unattached_subtrace_count
        self.min_events_in_activity_count = min_events_in_activity_count

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()

        append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
        append_adjusting_mode(config, const_adjusting_mode, self.adjusting_mode)
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_uint32_value(config, const_events_count, self.min_events_count)
        append_patterns_kind(config, const_patterns_kind, self.patterns_kind)
        append_patterns_discovery_strategy(config, const_patterns_discovery_strategy, self.strategy)
        append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity_count)
        append_undef_activity_handling_strat(config, const_undef_activity_handling_strategy, self.undef_strategy)

        if self.event_class is not None:
            append_string_value(config, const_event_class_regex, self.event_class)

        default_part = _create_default_pipeline_part(const_discover_activities_until_no_more, config)
        return GrpcPipelinePartBase(defaultPart=default_part)


class ExecuteWithEachActivityLog2(PipelinePart2):
    def __init__(self, activity_level: int, activity_log_pipeline: Pipeline2):
        super().__init__()
        self.activity_level = activity_level
        self.activity_log_pipeline = activity_log_pipeline

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_pipeline_value(config, const_pipeline, self.activity_log_pipeline)
        append_uint32_value(config, const_activity_level, self.activity_level)

        default_part = _create_default_pipeline_part(const_execute_with_each_activity_log, config)
        return GrpcPipelinePartBase(defaultPart=default_part)

    def append_parts_with_callbacks(self, parts: list['PipelinePart2WithCallback']):
        super().append_parts_with_callbacks(parts)
        self.activity_log_pipeline.append_parts_with_callbacks(parts)


class SubstituteUnderlyingEvents2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_substitute_underlying_events))


class ClearActivitiesRelatedStuff2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_clear_activities_related_stuff))


class PrintNumberOfUnderlyingEvents2(PipelinePart2WithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        part = _create_complex_get_context_part(self.uuid,
                                                const_underlying_events_count,
                                                const_get_number_of_underlying_events,
                                                GrpcPipelinePartConfiguration())

        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        print(f'Underlying events count: {context_value.uint32}')
