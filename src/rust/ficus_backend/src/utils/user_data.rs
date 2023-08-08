use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::{Hash, Hasher},
    marker::PhantomData,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

pub struct Key<T> {
    name: String,
    _phantom_data: PhantomData<T>,
    _hash: u64,
}

impl<T> Key<T>
where
    T: 'static,
{
    pub fn new(name: String) -> Self {
        static CURRENT_HASH: AtomicU64 = AtomicU64::new(0);

        Self {
            name: name.to_owned(),
            _phantom_data: PhantomData,
            _hash: CURRENT_HASH.fetch_add(1, Ordering::SeqCst),
        }
    }

    fn to_tuple(&self) -> (String, TypeId) {
        (self.name.to_owned(), self._phantom_data.type_id())
    }
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl<T> Hash for Key<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self._hash)
    }
}

pub struct ValueHolderImpl<T>
where
    T: Clone,
{
    value: Box<T>,
}

impl<T> ValueHolderImpl<T>
where
    T: Clone,
{
    pub fn new(value: Box<T>) -> Self {
        Self { value }
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Clone for ValueHolderImpl<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct UserData {
    values_map: Option<HashMap<(String, TypeId), Rc<Box<dyn Any>>>>,
}

impl UserData {
    pub fn new() -> Self {
        Self { values_map: None }
    }

    pub fn put<T: Clone + 'static>(&mut self, key: &Key<T>, value: Box<T>) {
        self.initialize_values_map();

        let values_map = self.values_map.as_mut().unwrap();
        values_map.insert(key.to_tuple(), Rc::new(Box::new(ValueHolderImpl::new(value))));
    }

    fn initialize_values_map(&mut self) {
        if self.values_map.is_some() {
            return;
        }

        self.values_map = Some(HashMap::new());
    }

    pub fn remove<T>(&mut self, key: &Key<T>)
    where
        T: Clone + 'static,
    {
        if self.values_map.is_none() {
            return;
        }

        self.values_map.as_mut().unwrap().remove(&key.to_tuple());
    }

    pub fn get<T: Clone + 'static>(&self, key: &Key<T>) -> Option<&T> {
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
}

impl Clone for UserData {
    fn clone(&self) -> Self {
        match self.values_map.as_ref() {
            None => Self { values_map: None },
            Some(map) => {
                let mut new_map = HashMap::new();
                for (key, value) in map {
                    new_map.insert(key.clone(), Rc::clone(value));
                }

                Self {
                    values_map: Some(new_map),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct UserDataHolder {
    user_data: Option<UserData>,
}

impl UserDataHolder {
    pub fn new() -> Self {
        Self { user_data: None }
    }

    pub fn get_mut(&mut self) -> &mut UserData {
        if self.user_data.is_none() {
            self.user_data = Some(UserData::new());
        }

        self.user_data.as_mut().unwrap()
    }
}

impl Clone for UserDataHolder {
    fn clone(&self) -> Self {
        match self.user_data.as_ref() {
            None => Self { user_data: None },
            Some(user_data) => Self {
                user_data: Some(user_data.clone()),
            },
        }
    }
}
