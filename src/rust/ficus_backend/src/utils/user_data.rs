use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

pub struct Key<T> {
    name: String,
    _phantom_data: PhantomData<T>,
}

impl<T> Key<T>
where
    T: 'static,
{
    pub fn new(name: &String) -> Self {
        Self {
            name: name.to_owned(),
            _phantom_data: PhantomData,
        }
    }

    fn to_tuple(&self) -> (String, TypeId) {
        (self.name.to_owned(), self._phantom_data.type_id())
    }
}

impl<T> ValueHolder<T> for ValueHolderImpl<T> {
    fn get(&self) -> &T {
        &self.value
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

pub trait ValueHolder<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

pub struct ValueHolderImpl<T> {
    value: Box<T>,
}

impl<T> ValueHolderImpl<T> {
    pub fn new(value: Box<T>) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct UserData {
    values_map: Option<HashMap<(String, TypeId), Box<dyn Any>>>,
}

impl UserData {
    pub fn new() -> Self {
        Self { values_map: None }
    }

    pub fn put<T>(&mut self, key: &Key<T>, value: Box<T>)
    where
        T: 'static,
    {
        self.initialize_values_map();

        let values_map = self.values_map.as_mut().unwrap();
        values_map.insert(key.to_tuple(), Box::new(ValueHolderImpl::new(value)));
    }

    fn initialize_values_map(&mut self) {
        if self.values_map.is_some() {
            return;
        }

        self.values_map = Some(HashMap::new());
    }

    pub fn remove<T>(&mut self, key: &Key<T>)
    where
        T: 'static,
    {
        if self.values_map.is_none() {
            return;
        }

        self.values_map.as_mut().unwrap().remove(&key.to_tuple());
    }

    pub fn get<T: 'static>(&self, key: &Key<T>) -> Option<&T> {
        if self.values_map.is_none() {
            return None;
        }

        let values_map = self.values_map.as_ref().unwrap();
        if let Some(value) = values_map.get(&key.to_tuple()) {
            Some(value.as_ref().downcast_ref::<ValueHolderImpl<T>>().unwrap().get())
        } else {
            None
        }
    }

    pub fn get_mut<T: 'static>(&mut self, key: &Key<T>) -> Option<&mut T> {
        if self.values_map.is_none() {
            return None;
        }

        let value_map = self.values_map.as_mut().unwrap();
        if let Some(value) = value_map.get_mut(&key.to_tuple()) {
            Some(value.as_mut().downcast_mut::<ValueHolderImpl<T>>().unwrap().get_mut())
        } else {
            None
        }
    }
}
