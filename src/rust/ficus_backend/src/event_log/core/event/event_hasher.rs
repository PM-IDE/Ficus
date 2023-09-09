use std::{
    collections::hash_map::DefaultHasher,
    f32::consts::E,
    hash::{Hash, Hasher},
};

use regex::{Error, Regex};

use super::event::Event;

pub trait EventHasher<TEvent>
where
    TEvent: Event,
{
    fn hash(&self, event: &TEvent) -> u64;
}

pub struct NameEventHasher;

impl NameEventHasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TEvent> EventHasher<TEvent> for NameEventHasher
where
    TEvent: Event,
{
    fn hash(&self, event: &TEvent) -> u64 {
        default_class_extractor(event)
    }
}

pub fn default_class_extractor<TEvent>(event: &TEvent) -> u64
where
    TEvent: Event,
{
    let mut hasher = DefaultHasher::new();
    event.get_name().hash(&mut hasher);

    hasher.finish()
}

pub struct RegexEventHasher {
    regex: Regex,
}

impl<TEvent> EventHasher<TEvent> for RegexEventHasher
where
    TEvent: Event,
{
    fn hash(&self, event: &TEvent) -> u64 {
        let name = event.get_name();
        match self.regex.find(name) {
            Some(m) => {
                if m.start() == 0 {
                    let mut hasher = DefaultHasher::new();
                    name[0..m.end()].hash(&mut hasher);

                    hasher.finish()
                } else {
                    default_class_extractor(event)
                }
            }
            None => default_class_extractor(event),
        }
    }
}

impl RegexEventHasher {
    pub fn new(regex: &String) -> Result<RegexEventHasher, Error> {
        match Regex::new(regex) {
            Ok(regex) => Ok(RegexEventHasher { regex }),
            Err(error) => Err(error),
        }
    }
}
