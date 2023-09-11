from dataclasses import dataclass

from ficus.analysis.event_log_analysis import ColoredRectangle, Color
from ficus.grpc_pipelines.models.pipelines_and_context_pb2 import *
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


@dataclass
class EnumContextValue(ContextValue):
    enum_name: str
    value: str

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(enum=GrpcEnum(enumType=self.enum_name, value=self.value))


@dataclass
class StringsContextValue(ContextValue):
    strings: list[str]


    def to_grpc_context_value(self) -> GrpcContextValue:
        strings = GrpcStrings()
        strings.strings.extend(self.strings)
        return GrpcContextValue(strings=strings)

def from_grpc_names_log(grpc_names_log: GrpcNamesEventLog) -> list[list[str]]:
    result = []
    for grpc_trace in grpc_names_log.log.traces:
        trace = []
        for event in grpc_trace.events:
            trace.append(event)

        result.append(trace)

    return result

@dataclass
class EventLogInfo(ContextValue):
    events_count: int
    event_classes_count: int
    traces_count: int

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcEventLogInfo(events_count = self.events_count,
                                traces_count = self.traces_count,
                                event_classes_count = self.event_classes_count)


def from_grpc_colors_log(grpc_colors_log: GrpcColorsEventLog) -> list[list[ColoredRectangle]]:
    result = []
    for grpc_trace in grpc_colors_log.traces:
        trace = []
        for colored_rectangle in grpc_trace.event_colors:
            trace.append(from_grpc_colored_rectangle(colored_rectangle))

        result.append(trace)

    return result


def from_grpc_colored_rectangle(grpc_color: GrpcColoredRectangle) -> ColoredRectangle:
    color = from_grpc_color(grpc_color.color)
    return ColoredRectangle(color, grpc_color.start_index, grpc_color.length, grpc_color.name)


def from_grpc_color(grpc_color: GrpcColor):
    return Color(grpc_color.red, grpc_color.green, grpc_color.blue)


def from_grpc_event_log_info(grpc_event_log_info: GrpcEventLogInfo) -> EventLogInfo:
    return EventLogInfo(events_count=grpc_event_log_info.events_count,
                        traces_count=grpc_event_log_info.traces_count,
                        event_classes_count=grpc_event_log_info.event_classes_count)
