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
