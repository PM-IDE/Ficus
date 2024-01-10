from sklearn.decomposition import PCA

from ficus.analysis.event_log_analysis import draw_pca_results
from ficus.grpc_pipelines.context_values import from_grpc_ficus_dataset, from_grpc_labeled_dataset
from ficus.grpc_pipelines.data_models import ActivitiesRepresentationSource, Distance
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
    def __init__(self,
                 narrow_activities: NarrowActivityKind = NarrowActivityKind.NarrowDown,
                 min_events_in_activity: int = 0,
                 activity_filter_kind: ActivityFilterKind = ActivityFilterKind.DefaultFilter):
        super().__init__()
        self.narrow_activities = narrow_activities
        self.min_events_in_activity = min_events_in_activity
        self.activity_filter_kind = activity_filter_kind

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_narrow_kind(config, const_narrow_activities, self.narrow_activities)
        append_uint32_value(config, const_min_events_in_activity, self.min_events_in_activity)
        append_activity_filter_kind(config, const_activity_filter_kind, self.activity_filter_kind)

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
                 min_events_in_activity_count: int = 0,
                 activity_filter_kind: ActivityFilterKind = ActivityFilterKind.DefaultFilter):
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
        self.activity_filter_kind = activity_filter_kind

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
        append_activity_filter_kind(config, const_activity_filter_kind, self.activity_filter_kind)

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

        default_part = _create_default_pipeline_part(const_execute_frontend_part, config)
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
                 min_events_in_activity_count: int = 0,
                 activity_filter_kind: ActivityFilterKind = ActivityFilterKind.DefaultFilter,
                 after_activities_extraction_pipeline: Optional[Pipeline2] = None):
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
        self.activity_filter_kind = activity_filter_kind
        self.after_activities_extraction_pipeline = after_activities_extraction_pipeline

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
        append_activity_filter_kind(config, const_activity_filter_kind, self.activity_filter_kind)

        if self.event_class is not None:
            append_string_value(config, const_event_class_regex, self.event_class)

        if self.after_activities_extraction_pipeline is not None:
            append_pipeline_value(config, const_pipeline, self.after_activities_extraction_pipeline)

        default_part = _create_default_pipeline_part(const_discover_activities_until_no_more, config)
        return GrpcPipelinePartBase(defaultPart=default_part)

    def append_parts_with_callbacks(self, parts: list['PipelinePart2WithCallback']):
        super().append_parts_with_callbacks(parts)
        self.after_activities_extraction_pipeline.append_parts_with_callbacks(parts)


class ExecuteWithEachActivityLog2(PipelinePart2):
    def __init__(self, activities_logs_source: ActivitiesLogsSource, activity_level: int,
                 activity_log_pipeline: Pipeline2):
        super().__init__()
        self.activity_level = activity_level
        self.activity_log_pipeline = activity_log_pipeline
        self.activities_logs_source = activities_logs_source

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_pipeline_value(config, const_pipeline, self.activity_log_pipeline)
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_activities_logs_source(config, const_activities_logs_source, self.activities_logs_source)

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
                                                [const_underlying_events_count],
                                                const_get_number_of_underlying_events,
                                                GrpcPipelinePartConfiguration())

        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        print(f'Underlying events count: {values[const_underlying_events_count].uint32}')


class ApplyClassExtractor2(PipelinePart2):
    def __init__(self, class_extractor_regex: str, filter_regex: str = ".*"):
        super().__init__()
        self.class_extractor_regex = class_extractor_regex
        self.filter_regex = filter_regex

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_string_value(config, const_event_class_regex, self.class_extractor_regex)
        append_string_value(config, const_regex, self.filter_regex)

        part = _create_default_pipeline_part(const_apply_class_extractor, config)
        return GrpcPipelinePartBase(defaultPart=part)


class ClusterizationPartWithPCAVisualization2(PipelinePart2WithCallback):
    def __init__(self,
                 show_visualization: bool,
                 fig_size: (int, int),
                 font_size: int,
                 save_path: Optional[str]):
        super().__init__()
        self.show_visualization = show_visualization
        self.fig_size = fig_size
        self.font_size = font_size
        self.save_path = save_path

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        if not self.show_visualization:
            return

        dataset = values[const_labeled_traces_activities_dataset].labeled_dataset
        df = from_grpc_labeled_dataset(dataset)
        pca = PCA(n_components=3)
        pca_result = pca.fit_transform(df.loc[:, df.columns != const_cluster_labels].values)

        draw_pca_results(df, pca_result, self.fig_size, self.font_size, self.save_path, const_cluster_labels)


