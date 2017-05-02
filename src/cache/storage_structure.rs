use std::collections::HashMap;

use cache::key::Key;
use cache::value::Value;
use cache::replacement_policy::CacheReplacementPolicy;

use cache::error::CacheError;

pub trait CacheStorageStructure {
    fn new() -> Self;

    fn size(&self) -> usize;

    fn get(&mut self, key: Key) -> Option<Value>;
    fn set(&mut self, key: Key, value: Value) -> Result<(), CacheError>;
    fn remove(&mut self, key: Key) -> Result<Value, CacheError>;
    fn contains(&mut self, key: Key) -> bool;
}

pub struct HashStorageStructure {
    rid_map: HashMap<Key, usize>,
    data: Vec<Value>,
    size: usize,
}

impl CacheStorageStructure for HashStorageStructure {
    fn new() -> Self {
        HashStorageStructure {
            rid_map: HashMap::new(),
            data: Vec::new(),
            size: 0
        }
    }    

    fn size(&self) -> usize {
        self.size
    }

    fn get(&mut self, key: Key) -> Option<Value> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => Some(self.data[index].clone()),
            None => None
        }
    }

    fn set(&mut self, key: Key, value: Value) -> Result<(), CacheError> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                // let current = self.data[index].clone();

                // loop {
                //     if self.size + value.len() - current.len() <= self.capacity {
                //         break;
                //     }

                //     try!(self.evict_next_candidate());
                // }

                self.size += value.len();                
                self.data.remove(index);
                self.data.insert(index, value);
            },
            None => {
                // loop {
                //     if self.size + value.len() <= self.capacity {
                //         break;
                //     }

                //     try!(self.evict_next_candidate());
                // }

                self.size += value.len();
                self.data.push(value);
                self.rid_map.insert(key.clone(), self.data.len() - 1);
            },
        };
        // self.replacement_policy.update(key);
        Ok(())
    }

    fn remove(&mut self, key: Key) -> Result<Value, CacheError> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                let removed = self.data.remove(index);
                self.rid_map.remove(&key);
                self.size -= removed.len();
                Ok(removed)
            },
            None => Err(CacheError::KeyNotFound),
        }
        // self.replacement_policy.remove(key);
    }

    fn contains(&mut self, key: Key) -> bool {
        self.rid_map.contains_key(&key)
    }


}