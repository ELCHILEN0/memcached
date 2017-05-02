use std::collections::HashMap;

use cache::key::Key;
use cache::value::Value;
use cache::replacement_policy::CacheReplacementPolicy;

pub trait CacheStorageStructure<T> {
    fn new(replacement_policy: T) -> Self;
    fn get(&mut self, key: Key) -> Option<Value>;
    fn set(&mut self, key: Key, value: Value);
}

pub struct HashStorageStructure<T> {
    rid_map: HashMap<Key, usize>,
    data: Vec<Value>,
    replacement_policy: T,
}

impl<T: CacheReplacementPolicy> CacheStorageStructure<T> for HashStorageStructure<T> {
    fn new(replacement_policy: T) -> Self {
        HashStorageStructure {
            rid_map: HashMap::new(),
            data: Vec::new(),
            replacement_policy: replacement_policy
        }
    }    

    fn get(&mut self, key: Key) -> Option<Value> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => Some(self.data[index].clone()),
            None => None
        }
    }

    fn set(&mut self, key: Key, value: Value) {
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                self.data.remove(index);
                self.data.insert(index, value);
            },
            None => {
                self.data.push(value);
                self.rid_map.insert(key, self.data.len() - 1);
            },
        };

    }
}