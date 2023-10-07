from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GrpcSimpleEventLog(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcSimpleTrace]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcSimpleTrace, _Mapping]]] = ...) -> None: ...

class GrpcSimpleTrace(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedCompositeFieldContainer[GrpcEvent]
    def __init__(self, events: _Optional[_Iterable[_Union[GrpcEvent, _Mapping]]] = ...) -> None: ...

class GrpcEvent(_message.Message):
    __slots__ = ["name", "stamp"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    STAMP_FIELD_NUMBER: _ClassVar[int]
    name: str
    stamp: GrpcEventStamp
    def __init__(self, name: _Optional[str] = ..., stamp: _Optional[_Union[GrpcEventStamp, _Mapping]] = ...) -> None: ...

class GrpcEventStamp(_message.Message):
    __slots__ = ["date", "order"]
    DATE_FIELD_NUMBER: _ClassVar[int]
    ORDER_FIELD_NUMBER: _ClassVar[int]
    date: _timestamp_pb2.Timestamp
    order: int
    def __init__(self, date: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., order: _Optional[int] = ...) -> None: ...

class GrpcHashesEventLog(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcHashesLogTrace]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcHashesLogTrace, _Mapping]]] = ...) -> None: ...

class GrpcHashesLogTrace(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, events: _Optional[_Iterable[int]] = ...) -> None: ...

class GrpcNamesEventLog(_message.Message):
    __slots__ = ["traces"]
    TRACES_FIELD_NUMBER: _ClassVar[int]
    traces: _containers.RepeatedCompositeFieldContainer[GrpcNamesTrace]
    def __init__(self, traces: _Optional[_Iterable[_Union[GrpcNamesTrace, _Mapping]]] = ...) -> None: ...

class GrpcNamesTrace(_message.Message):
    __slots__ = ["events"]
    EVENTS_FIELD_NUMBER: _ClassVar[int]
    events: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, events: _Optional[_Iterable[str]] = ...) -> None: ...

class GrpcPetriNet(_message.Message):
    __slots__ = ["places", "transitions", "marking"]
    PLACES_FIELD_NUMBER: _ClassVar[int]
    TRANSITIONS_FIELD_NUMBER: _ClassVar[int]
    MARKING_FIELD_NUMBER: _ClassVar[int]
    places: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetPlace]
    transitions: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetTransition]
    marking: GrpcPetriNetMarking
    def __init__(self, places: _Optional[_Iterable[_Union[GrpcPetriNetPlace, _Mapping]]] = ..., transitions: _Optional[_Iterable[_Union[GrpcPetriNetTransition, _Mapping]]] = ..., marking: _Optional[_Union[GrpcPetriNetMarking, _Mapping]] = ...) -> None: ...

class GrpcPetriNetPlace(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: int
    def __init__(self, id: _Optional[int] = ...) -> None: ...

class GrpcPetriNetTransition(_message.Message):
    __slots__ = ["id", "incomingArcs", "outgoingArcs", "data"]
    ID_FIELD_NUMBER: _ClassVar[int]
    INCOMINGARCS_FIELD_NUMBER: _ClassVar[int]
    OUTGOINGARCS_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    id: int
    incomingArcs: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetArc]
    outgoingArcs: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetArc]
    data: str
    def __init__(self, id: _Optional[int] = ..., incomingArcs: _Optional[_Iterable[_Union[GrpcPetriNetArc, _Mapping]]] = ..., outgoingArcs: _Optional[_Iterable[_Union[GrpcPetriNetArc, _Mapping]]] = ..., data: _Optional[str] = ...) -> None: ...

class GrpcPetriNetArc(_message.Message):
    __slots__ = ["placeId"]
    PLACEID_FIELD_NUMBER: _ClassVar[int]
    placeId: int
    def __init__(self, placeId: _Optional[int] = ...) -> None: ...

class GrpcPetriNetMarking(_message.Message):
    __slots__ = ["Markings"]
    MARKINGS_FIELD_NUMBER: _ClassVar[int]
    Markings: _containers.RepeatedCompositeFieldContainer[GrpcPetriNetSinglePlaceMarking]
    def __init__(self, Markings: _Optional[_Iterable[_Union[GrpcPetriNetSinglePlaceMarking, _Mapping]]] = ...) -> None: ...

class GrpcPetriNetSinglePlaceMarking(_message.Message):
    __slots__ = ["placeId", "tokensCount"]
    PLACEID_FIELD_NUMBER: _ClassVar[int]
    TOKENSCOUNT_FIELD_NUMBER: _ClassVar[int]
    placeId: int
    tokensCount: int
    def __init__(self, placeId: _Optional[int] = ..., tokensCount: _Optional[int] = ...) -> None: ...
