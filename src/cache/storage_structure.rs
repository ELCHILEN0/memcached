use std::collections::HashMap;

use cache::key::Key;
use cache::value::Value;
use cache::data_entry::DataEntry;
use cache::replacement_policy::CacheReplacementPolicy;

use cache::error::CacheError;

/**
 * TODO: Currently we maintain two separate data structures for storage and for maintaining the cache.
 * This does not allow us to make optimizations to the storage in terms related to the replacement
 * policies.  e.g. LRU reorders entries on insert and requires no additional storage
 *
 * To meet this goal we should expose an interface for storage that allows us to restructure while
 * maintaining all storage indexes and data.  Think a move_to_index(Key, index).  If we constrain every
 * storage structure to be simply a Vec<Value> with keys indexing to unique elements then move to simply
 * has to update the keys to point to the new index, easy!  The replacement policy can now return an
 * an index when a value is updated that will trigger the rebuild of the structure!
 */
pub trait CacheStorageStructure {
    fn new() -> Self;

    fn size(&self) -> usize;

    fn get(&mut self, key: Key) -> Option<DataEntry>;
    fn set(&mut self, key: Key, value: Value) -> Result<usize, CacheError>;
    fn remove(&mut self, key: Key) -> Result<DataEntry, CacheError>;
    fn contains(&mut self, key: Key) -> bool;

    fn get_index(&mut self, key: Key) -> Option<usize>;
    fn get_with_index(&mut self, index: usize) -> Option<DataEntry>;
    fn move_entry(&mut self, key: Key, index: usize) -> Result<(), CacheError>;
}

pub struct HashStorageStructure {
    rid_map: HashMap<Key, usize>,
    data: Vec<DataEntry>,
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

    fn get(&mut self, key: Key) -> Option<DataEntry> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => Some(self.data[index].clone()),
            None => None
        }
    }

    fn set(&mut self, key: Key, value: Value) -> Result<usize, CacheError> {
        self.size += value.len();                
        
        let new = DataEntry::new(key.clone(), value);
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                self.data.remove(index);
                self.data.insert(index, new);
                Ok(index)
            },
            None => {
                self.data.push(new);
                self.rid_map.insert(key, self.data.len() - 1);
                Ok(self.data.len() - 1)
            },
        }
    }

    fn remove(&mut self, key: Key) -> Result<DataEntry, CacheError> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                let removed = self.data.remove(index);
                self.rid_map.remove(&key);
                self.size -= removed.len();
                Ok(removed)
            },
            None => Err(CacheError::KeyNotFound),
        }
    }

    fn contains(&mut self, key: Key) -> bool {
        self.rid_map.contains_key(&key)
    }

    fn get_index(&mut self, key: Key) -> Option<usize> {
        self.rid_map.get(&key).cloned()
    }

    fn get_with_index(&mut self, index: usize) -> Option<DataEntry> {
        self.data.get(index).cloned()
    }

    fn move_entry(&mut self, key: Key, index: usize) -> Result<(), CacheError> {
        match self.rid_map.get(&key).cloned() {
            Some(index) => {
                let removed = self.data.remove(index);
                self.data.insert(index, removed);
                Ok(())
            },
            None => Err(CacheError::KeyNotFound)
        }
    }
}