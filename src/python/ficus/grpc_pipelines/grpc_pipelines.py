from dataclasses import dataclass

from ficus.analysis.event_log_analysis import draw_colors_event_log
from ficus.grpc_pipelines.constants import *
from ficus.grpc_pipelines.context_values import ContextValue, from_grpc_colors_log, \
    StringContextValue, Uint32ContextValue, BoolContextValue, EnumContextValue, from_grpc_event_log_info, \
    StringsContextValue
from ficus.grpc_pipelines.data_models import PatternsDiscoveryStrategy, PatternsKind
from ficus.grpc_pipelines.models.backend_service_pb2 import *
from ficus.grpc_pipelines.models.backend_service_pb2_grpc import *
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *
from ficus.grpc_pipelines.models.util_pb2 import *
from ficus.pipelines.analysis.patterns.models import AdjustingMode


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

            callback_parts = []
            self.append_parts_with_callbacks(callback_parts)
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

    def append_parts_with_callbacks(self, parts: list['PipelinePart2WithCallback']):
        for part in list(self.parts):
            part.append_parts_with_callbacks(parts)

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

    def append_parts_with_callbacks(self, parts: list['PipelinePart2WithCallback']):
        pass


class PipelinePart2WithCallback(PipelinePart2):
    def execute_callback(self, context_value: GrpcContextValue):
        raise NotImplementedError()


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

    def append_parts_with_callbacks(self, parts: list['PipelinePart2WithCallback']):
        super().append_parts_with_callbacks(parts)
        parts.append(self)


class PrintEventLogInfo2(PipelinePart2WithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(const_event_log_info, const_get_event_log_info, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        log_info = from_grpc_event_log_info(context_value.event_log_info)
        print(log_info)


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


def append_patterns_discovery_strategy(config: GrpcPipelinePartConfiguration, key: str,
                                       value: PatternsDiscoveryStrategy):
    append_enum_value(config, key, const_pattern_discovery_strategy_enum_name, value.name)


def append_strings_context_value(config: GrpcPipelinePartConfiguration, key: str, value: list[str]):
    _append_context_value(config, key, StringsContextValue(value))


def append_patterns_kind(config: GrpcPipelinePartConfiguration, key: str, value: PatternsKind):
    append_enum_value(config, key, const_patterns_kind_enum_name, value.name)


def append_adjusting_mode(config: GrpcPipelinePartConfiguration, key: str, value: AdjustingMode):
    append_enum_value(config, key, const_adjusting_mode_enum_name, value.name)


def append_pipeline_value(config: GrpcPipelinePartConfiguration, key: str, value: Pipeline2):
    _append_context_value(config, key, PipelineContextValue(value))


@dataclass
class PipelineContextValue(ContextValue):
    pipeline: Pipeline2

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(pipeline=self.pipeline.to_grpc_pipeline())
