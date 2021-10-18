use std::collections::HashMap;
use std::hash::Hash;

use serde_json::Value;

pub fn map_push<K, V>(dict: &mut HashMap<K, Vec<V>>, key: K, value: V)
where
    K: Eq + Hash,
{
    match dict.get_mut(&key) {
        Some(list) => list.push(value),
        None => {
            dict.insert(key, vec![value]);
        }
    }
}

pub fn map_append<K, V>(dict: &mut HashMap<K, Vec<V>>, from: HashMap<K, Vec<V>>)
where
    K: Eq + Hash,
    V: Clone,
{
    for (key, vs) in from {
        match dict.get_mut(&key) {
            Some(list) => list.append(&mut vs.clone()),
            None => {
                dict.insert(key, vs);
            }
        }
    }
}

pub type Object = serde_json::Map<String, serde_json::Value>;

pub fn merge_value(left: &mut Value, right: Value) {
    if left.is_array() && right.is_array() {
        let mut right = right.as_array().unwrap().clone();
        left.as_array_mut().unwrap().append(&mut right);
    } else if left.is_object() && right.is_object() {
        let left = left.as_object_mut().unwrap();
        let right = right.as_object().unwrap().clone();
        merge_object(left, right)
    } else {
        *left = right
    }
}

pub fn merge_object(left: &mut Object, right: Object) {
    for (key, value) in right.into_iter() {
        if let Some(old) = left.get_mut(&key) {
            merge_value(old, value);
        } else {
            left.insert(key, value);
        }
    }
}

pub fn identity<T>(x: T) -> T { x }

pub fn map_apply<K, V>(left: &mut HashMap<K, V>, right: HashMap<K, V>)
where
    K: Eq + Hash,
{
    for (k, v) in right.into_iter() {
        left.insert(k, v);
    }
}

pub fn to_map_by<I, K, F, V>(defs: I, mut get_key: F) -> HashMap<K, Vec<V>>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> (K, V),
    K: Eq + Hash,
{
    let mut out = HashMap::new();
    for value in defs.into_iter() {
        let (key, value) = get_key(value);
        map_push(&mut out, key, value);
    }
    out
}
