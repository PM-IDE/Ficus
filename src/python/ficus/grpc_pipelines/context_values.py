from dataclasses import dataclass

from ficus.grpc_pipelines.models.context_pb2 import *
from ficus.grpc_pipelines.models.pm_models_pb2 import *
from ficus.grpc_pipelines.models.util_pb2 import GrpcColor


@dataclass
class ContextValue:
    def to_grpc_context_value(self) -> GrpcContextValue:
        pass


@dataclass
class StringContextValue(ContextValue):
    value: str

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(string=self.value)


@dataclass
class Uint32ContextValue(ContextValue):
    value: int

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(uint32=self.value)


@dataclass
class BoolContextValue(ContextValue):
    value: bool

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(bool=self.value)


@dataclass
class HashesLogContextValue(ContextValue):
    value: list[list[int]]

    def to_grpc_context_value(self) -> GrpcContextValue:
        log = GrpcHashesEventLog()
        for trace in self.value:
            grpc_trace = GrpcHashesLogTrace()
            for event in trace:
                grpc_trace.events.append(event)

            log.traces.append(grpc_trace)

        return GrpcContextValue(hashes_log=GrpcHashesEventLogContextValue(log=log))


@dataclass
class NamesLogContextValue(ContextValue):
    value: list[list[str]]

    def to_grpc_context_value(self) -> GrpcContextValue:
        log = GrpcNamesEventLog()
        for trace in self.value:
            grpc_trace = GrpcNamesTrace()
            for event in trace:
                grpc_trace.events.append(event)

            log.traces.append(grpc_trace)

        return GrpcContextValue(names_log=GrpcNamesEventLogContextValue(log=log))


def from_grpc_names_log(grpc_names_log: GrpcNamesEventLog) -> list[list[str]]:
    result = []
    for grpc_trace in grpc_names_log.log.traces:
        trace = []
        for event in grpc_trace.events:
            trace.append(event)

        result.append(trace)

    return result


@dataclass
class Color:
    red: int
    green: int
    blue: int


def from_grpc_colors_log(grpc_colors_log: GrpcColorsEventLog) -> list[list[Color]]:
    result = []
    for grpc_trace in grpc_colors_log.traces:
        trace = []
        for event_color in grpc_trace.event_colors:
            trace.append(from_grpc_color(event_color))

        result.append(trace)

    return result


def from_grpc_color(grpc_color: GrpcColor) -> Color:
    return Color(grpc_color.red, grpc_color.green, grpc_color.blue)
