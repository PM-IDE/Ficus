use std::collections::HashMap;

pub struct RelationsCache {
    cache: HashMap<String, HashMap<String, bool>>,
}

impl RelationsCache {
    pub fn empty() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn try_get(&self, first: &str, second: &str) -> Option<&bool> {
        if let Some(map) = self.cache.get(first) {
            if let Some(value) = map.get(second) {
                return Some(value);
            }
        }

        None
    }

    pub fn put(&mut self, first: &str, second: &str, value: bool) {
        if !self.cache.contains_key(first) {
            self.cache.insert(first.to_owned(), HashMap::new());
        }

        let map = self.cache.get_mut(first).unwrap();
        if map.contains_key(second) {
            return;
        }

        map.insert(second.to_owned(), value);
    }
}

pub struct RelationsCaches {
    caches: HashMap<&'static str, RelationsCache>
}

impl RelationsCaches {
    pub fn new(caches_names: &'static [&'static str]) -> Self {
        let mut caches = HashMap::new();
        for name in caches_names {
            caches.insert(*name, RelationsCache::empty());
        }

        Self {
            caches
        }
    }

    pub fn cache(&self, cache_name: &'static str) -> &RelationsCache {
        self.caches.get(cache_name).unwrap()
    }

    pub fn cache_mut(&mut self, cache_name: &'static str) -> &mut RelationsCache {
        self.caches.get_mut(cache_name).unwrap()
    }
}