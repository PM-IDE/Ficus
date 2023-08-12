use crate::{
    pipelines::context::PipelineContext,
    utils::user_data::{DefaultKey, Key},
};
use std::hash::{Hash, Hasher};

use super::context_keys::ContextKeys;

type ContextKeyValueFactory<T> = Box<dyn Fn(&PipelineContext, &ContextKeys) -> Option<T>>;

pub trait ContextKey {
    fn key(&self) -> &dyn Key;
}

pub struct DefaultContextKey<T>
where
    T: 'static,
{
    key: DefaultKey<T>,
    factory: Option<ContextKeyValueFactory<T>>,
}

impl<T> ContextKey for DefaultContextKey<T> {
    fn key(&self) -> &dyn Key {
        &self.key
    }
}

impl<T> DefaultContextKey<T>
where
    T: 'static,
{
    pub fn new(name: &str) -> Self {
        Self {
            key: DefaultKey::new(name.to_owned()),
            factory: None,
        }
    }

    pub fn new_with_factory(name: String, factory: ContextKeyValueFactory<T>) -> Self {
        Self {
            key: DefaultKey::new(name),
            factory: Some(factory),
        }
    }

    pub fn key(&self) -> &DefaultKey<T> {
        &self.key
    }

    pub fn factory(&self) -> Option<&ContextKeyValueFactory<T>> {
        self.factory.as_ref()
    }
}

impl<T> PartialEq for DefaultContextKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for DefaultContextKey<T> {}

impl<T> Hash for DefaultContextKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
