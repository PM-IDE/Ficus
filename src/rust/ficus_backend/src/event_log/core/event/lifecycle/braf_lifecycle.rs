use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
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

const UNSPECIFIED: &'static str = "Unspecified";
const CLOSED: &'static str = "Closed";
const CLOSED_CANCELLED: &'static str = "Closed.Cancelled";
const CLOSED_CANCELLED_ABORTED: &'static str = "Closed.Cancelled.Aborted";
const CLOSED_CANCELLED_ERROR: &'static str = "Closed.Cancelled.Error";
const CLOSED_CANCELLED_EXITED: &'static str = "Closed.Cancelled.Exited";
const CLOSED_CANCELLED_OBSOLETE: &'static str = "Closed.Cancelled.Obsolete";
const CLOSED_CANCELLED_TERMINATED: &'static str = "Closed.Cancelled.Terminated";
const COMPLETED: &'static str = "Completed";
const COMPLETED_FAILED: &'static str = "Completed.Failed";
const COMPLETED_SUCCESS: &'static str = "Completed.Success";
const OPEN: &'static str = "Open";
const OPEN_NOTRUNNING: &'static str = "Open.NotRunning";
const OPEN_NOTRUNNING_ASSIGNED: &'static str = "Open.NotRunning.Assigned";
const OPEN_NOTRUNNING_RESERVED: &'static str = "Open.NotRunning.Reserved";
const OPEN_NOTRUNNING_SUSPENDED_ASSIGNED: &'static str = "Open.NotRunning.Suspended.Assigned";
const OPEN_NOTRUNNING_SUSPENDED_RESERVED: &'static str = "Open.NotRunning.Suspended.Reserved";
const OPEN_RUNNING: &'static str = "Open.Running";
const OPEN_RUNNING_INPROGRESS: &'static str = "Open.Running.InProgress";
const OPEN_RUNNING_SUSPENDED: &'static str = "Open.Running.Suspended";

impl ToString for XesBrafLifecycle {
    fn to_string(&self) -> String {
        match self {
            XesBrafLifecycle::Unspecified => String::from_str(UNSPECIFIED).ok().unwrap(),
            XesBrafLifecycle::Closed => String::from_str(CLOSED).ok().unwrap(),
            XesBrafLifecycle::ClosedCancelled => String::from_str(CLOSED_CANCELLED).ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledAborted => String::from_str(CLOSED_CANCELLED_ABORTED).ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledError => String::from_str(CLOSED_CANCELLED_ERROR).ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledExited => String::from_str(CLOSED_CANCELLED_EXITED).ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledObsolete => String::from_str(CLOSED_CANCELLED_OBSOLETE).ok().unwrap(),
            XesBrafLifecycle::ClosedCancelledTerminated => String::from_str(CLOSED_CANCELLED_TERMINATED).ok().unwrap(),
            XesBrafLifecycle::Completed => String::from_str(COMPLETED).ok().unwrap(),
            XesBrafLifecycle::CompletedFailed => String::from_str(COMPLETED_FAILED).ok().unwrap(),
            XesBrafLifecycle::CompletedSuccess => String::from_str(COMPLETED_SUCCESS).ok().unwrap(),
            XesBrafLifecycle::Open => String::from_str(OPEN).ok().unwrap(),
            XesBrafLifecycle::OpenNotRunning => String::from_str(OPEN_NOTRUNNING).ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningAssigned => String::from_str(OPEN_NOTRUNNING_ASSIGNED).ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningReserved => String::from_str(OPEN_NOTRUNNING_RESERVED).ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningSuspendedAssigned => String::from_str(OPEN_NOTRUNNING_SUSPENDED_ASSIGNED).ok().unwrap(),
            XesBrafLifecycle::OpenNotRunningSuspendedReserved => String::from_str(OPEN_NOTRUNNING_SUSPENDED_RESERVED).ok().unwrap(),
            XesBrafLifecycle::OpenRunning => String::from_str(OPEN_RUNNING).ok().unwrap(),
            XesBrafLifecycle::OpenRunningInProgress => String::from_str(OPEN_RUNNING_INPROGRESS).ok().unwrap(),
            XesBrafLifecycle::OpenRunningSuspended => String::from_str(OPEN_RUNNING_SUSPENDED).ok().unwrap(),
        }
    }
}

impl FromStr for XesBrafLifecycle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            UNSPECIFIED => Ok(XesBrafLifecycle::Unspecified),
            CLOSED => Ok(XesBrafLifecycle::Closed),
            CLOSED_CANCELLED => Ok(XesBrafLifecycle::ClosedCancelled),
            CLOSED_CANCELLED_ABORTED => Ok(XesBrafLifecycle::ClosedCancelledAborted),
            CLOSED_CANCELLED_ERROR => Ok(XesBrafLifecycle::ClosedCancelledError),
            CLOSED_CANCELLED_EXITED => Ok(XesBrafLifecycle::ClosedCancelledExited),
            CLOSED_CANCELLED_OBSOLETE => Ok(XesBrafLifecycle::ClosedCancelledObsolete),
            CLOSED_CANCELLED_TERMINATED => Ok(XesBrafLifecycle::ClosedCancelledTerminated),
            COMPLETED => Ok(XesBrafLifecycle::Completed),
            COMPLETED_FAILED => Ok(XesBrafLifecycle::CompletedFailed),
            COMPLETED_SUCCESS => Ok(XesBrafLifecycle::CompletedSuccess),
            OPEN => Ok(XesBrafLifecycle::Open),
            OPEN_NOTRUNNING => Ok(XesBrafLifecycle::OpenNotRunning),
            OPEN_NOTRUNNING_ASSIGNED => Ok(XesBrafLifecycle::OpenNotRunningAssigned),
            OPEN_NOTRUNNING_RESERVED => Ok(XesBrafLifecycle::OpenNotRunningReserved),
            OPEN_NOTRUNNING_SUSPENDED_ASSIGNED => Ok(XesBrafLifecycle::OpenNotRunningSuspendedAssigned),
            OPEN_NOTRUNNING_SUSPENDED_RESERVED => Ok(XesBrafLifecycle::OpenNotRunningSuspendedReserved),
            OPEN_RUNNING => Ok(XesBrafLifecycle::OpenRunning),
            OPEN_RUNNING_INPROGRESS => Ok(XesBrafLifecycle::OpenRunningInProgress),
            OPEN_RUNNING_SUSPENDED => Ok(XesBrafLifecycle::OpenRunningSuspended),
            _ => Err(()),
        }
    }
}
