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

impl FromStr for XesBrafLifecycle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Unspecified" => Ok(XesBrafLifecycle::Unspecified),
            "Closed" => Ok(XesBrafLifecycle::Closed),
            "Closed.Cancelled" => Ok(XesBrafLifecycle::ClosedCancelled),
            "Closed.Cancelled.Aborted" => Ok(XesBrafLifecycle::ClosedCancelledAborted),
            "Closed.Cancelled.Error" => Ok(XesBrafLifecycle::ClosedCancelledError),
            "Closed.Cancelled.Exited" => Ok(XesBrafLifecycle::ClosedCancelledExited),
            "Closed.Cancelled.Obsolete" => Ok(XesBrafLifecycle::ClosedCancelledObsolete),
            "Closed.Cancelled.Terminated" => Ok(XesBrafLifecycle::ClosedCancelledTerminated),
            "Completed" => Ok(XesBrafLifecycle::Completed),
            "Completed.Failed" => Ok(XesBrafLifecycle::CompletedFailed),
            "Completed.Success" => Ok(XesBrafLifecycle::CompletedSuccess),
            "Open" => Ok(XesBrafLifecycle::Open),
            "Open.NotRunning" => Ok(XesBrafLifecycle::OpenNotRunning),
            "Open.NotRunning.Assigned" => Ok(XesBrafLifecycle::OpenNotRunningAssigned),
            "Open.NotRunning.Reserved" => Ok(XesBrafLifecycle::OpenNotRunningReserved),
            "Open.NotRunning.Suspended.Assigned" => Ok(XesBrafLifecycle::OpenNotRunningSuspendedAssigned),
            "Open.NotRunning.Suspended.Reserved" => Ok(XesBrafLifecycle::OpenNotRunningSuspendedReserved),
            "Open.Running" => Ok(XesBrafLifecycle::OpenRunning),
            "Open.Running.InProgress" => Ok(XesBrafLifecycle::OpenRunningInProgress),
            "Open.Running.Suspended" => Ok(XesBrafLifecycle::OpenRunningSuspended),
            _ => Err(()),
        }
    }
}
