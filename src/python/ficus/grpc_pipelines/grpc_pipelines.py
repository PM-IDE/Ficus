from ficus.grpc_pipelines.context_values import ContextValue
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
            request = GrpcPipelineExecutionRequest(
                pipeline=self._create_grpc_pipeline(list(self.parts)),
                initialContext=self._create_initial_context(initial_context)
            )

            return stub.ExecutePipeline(request)

    @staticmethod
    def _create_grpc_pipeline(parts) -> GrpcPipeline:
        pipeline = GrpcPipeline()
        for part in parts:
            if not isinstance(part, PipelinePart2):
                raise TypeError()

            pipeline.parts.append(part.to_grpc_part())

        return pipeline

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


class ReadLogFromXes2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePartBase:
        part = _create_empty_pipeline_part()
        part.name = "ReadLogFromXes"

        return GrpcPipelinePartBase(defaultPart=part)


def _create_empty_pipeline_part():
    return GrpcPipelinePart(configuration=GrpcPipelinePartConfiguration())
