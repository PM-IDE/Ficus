use super::pipelines::PipelineParts;

impl PipelineParts {
    pub const READ_LOG_FROM_XES: &str = "ReadLogFromXes";
    pub const WRITE_LOG_TO_XES: &str = "WriteLogToXes";
    pub const FIND_PRIMITIVE_TANDEM_ARRAYS: &str = "FindPrimitiveTandemArrays";
    pub const FIND_MAXIMAL_TANDEM_ARRAYS: &str = "FindMaximalTandemArrays";
    pub const FIND_MAXIMAL_REPEATS: &str = "FindMaximalRepeats";
    pub const FIND_SUPER_MAXIMAL_REPEATS: &str = "FindSuperMaximalRepeats";
    pub const FIND_NEAR_SUPER_MAXIMAL_REPEATS: &str = "FindNearSuperMaximalRepeats";
    pub const DISCOVER_ACTIVITIES: &str = "DiscoverActivities";
    pub const DISCOVER_ACTIVITIES_INSTANCES: &str = "DiscoverActivitiesInstances";
    pub const CREATE_LOG_FROM_ACTIVITIES: &str = "CreateLogFromActivities";
    pub const FILTER_EVENTS_BY_NAME: &str = "FilterEventsByName";
    pub const FILTER_EVENTS_BY_REGEX: &str = "FilterEventsByRegex";
    pub const FILTER_LOG_BY_VARIANTS: &str = "FilterLogByVariants";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_NAME: &str = "DrawPlacementOfEventByName";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_REGEX: &str = "DrawPlacementOfEventsByRegex";
    pub const DRAW_FULL_ACTIVITIES_DIAGRAM: &str = "DrawFullActivitiesDiagram";
    pub const DRAW_SHORT_ACTIVITIES_DIAGRAM: &str = "DrawShortActivitiesDiagram";
    pub const GET_EVENT_LOG_INFO: &str = "GetEventLogInfo";
    pub const CLEAR_ACTIVITIES: &str = "ClearActivities";
    pub const GET_UNDERLYING_EVENTS_COUNT: &str = "GetUnderlyingEventsCount";
    pub const FILTER_TRACES_BY_EVENTS_COUNT: &str = "FilterTracesByEventsCount";
    pub const TRACES_DIVERSITY_DIAGRAM: &str = "TracesDiversityDiagram";
    pub const GET_NAMES_EVENT_LOG: &str = "GetNamesEventLog";
    pub const GET_HASHES_EVENT_LOG: &str = "GetHashesEventLog";
    pub const USE_NAMES_EVENT_LOG: &str = "UseNamesEventLog";
    pub const DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL: &str = "DiscoverActivitiesForSeveralLevels";
    pub const DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES: &str = "DiscoverActivitiesInUnattachedSubTraces";
    pub const DISCOVER_ACTIVITIES_FROM_PATTERNS: &str = "DiscoverActivitiesFromPatterns";
}
