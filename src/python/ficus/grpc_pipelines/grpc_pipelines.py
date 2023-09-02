from ficus.grpc_pipelines.context_values import ContextValue, from_grpc_names_log
from ficus.grpc_pipelines.models.backend_service_pb2 import *
from ficus.grpc_pipelines.models.backend_service_pb2_grpc import *
from ficus.grpc_pipelines.models.context_pb2 import *
from ficus.grpc_pipelines.models.pipelines_pb2 import *
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

            return last_result

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
        part = _create_empty_pipeline_part()
        part.name = "ReadLogFromXes"

        return GrpcPipelinePartBase(defaultPart=part)


class TracesDiversityDiagram2(PipelinePart2WithCallback):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        return GrpcPipelinePartBase(contextRequestPart=_create_get_context_value_part("names_event_log"))

    def execute_callback(self, context_value: GrpcContextValue):
        print(from_grpc_names_log(context_value.names_log))


def _create_get_context_value_part(key_name: str):
    return GrpcContextRequestPipelinePart(key=GrpcContextKey(name=key_name))


def _create_empty_pipeline_part():
    return GrpcPipelinePart(configuration=GrpcPipelinePartConfiguration())
