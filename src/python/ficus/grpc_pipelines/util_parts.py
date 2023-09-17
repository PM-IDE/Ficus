from ficus.grpc_pipelines.constants import const_use_names_event_log, const_names_event_log, const_get_names_event_log, \
    const_pipeline
from ficus.grpc_pipelines.grpc_pipelines import PipelinePart2, _create_default_pipeline_part, PipelinePart2WithCallback, \
    _create_complex_get_context_part, Pipeline2, append_pipeline_value, PrintEventLogInfo2
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import GrpcPipelinePartBase, GrpcPipelinePartConfiguration, \
    GrpcContextValue


class UseNamesEventLog2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part(const_use_names_event_log))


class PrintEventLog2(PipelinePart2WithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()
        part = _create_complex_get_context_part(self.uuid, const_names_event_log, const_get_names_event_log, config)
        return GrpcPipelinePartBase(complexContextRequestPart=part)

    def execute_callback(self, context_value: GrpcContextValue):
        for trace in context_value.names_log.log.traces:
            print(list(trace.events))


class PrintEventlogInfoBeforeAfter(PipelinePart2):
    def __init__(self, inner_pipeline: Pipeline2):
        super().__init__()
        self.inner_pipeline = inner_pipeline

    def to_grpc_part(self) -> GrpcPipelinePartBase:
        config = GrpcPipelinePartConfiguration()

        pipeline = Pipeline2(
            PrintEventLogInfo2(),
        )

        for part in self.inner_pipeline.parts:
            pipeline.parts.append(part)

        pipeline.parts.append(PrintEventLogInfo2())

        append_pipeline_value(config, const_pipeline, pipeline)

        return GrpcPipelinePartBase(defaultPart=_create_default_pipeline_part())
