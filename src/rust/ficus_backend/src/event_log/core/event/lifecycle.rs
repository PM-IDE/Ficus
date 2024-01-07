use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Lifecycle {
    XesStandardLifecycle(XesStandardLifecycle),
    BrafLifecycle(XesBrafLifecycle),
}

impl ToString for Lifecycle {
    fn to_string(&self) -> String {
        match self {
            Self::XesStandardLifecycle(xes_lifecycle) => xes_lifecycle.to_string(),
            Self::BrafLifecycle(braf_lifecycle) => braf_lifecycle.to_string(),
        }
    }
}

impl ToString for XesStandardLifecycle {
    fn to_string(&self) -> String {
        match self {
            Self::Schedule => String::from_str("schedule").ok().unwrap(),
            Self::Start => String::from_str("start").ok().unwrap(),
            Self::Complete => String::from_str("complete").ok().unwrap(),
            Self::Unknown => String::from_str("unknown").ok().unwrap(),
            Self::Unspecified => String::from_str("unspecified").ok().unwrap(),
            Self::Assign => String::from_str("assign").ok().unwrap(),
            Self::AteAbort => String::from_str("ate_abort").ok().unwrap(),
            Self::Autoskip => String::from_str("autoskip").ok().unwrap(),
            Self::ManualSkip => String::from_str("manualskip").ok().unwrap(),
            Self::PiAbort => String::from_str("pi_abort").ok().unwrap(),
            Self::ReAssign => String::from_str("reassign").ok().unwrap(),
            Self::Resume => String::from_str("resume").ok().unwrap(),
            Self::Suspend => String::from_str("suspend").ok().unwrap(),
            Self::Withdraw => String::from_str("withdraw").ok().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum XesStandardLifecycle {
    Unspecified = 0,
    Assign = 1,
    AteAbort = 2,
    Autoskip = 3,
    Complete = 4,
    ManualSkip = 5,
    PiAbort = 6,
    ReAssign = 7,
    Resume = 8,
    Schedule = 9,
    Start = 10,
    Suspend = 11,
    Unknown = 12,
    Withdraw = 13,
}

impl FromStr for XesStandardLifecycle {
    type Err = ParseXesStandardLifecycleError;

    fn from_str(s: &str) -> Result<XesStandardLifecycle, Self::Err> {
        match s {
            "schedule" => Ok(XesStandardLifecycle::Schedule),
            "start" => Ok(XesStandardLifecycle::Start),
            "complete" => Ok(XesStandardLifecycle::Complete),
            "unknown" => Ok(XesStandardLifecycle::Unknown),
            _ => Err(ParseXesStandardLifecycleError),
        }
    }
}

pub struct ParseXesStandardLifecycleError;

#[derive(Debug, Clone, Copy)]
pub enum XesBrafLifecycle {
    Unspecified = 0,
    Closed = 1,
    ClosedCancelled = 2,
    ClosedCancelledAborted = 3,
    ClosedCancelledError = 4,
    ClosedCancelledExited = 5,
    ClosedCancelledObsolete = 6,
    ClosedCancelledTerminated = 7,
    Completed = 8,
    CompletedFailed = 9,
    CompletedSuccess = 10,
    Open = 11,
    OpenNotRunning = 12,
    OpenNotRunningAssigned = 13,
    OpenNotRunningReserved = 14,
    OpenNotRunningSuspendedAssigned = 15,
    OpenNotRunningSuspendedReserved = 16,
    OpenRunning = 17,
    OpenRunningInProgress = 18,
    OpenRunningSuspended = 19,
}

impl ToString for XesBrafLifecycle {
    fn to_string(&self) -> String {
        match self {
            XesBrafLifecycle::Unspecified => String::from_str("Unspecified").ok().unwrap(),
            XesBrafLifecycle::Closed => String::from_str("Closed").ok().unwrap(),
            XesBrafLifecycle::ClosedCancelled => String::from_str("Closed.Cancelled").ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledAborted => String::from_str("Closed.Cancelled.Aborted").ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledError => String::from_str("Closed.Cancelled.Error").ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledExited => String::from_str("Closed.Cancelled.Exited").ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledObsolete => String::from_str("Closed.Cancelled.Obsolete").ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledTerminated => String::from_str("Closed.Cancelled.Terminated").ok().unwrap(),
            XesBrafLifecycle::Completed => String::from_str("Completed").ok().unwrap(),
            XesBrafLifecycle::CompletedFailed => String::from_str("Completed.Failed").ok().unwrap(),
            XesBrafLifecycle::CompletedSuccess => String::from_str("Completed.Success").ok().unwrap(),
            XesBrafLifecycle::Open => String::from_str("Open").ok().unwrap(),
            XesBrafLifecycle::OpenNotRunning => String::from_str("Open.NotRunning").ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningAssigned => String::from_str("Open.NotRunning.Assigned").ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningReserved => String::from_str("Open.NotRunning.Reserved").ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningSuspendedAssigned => String::from_str("Open.NotRunning.Suspended.Assigned").ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningSuspendedReserved => String::from_str("Open.NotRunning.Suspended.Reserved").ok().unwrap(),
            XesBrafLifecycle::OpenRunning => String::from_str("Open.Running").ok().unwrap(),
            XesBrafLifecycle::OpenRunningInProgress => String::from_str("Open.Running.InProgress").ok().unwrap(),
            XesBrafLifecycle::OpenRunningSuspended => String::from_str("Open.Running.Suspended").ok().unwrap(),
        }
    }
}
