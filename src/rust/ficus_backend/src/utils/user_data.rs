use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData};

pub struct Key<T> {
    name: String,
    _phantom_data: PhantomData<T>
}

impl<T> Key<T> where T: 'static {
    pub fn new(name: &String) -> Self {
        Self { name: name.to_owned(), _phantom_data: PhantomData }
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
    value: Box<T>
}

impl<T> ValueHolderImpl<T> {
    pub fn new(value: Box<T>) -> Self {
        Self { value }
    }
}

pub struct UserData {
    data: HashMap<(String, TypeId), Box<dyn Any>>
}

impl UserData {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn put<T>(&mut self, key: &Key<T>, value: Box<T>) where T: 'static {
        self.data.insert(key.to_tuple(), Box::new(ValueHolderImpl::new(value)));
    }

    pub fn get<T: 'static>(&self, key: &Key<T>) -> Option<&T> {
        if let Some(value) = self.data.get(&key.to_tuple()) {
            Some(value.as_ref().downcast_ref::<ValueHolderImpl<T>>().unwrap().get())
        } else {
            None
        }
    }

    pub fn get_mut<T: 'static>(&mut self, key: &Key<T>) -> Option<&mut T> {
        if let Some(value) = self.data.get_mut(&key.to_tuple()) {
            Some(value.as_mut().downcast_mut::<ValueHolderImpl<T>>().unwrap().get_mut())
        } else {
            None
        }
    }
}
