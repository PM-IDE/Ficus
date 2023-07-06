use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Lifecycle {
    XesStandardLifecycle(XesStandardLifecycle),
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