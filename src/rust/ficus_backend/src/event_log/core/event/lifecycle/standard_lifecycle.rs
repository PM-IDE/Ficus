use std::str::FromStr;

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

impl FromStr for XesStandardLifecycle {
    type Err = ParseXesStandardLifecycleError;

    fn from_str(s: &str) -> Result<XesStandardLifecycle, Self::Err> {
        match s {
            "schedule" => Ok(XesStandardLifecycle::Schedule),
            "start" => Ok(XesStandardLifecycle::Start),
            "complete" => Ok(XesStandardLifecycle::Complete),
            "unknown" => Ok(XesStandardLifecycle::Unknown),
            "unspecified" => Ok(XesStandardLifecycle::Unspecified),
            "assign" => Ok(XesStandardLifecycle::Assign),
            "ate_abort" => Ok(XesStandardLifecycle::AteAbort),
            "autoskip" => Ok(XesStandardLifecycle::Autoskip),
            "manualskip" => Ok(XesStandardLifecycle::ManualSkip),
            "pi_abort" => Ok(XesStandardLifecycle::PiAbort),
            "reassign" => Ok(XesStandardLifecycle::ReAssign),
            "suspend" => Ok(XesStandardLifecycle::Suspend),
            "resume" => Ok(XesStandardLifecycle::Resume),
            "withdraw" => Ok(XesStandardLifecycle::Withdraw),
            _ => Err(ParseXesStandardLifecycleError),
        }
    }
}

pub struct ParseXesStandardLifecycleError;
