use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Lifecycle {
    XesStandardLifecycle(XesStandardLifecycle),
}

impl ToString for Lifecycle {
    fn to_string(&self) -> String {
        match self {
            Self::XesStandardLifecycle(xes_lifecycle) => xes_lifecycle.to_string(),
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum XesStandardLifecycle {
    Schedule,
    Start,
    Complete,
    Unknown,
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
