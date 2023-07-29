use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use super::event::Event;

pub trait EventHasher<TEvent>
where
    TEvent: Event,
{
    fn hash(event: &TEvent) -> u64;
}

pub struct NameEventHasher;

impl<TEvent> EventHasher<TEvent> for NameEventHasher
where
    TEvent: Event,
{
    fn hash(event: &TEvent) -> u64 {
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
