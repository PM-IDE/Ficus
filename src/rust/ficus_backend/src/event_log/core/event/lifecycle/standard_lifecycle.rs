use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
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

const SCHEDULE: &'static str = "schedule";
const START: &'static str = "start";
const COMPLETE: &'static str = "complete";
const UNKNOWN: &'static str = "unknown";
const UNSPECIFIED: &'static str = "unspecified";
const ASSIGN: &'static str = "assign";
const ATE_ABORT: &'static str = "ate_abort";
const AUTOSKIP: &'static str = "autoskip";
const MANUAL_SKIP: &'static str = "manualskip";
const PI_ABORT: &'static str = "pi_abort";
const RE_ASSIGN: &'static str = "reassign";
const RESUME: &'static str = "resume";
const SUSPEND: &'static str = "suspend";
const WITHDRAW: &'static str = "withdraw";

impl ToString for XesStandardLifecycle {
    fn to_string(&self) -> String {
        match self {
            Self::Schedule => String::from_str(SCHEDULE).ok().unwrap(),
            Self::Start => String::from_str(START).ok().unwrap(),
            Self::Complete => String::from_str(COMPLETE).ok().unwrap(),
            Self::Unknown => String::from_str(UNKNOWN).ok().unwrap(),
            Self::Unspecified => String::from_str(UNSPECIFIED).ok().unwrap(),
            Self::Assign => String::from_str(ASSIGN).ok().unwrap(),
            Self::AteAbort => String::from_str(ATE_ABORT).ok().unwrap(),
            Self::Autoskip => String::from_str(AUTOSKIP).ok().unwrap(),
            Self::ManualSkip => String::from_str(MANUAL_SKIP).ok().unwrap(),
            Self::PiAbort => String::from_str(PI_ABORT).ok().unwrap(),
            Self::ReAssign => String::from_str(RE_ASSIGN).ok().unwrap(),
            Self::Resume => String::from_str(RESUME).ok().unwrap(),
            Self::Suspend => String::from_str(SUSPEND).ok().unwrap(),
            Self::Withdraw => String::from_str(WITHDRAW).ok().unwrap(),
        }
    }
}

impl FromStr for XesStandardLifecycle {
    type Err = ParseXesStandardLifecycleError;

    fn from_str(s: &str) -> Result<XesStandardLifecycle, Self::Err> {
        match s {
            SCHEDULE => Ok(XesStandardLifecycle::Schedule),
            START => Ok(XesStandardLifecycle::Start),
            COMPLETE => Ok(XesStandardLifecycle::Complete),
            UNKNOWN => Ok(XesStandardLifecycle::Unknown),
            UNSPECIFIED => Ok(XesStandardLifecycle::Unspecified),
            ASSIGN => Ok(XesStandardLifecycle::Assign),
            ATE_ABORT => Ok(XesStandardLifecycle::AteAbort),
            AUTOSKIP => Ok(XesStandardLifecycle::Autoskip),
            MANUAL_SKIP => Ok(XesStandardLifecycle::ManualSkip),
            PI_ABORT => Ok(XesStandardLifecycle::PiAbort),
            RE_ASSIGN => Ok(XesStandardLifecycle::ReAssign),
            SUSPEND => Ok(XesStandardLifecycle::Suspend),
            RESUME => Ok(XesStandardLifecycle::Resume),
            WITHDRAW => Ok(XesStandardLifecycle::Withdraw),
            _ => Err(ParseXesStandardLifecycleError),
        }
    }
}

pub struct ParseXesStandardLifecycleError;
