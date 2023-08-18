use crate::{
    pipelines::context::PipelineContext,
    utils::user_data::{
        keys::{DefaultKey, Key},
        user_data::UserData,
    },
};
use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use super::context_keys::ContextKeys;

type ContextKeyValueFactory<T> = Rc<Box<dyn Fn(&PipelineContext, &ContextKeys) -> Option<T>>>;

pub trait ContextKey {
    fn key(&self) -> &dyn Key;
    fn try_create_value_into_context(&self, context: &mut PipelineContext, keys: &ContextKeys);
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

    fn try_create_value_into_context(&self, context: &mut PipelineContext, keys: &ContextKeys) {
        if context.get_concrete(&self.key).is_some() {
            return;
        }

        if let Some(factory) = self.factory.as_ref() {
            context.put_concrete(&self.key, factory(context, keys).unwrap())
        }
    }
}

impl<T> Clone for DefaultContextKey<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            factory: self.factory.clone(),
        }
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
