from dataclasses import dataclass

from ficus.grpc_pipelines.models.backend_service_pb2 import *
from ficus.grpc_pipelines.models.backend_service_pb2_grpc import *
from ficus.grpc_pipelines.models.context_pb2 import *
from ficus.grpc_pipelines.models.pipelines_pb2 import *
from ficus.grpc_pipelines.models.pm_models_pb2 import *
from ficus.grpc_pipelines.models.util_pb2 import *


@dataclass
class ContextValue:
    def to_grpc_context_value(self):
        pass

@dataclass
class StringContextValue(ContextValue):
    value: str

    def to_grpc_context_value(self):
        return self.value


@dataclass
class Uint32ContextValue(ContextValue):
    value: int


    def to_grpc_context_value(self):
        return self.value

@dataclass
class BoolContextValue(ContextValue):
    value: bool

    def to_grpc_context_value(self):
        return self.value

@dataclass
class HashesLogContextValue(ContextValue):
    value: list[list[int]]

    def to_grpc_context_value(self):
        log = GrpcHashesEventLog()
        for trace in self.value:
            grpc_trace = GrpcHashesLogTrace()
            for event in trace:
                grpc_trace.events.append(event)

            log.traces.append(grpc_trace)

        return GrpcHashesEventLogContextValue(log=log)


@dataclass
class NamesLogContextValue(ContextValue):
    value: list[list[str]]


    def to_grpc_context_value(self):
        log = GrpcNamesEventLog()
        for trace in self.value:
            grpc_trace = GrpcNamesTrace()
            for event in trace:
                grpc_trace.events.append(event)

            log.traces.append(grpc_trace)

        return GrpcNamesEventLogContextValue(log=log)


class Pipeline2:
    def __init__(self, *parts, initial_config: dict[str, ContextValue]):
        self.parts = parts
        self.initial_context = initial_config


    def execute(self) -> GrpcGuid:
        with grpc.insecure_channel('localhost:8080') as channel:
            stub = GrpcBackendServiceStub(channel)
            request = GrpcPipelineExecutionRequest()
            request.pipeline = self._create_grpc_pipeline(self.parts)
            request.initialContext = self._create_initial_context(self.initial_context)
            return stub.ExecutePipeline(request)


    @staticmethod
    def _create_grpc_pipeline(*parts) -> GrpcPipeline:
        pipeline = GrpcPipeline()
        for part in parts:
            if part is not PipelinePart2:
                raise TypeError()

            pipeline.parts.append(part.to_grpc_part())

        return pipeline


    @staticmethod
    def _create_initial_context(context: dict[str, ContextValue]) -> list[GrpcContextKeyValue]:
        result = []
        for key, value in context.items():
            grpc_key_value = GrpcContextKeyValue()
            grpc_key_value.key = GrpcContextKey()
            grpc_key_value.key.key = key
            grpc_key_value.value = value.to_grpc_context_value()

            result.append(grpc_key_value)

        return result


class PipelinePart2:
    def to_grpc_part(self) -> GrpcPipelinePart:
        pass


class ReadLogFromXes2(PipelinePart2):
    def to_grpc_part(self) -> GrpcPipelinePart:
        part = _create_empty_pipeline_part()
        part.name = "ReadLogFromXes"

        return part


def _create_empty_pipeline_part():
    part = GrpcPipelinePart()
    part.configuration = GrpcPipelinePartConfiguration()
    return part