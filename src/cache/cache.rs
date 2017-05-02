use std::marker::PhantomData;

use cache::key::Key;
use cache::value::Value;
use cache::data_entry::DataEntry;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;
use cache::error::CacheError;

pub struct Cache<T, R> {
    pub capacity: usize,
    pub item_lifetime: u64,
    pub max_key_len: usize,
    pub max_val_len: usize,
    pub storage_structure: T,
    pub replacement_policy: R
}

impl <T: CacheStorageStructure, R: CacheReplacementPolicy> Cache<T, R> {
    pub fn get(&mut self, key: Key) -> Option<DataEntry> {
        self.storage_structure.get(key)
    }

    pub fn set(&mut self, key: Key, value: Value) -> Result<(), CacheError> {
        // Ensure that adding a new element will not overflow the capacity of the cache
        let current_elem_size = match self.storage_structure.get(key.clone()) {
            Some(value) => value.len(),
            None => 0,
        };

        loop {
            if self.storage_structure.size() + value.len() - current_elem_size <= self.capacity {
                break;
            }

            try!(self.evict_next());
        }

        let index = self.replacement_policy.update(key.clone());
        try!(self.storage_structure.set(key.clone(), value));
        try!(self.storage_structure.move_to_index(key, index));
        Ok(())
    }

    pub fn remove(&mut self, key: Key) {
        self.replacement_policy.remove(key.clone());
        self.storage_structure.remove(key);
    }

    pub fn contains(&mut self, key: Key) -> bool {
        self.storage_structure.contains(key)
    }

    fn evict_next(&mut self) -> Result<(), CacheError> {
        match self.replacement_policy.evict_next() {
            Some(evict_index) => {
                match self.storage_structure.get_index(evict_index) {
                    Some(evict) => {
                        match self.storage_structure.remove(evict.key) {
                            Ok(evicted_value) => Ok(()),
                            Err(err) => Err(CacheError::EvictionFailure)
                        }
                    },
                    None => Err(CacheError::EvictionFailure)
                }
            },
            None => Err(CacheError::EvictionFailure)
        }
    }
}