class ClusterizeActivitiesFromTracesKMeans(ClusterizationPartWithPCAVisualization2):
    def __init__(self,
                 activity_level: int = 0,
                 clusters_count: int = 10,
                 learning_iterations_count: int = 200,
                 tolerance: float = 1e-5,
                 class_extractor: Optional[str] = None,
                 show_visualization: bool = True,
                 fig_size: (int, int) = (7, 9),
                 font_size: int = 14,
                 save_path: Optional[str] = None,
                 activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
                 distance: Distance = Distance.Cosine):
        super().__init__(show_visualization, fig_size, font_size, save_path)
        self.clusters_count = clusters_count
        self.learning_iterations_count = learning_iterations_count
        self.tolerance = tolerance
        self.activity_level = activity_level
        self.class_extractor = class_extractor
        self.activities_repr_source = activities_repr_source
        self.distance = distance

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_uint32_value(config, const_clusters_count, self.clusters_count)
        append_uint32_value(config, const_learning_iterations_count, self.learning_iterations_count)
        append_float_value(config, const_tolerance, self.tolerance)

        append_enum_value(config,
                          const_activities_representation_source,
                          const_activities_repr_source_enum_name,
                          self.activities_repr_source.name)

        append_enum_value(config,
                          const_distance,
                          const_distance_enum_name,
                          self.distance.name)

        if self.class_extractor is not None:
            append_string_value(config, const_event_class_regex, self.class_extractor)

        part = _create_complex_get_context_part(self.uuid,
                                                [const_labeled_traces_activities_dataset],
                                                const_clusterize_activities_from_traces_k_means,
                                                config)

        return GrpcPipelinePartBase(complexContextRequestPart=part)


class ClusterizeActivitiesFromTracesKMeansGridSearch(ClusterizationPartWithPCAVisualization2):
    def __init__(self,
                 activity_level: int = 0,
                 learning_iterations_count: int = 200,
                 tolerance: float = 1e-5,
                 class_extractor: Optional[str] = None,
                 show_visualization: bool = True,
                 fig_size: (int, int) = (7, 9),
                 font_size: int = 14,
                 activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
                 save_path: Optional[str] = None,
                 distance: Distance = Distance.Cosine):
        super().__init__(show_visualization, fig_size, font_size, save_path)
        self.learning_iterations_count = learning_iterations_count
        self.tolerance = tolerance
        self.activity_level = activity_level
        self.class_extractor = class_extractor
        self.activities_repr_source = activities_repr_source
        self.distance = distance

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_uint32_value(config, const_learning_iterations_count, self.learning_iterations_count)
        append_float_value(config, const_tolerance, self.tolerance)

        append_enum_value(config,
                          const_activities_representation_source,
                          const_activities_repr_source_enum_name,
                          self.activities_repr_source.name)

        append_enum_value(config,
                          const_distance,
                          const_distance_enum_name,
                          self.distance.name)

        if self.class_extractor is not None:
            append_string_value(config, const_event_class_regex, self.class_extractor)

        part = _create_complex_get_context_part(self.uuid,
                                                [const_labeled_traces_activities_dataset],
                                                const_clusterize_activities_from_traces_k_means_grid_search,
                                                config)

        return GrpcPipelinePartBase(complexContextRequestPart=part)


class ClusterizeActivitiesFromTracesDbscan(ClusterizationPartWithPCAVisualization2):
    def __init__(self,
                 activity_level: int = 0,
                 min_events_count_in_cluster: int = 1,
                 tolerance: float = 1e-5,
                 class_extractor: Optional[str] = None,
                 show_visualization: bool = True,
                 fig_size: (int, int) = (7, 9),
                 font_size: int = 14,
                 activities_repr_source: ActivitiesRepresentationSource = ActivitiesRepresentationSource.EventClasses,
                 save_path: Optional[str] = None,
                 distance: Distance = Distance.Cosine):
        super().__init__(show_visualization, fig_size, font_size, save_path)
        self.min_events_count_in_cluster = min_events_count_in_cluster
        self.tolerance = tolerance
        self.activity_level = activity_level
        self.class_extractor = class_extractor
        self.activities_repr_source = activities_repr_source
        self.distance = distance

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_activity_level, self.activity_level)
        append_uint32_value(config, const_min_events_in_cluster_count, self.min_events_count_in_cluster)
        append_float_value(config, const_tolerance, self.tolerance)

        append_enum_value(config,
                          const_activities_representation_source,
                          const_activities_repr_source_enum_name,
                          self.activities_repr_source.name)

        append_enum_value(config,
                          const_distance,
                          const_distance_enum_name,
                          self.distance.name)

        if self.class_extractor is not None:
            append_string_value(config, const_event_class_regex, self.class_extractor)

        part = _create_complex_get_context_part(self.uuid,
                                                [const_labeled_traces_activities_dataset],
                                                const_clusterize_activities_from_traces_dbscan,
                                                config)

        return GrpcPipelinePartBase(complexContextRequestPart=part)


class VisualizeTracesActivities2(PipelinePart2WithCallback):
    def __init__(self,
                 activity_level: int = 0,
                 class_extractor: Optional[str] = None,
                 fig_size: (int, int) = (7, 9),
                 font_size: int = 14,
                 save_path: Optional[str] = None):
        super().__init__()
        self.activity_level = activity_level
        self.class_extractor = class_extractor
        self.fig_size = fig_size
        self.font_size = font_size
        self.save_path = save_path

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        append_uint32_value(config, const_activity_level, self.activity_level)

        if self.class_extractor is not None:
            append_string_value(config, const_event_class_regex, self.class_extractor)

        part = _create_complex_get_context_part(self.uuid,
                                                [const_traces_activities_dataset],
                                                const_create_traces_activities_dataset,
                                                config)

        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, values: dict[str, GrpcContextValue]):
        dataset = values[const_traces_activities_dataset].dataset
        df = from_grpc_ficus_dataset(dataset)

        pca = PCA(n_components=3)
        pca_result = pca.fit_transform(df.values)
        draw_pca_results(df, pca_result, self.fig_size, self.font_size, save_path=self.save_path)
