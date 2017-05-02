use std::collections::HashMap;

use cache::key::Key;
use cache::value::Value;
use cache::replacement_policy::CacheReplacementPolicy;

pub trait CacheStorageStructure<T> {
    fn new(replacement_policy: T) -> Self;

    // fn size(&self) -> usize;
    // fn capacity

    fn get(&mut self, key: Key) -> Option<Value>;
    fn set(&mut self, key: Key, value: Value) -> Result<(), &str>;
    fn remove(&mut self, key: Key);
    fn contains(&mut self, key: Key) -> bool;
}

pub struct HashStorageStructure<T> {
    rid_map: HashMap<Key, usize>,
    data: Vec<Value>,
    replacement_policy: T,
    size: usize,
    capacity: usize,
}

impl<T: CacheReplacementPolicy> CacheStorageStructure<T> for HashStorageStructure<T> {
    fn new(replacement_policy: T) -> Self {
        HashStorageStructure {
            rid_map: HashMap::new(),
            data: Vec::new(),
            replacement_policy: replacement_policy,
            size: 0,
            capacity: 360
        }
    }    

    fn get(&mut self, key: Key) -> Option<Value> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => Some(self.data[index].clone()),
            None => None
        }
    }

    fn set(&mut self, key: Key, value: Value) -> Result<(), &str> {
        println!("Size: {}", self.size);
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                let current = self.data[index].clone();

                loop {
                    if self.size + value.len() - current.len() <= self.capacity {
                        break;
                    }

                    match self.replacement_policy.next_candidate() {
                        Some(evict_key) => {
                            println!("{}", evict_key.clone().item);
                            match self.rid_map.get(&evict_key) {
                                Some(evict_index) => {
                                    self.size -= self.data[*evict_index].len();
                                },
                                None => {
                                    return Err("Index map not found");
                                }
                            }
                            self.remove(evict_key);
                        },
                        None => {
                            return Err("No more candidates")
                        }
                    }
                }

                self.size += value.len();                
                self.data.remove(index);
                self.data.insert(index, value);
            },
            None => {
                loop {
                    if self.size + value.len() <= self.capacity {
                        break;
                    }

                    match self.replacement_policy.next_candidate() {
                        Some(evict_key) => {
                            println!("{}", evict_key.clone().item);                            
                            match self.rid_map.get(&evict_key) {
                                Some(evict_index) => {
                                    self.size -= self.data[*evict_index].len();
                                },
                                None => {
                                    return Err("Index map not found");
                                }
                            }
                            self.remove(evict_key);
                        },
                        None => {
                            return Err("No more candidates")
                        }
                    }
                }

                self.size += value.len();
                self.data.push(value);
                self.rid_map.insert(key.clone(), self.data.len() - 1);
            },
        };
        self.replacement_policy.update(key);
        Ok(())
    }

    fn remove(&mut self, key: Key) {
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                self.data.remove(index);
                self.rid_map.remove(&key);
            },
            None => {},
        };
        self.replacement_policy.remove(key);
    }

    fn contains(&mut self, key: Key) -> bool {
        self.rid_map.contains_key(&key)
    }
}