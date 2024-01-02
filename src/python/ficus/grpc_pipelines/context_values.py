from dataclasses import dataclass
from typing import Optional

from ficus.analysis.event_log_analysis import ColoredRectangle, Color
from ficus.discovery.petri_net import Arc, Transition, Place, PetriNet, Marking, SinglePlaceMarking
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
class FloatContextValue(ContextValue):
    value: float

    def to_grpc_context_value(self) -> GrpcContextValue:
        return GrpcContextValue(float=self.value)


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
        return GrpcEventLogInfo(events_count=self.events_count,
                                traces_count=self.traces_count,
                                event_classes_count=self.event_classes_count)


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


def from_grpc_petri_net(grpc_petri_net: GrpcPetriNet) -> 'PetriNet':
    petri_net = PetriNet()
    for grpc_place in grpc_petri_net.places:
        place = from_grpc_petri_net_place(grpc_place)
        petri_net.places[place.id] = place

    for grpc_transition in grpc_petri_net.transitions:
        transition = from_grpc_transition(grpc_transition)
        petri_net.transitions[transition.id] = transition

    petri_net.initial_marking = try_from_grpc_marking(grpc_petri_net.initial_marking)
    petri_net.final_marking = try_from_grpc_marking(grpc_petri_net.final_marking)

    return petri_net


def from_grpc_petri_net_place(grpc_petri_net_place: GrpcPetriNetPlace) -> 'Place':
    return Place(grpc_petri_net_place.id, grpc_petri_net_place.name)


def from_grpc_transition(grpc_petri_net_transition: GrpcPetriNetTransition) -> 'Transition':
    transition = Transition(grpc_petri_net_transition.id)
    for grpc_incoming_arc in grpc_petri_net_transition.incomingArcs:
        transition.incoming_arcs.append(from_grpc_arc(grpc_incoming_arc))

    for grpc_outgoing_arc in grpc_petri_net_transition.outgoingArcs:
        transition.outgoing_arcs.append(from_grpc_arc(grpc_outgoing_arc))

    transition.data = grpc_petri_net_transition.data
    return transition


def from_grpc_arc(grpc_arc: GrpcPetriNetArc) -> 'Arc':
    return Arc(grpc_arc.id, grpc_arc.placeId, grpc_arc.tokens_count)


def try_from_grpc_marking(grpc_marking: Optional[GrpcPetriNetMarking]) -> Optional[Marking]:
    if grpc_marking is None:
        return None

    return Marking(list(map(from_grpc_single_marking, grpc_marking.markings)))


def from_grpc_single_marking(grpc_marking: GrpcPetriNetSinglePlaceMarking) -> SinglePlaceMarking:
    return SinglePlaceMarking(grpc_marking.placeId, grpc_marking.tokensCount)


def from_grpc_count_annotation(grpc_annotation: GrpcPetriNetCountAnnotation) -> dict[int, str]:
    map = dict()
    for annotation in grpc_annotation.annotations:
        map[annotation.arcId] = str(annotation.count)

    return map


def from_grpc_frequency_annotation(grpc_annotation: GrpcPetriNetFrequenciesAnnotation) -> dict[int, str]:
    map = dict()
    for annotation in grpc_annotation.annotations:
        map[annotation.arcId] = str(annotation.frequency)

    return map
