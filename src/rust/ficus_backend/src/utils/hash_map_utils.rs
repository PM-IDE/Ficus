use std::collections::HashMap;
use std::hash::Hash;

pub fn increase_in_map<TKey>(map: &mut HashMap<TKey, usize>, key: &TKey)
where
    TKey: Hash + Eq + PartialEq + Clone,
{
    if let Some(value) = map.get_mut(key) {
        *value += 1;
    } else {
        map.insert(key.clone(), 1);
    }
}

pub fn add_to_list_in_map<TKey, TValue>(map: &mut HashMap<TKey, Vec<TValue>>, key: &TKey, value: TValue)
where
    TKey: Hash + Eq + PartialEq + Clone,
{
    if let Some(list) = map.get_mut(key) {
        list.push(value);
    } else {
        map.insert(key.clone(), vec![value]);
    }
}
