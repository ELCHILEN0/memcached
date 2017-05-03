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
        match self.storage_structure.get(key) {
            Some((index, entry)) => Some(entry),
            None => None
        }
    }

    pub fn set(&mut self, key: Key, value: Value) -> Result<(), CacheError> {
        // Ensure that adding a new element will not overflow the capacity of the cache
        let current_elem_size = match self.storage_structure.get(key.clone()) {
            Some((curr_index, curr_entry)) => curr_entry.len(),
            None => 0,
        };

        // Evict until there is sufficient space, TODO: Add error handling
        loop {
            if self.storage_structure.size() + value.len() - current_elem_size <= self.capacity {
                break;
            }

            try!(self.evict_next());
        }

        // Set the value in the cache, then invoke the replacement policy on the index
        let (index, entry) = self.storage_structure.set(key.clone(), value);
        self.replacement_policy.update(index);
        
        Ok(())
    }

    pub fn remove(&mut self, key: Key) {
        match self.storage_structure.remove(key) {
            Some((index, entry)) => {
                self.replacement_policy.remove(index);
            },
            None => {},
        };
    }

    pub fn contains(&mut self, key: Key) -> bool {
        self.storage_structure.contains(key)
    }

    fn evict_next(&mut self) -> Result<(), CacheError> {
        // Determine the next candidate and remove it
        match self.replacement_policy.evict_next() {
            Ok(evict_index) => {
                match self.storage_structure.remove_index(evict_index) {
                    Some((index, evicted)) => Ok(()),
                    None => Err(CacheError::EvictionFailure)
                }
            },
            Err(err) => Err(CacheError::EvictionFailure)
        }
    }
}