from enum import Enum


class PatternsDiscoveryStrategy(Enum):
    FromAllTraces = 0
    FromSingleMergedTrace = 1


class PatternsKind(Enum):
    PrimitiveTandemArrays = 0,
    MaximalTandemArrays = 1,
    MaximalRepeats = 2,
    SuperMaximalRepeats = 3,
    NearSuperMaximalRepeats = 4,


class NarrowActivityKind(Enum):
    DontNarrow = 0,
    StayTheSame = 1,
    NarrowUp = 2,
    NarrowDown = 3,


class ActivityFilterKind(Enum):
    NoFilter = 0,
    DefaultFilter = 1,