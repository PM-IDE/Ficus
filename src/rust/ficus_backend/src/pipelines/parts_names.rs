use super::pipelines::PipelineParts;

impl PipelineParts {
    pub const READ_LOG_FROM_XES: &'static str = "ReadLogFromXes";
    pub const WRITE_LOG_TO_XES: &'static str = "WriteLogToXes";
    pub const FIND_PRIMITIVE_TANDEM_ARRAYS: &'static str = "FindPrimitiveTandemArrays";
    pub const FIND_MAXIMAL_TANDEM_ARRAYS: &'static str = "FindMaximalTandemArrays";
    pub const FIND_MAXIMAL_REPEATS: &'static str = "FindMaximalRepeats";
    pub const FIND_SUPER_MAXIMAL_REPEATS: &'static str = "FindSuperMaximalRepeats";
    pub const FIND_NEAR_SUPER_MAXIMAL_REPEATS: &'static str = "FindNearSuperMaximalRepeats";
    pub const DISCOVER_ACTIVITIES: &'static str = "DiscoverActivities";
    pub const DISCOVER_ACTIVITIES_INSTANCES: &'static str = "DiscoverActivitiesInstances";
    pub const CREATE_LOG_FROM_ACTIVITIES: &'static str = "CreateLogFromActivities";
    pub const FILTER_EVENTS_BY_NAME: &'static str = "FilterEventsByName";
    pub const FILTER_EVENTS_BY_REGEX: &'static str = "FilterEventsByRegex";
    pub const FILTER_LOG_BY_VARIANTS: &'static str = "FilterLogByVariants";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_NAME: &'static str = "DrawPlacementOfEventByName";
    pub const DRAW_PLACEMENT_OF_EVENT_BY_REGEX: &'static str = "DrawPlacementOfEventsByRegex";
    pub const DRAW_FULL_ACTIVITIES_DIAGRAM: &'static str = "DrawFullActivitiesDiagram";
    pub const DRAW_SHORT_ACTIVITIES_DIAGRAM: &'static str = "DrawShortActivitiesDiagram";
    pub const GET_EVENT_LOG_INFO: &'static str = "GetEventLogInfo";
    pub const CLEAR_ACTIVITIES: &'static str = "ClearActivities";
    pub const GET_UNDERLYING_EVENTS_COUNT: &'static str = "GetUnderlyingEventsCount";
    pub const FILTER_TRACES_BY_EVENTS_COUNT: &'static str = "FilterTracesByEventsCount";
    pub const TRACES_DIVERSITY_DIAGRAM: &'static str = "TracesDiversityDiagram";
    pub const GET_NAMES_EVENT_LOG: &'static str = "GetNamesEventLog";
    pub const GET_HASHES_EVENT_LOG: &'static str = "GetHashesEventLog";
    pub const USE_NAMES_EVENT_LOG: &'static str = "UseNamesEventLog";
    pub const DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL: &'static str = "DiscoverActivitiesForSeveralLevels";
    pub const DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES: &'static str = "DiscoverActivitiesInUnattachedSubTraces";
    pub const DISCOVER_ACTIVITIES_FROM_PATTERNS: &'static str = "DiscoverActivitiesFromPatterns";
    pub const DISCOVER_ACTIVITIES_UNTIL_NO_MORE: &'static str = "DiscoverActivitiesUntilNoMore";
}
