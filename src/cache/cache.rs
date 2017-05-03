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

        // Evict until there is sufficient space, TODO: Add error handling
        loop {
            if self.storage_structure.size() + value.len() - current_elem_size <= self.capacity {
                break;
            }

            try!(self.evict_next());
        }

        // Add/Update the value in the cache, return its current index
        let index = try!(self.storage_structure.set(key.clone(), value));
        // Update the associated cache replacement policy, returning the new index, move it there
        match self.replacement_policy.update(index) {
            Some(new_index) => try!(self.storage_structure.move_entry(key, new_index)),
            None => {}
        }
        
        Ok(())
    }

    pub fn remove(&mut self, key: Key) {
        // TODO: Remove index
        match self.storage_structure.get_index(key.clone()) {
            Some(index) => {
                self.replacement_policy.remove(index);
                self.storage_structure.remove(key);
            },
            None => {
                self.storage_structure.remove(key);
            },
        };
    }

    pub fn contains(&mut self, key: Key) -> bool {
        self.storage_structure.contains(key)
    }

    fn evict_next(&mut self) -> Result<(), CacheError> {
        // Determine the next candidate and remove it
        match self.replacement_policy.evict_next() {
            Some(evict_index) => {
                // Given an index, find the entry
                match self.storage_structure.get_with_index(evict_index) {
                    Some(evict) => {
                        // Remove the entry from the cache
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