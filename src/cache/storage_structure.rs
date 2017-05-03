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

    // TODO: Cleanup these convoluted return types
    
    /**
     * Returns the index and entry if it exists
     */
    fn get(&mut self, key: Key) -> Option<(usize, DataEntry)>;
    fn get_index(&mut self, index: usize) -> Option<(usize, DataEntry)>;

    /**
     * Set a key, value pair and return the new index and the removed entry if it exists
     */
    fn set(&mut self, key: Key, value: Value) -> (usize, Option<DataEntry>);
    fn set_index(&mut self, index: usize, key: Key, value: Value) -> (usize, Option<DataEntry>);

    /**
     * Remove a key, value pair and return the old index and entry if it exists
     */
    fn remove(&mut self, key: Key) -> Option<(usize, DataEntry)>;
    fn remove_index(&mut self, index: usize) -> Option<(usize, DataEntry)>;

    fn contains(&mut self, key: Key) -> bool;
}

/**
 * A naive storage structure with O(n) insert, lookup, and delete.
 */
pub struct NaiveStorageStructure {
    data: Vec<DataEntry>,
    size: usize,
}

impl CacheStorageStructure for NaiveStorageStructure {
    fn new() -> Self {
        NaiveStorageStructure {
            data: Vec::new(),
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn get(&mut self, key: Key) -> Option<(usize, DataEntry)> {
        let mut index: usize = 0;
        for entry in self.data.clone().into_iter() {
            if entry.key == key {
                return self.get_index(index);
            }
            index += 1;
        }

        None
    }

    fn get_index(&mut self, index: usize) -> Option<(usize, DataEntry)> {
        match self.data.get(index) {
            Some(entry) => Some((index, entry.clone())),
            None => None
        }
    }    

    fn set(&mut self, key: Key, value: Value) -> (usize, Option<DataEntry>) {
        let new = DataEntry::new(key.clone(), value);
        self.size += new.len();  

        match self.remove(key) {
            Some((index, entry)) => {
                self.data.insert(index, new);
                (index, Some(entry))
            },
            None => {
                self.data.push(new);
                (self.data.len() - 1, None)
            }
        }
    }

    fn set_index(&mut self, index: usize, key: Key, value: Value) -> (usize, Option<DataEntry>) {
        unimplemented!()
    }    

    fn remove(&mut self, key: Key) -> Option<(usize, DataEntry)> {
        match self.get(key) {
            Some((index, entry)) => self.remove_index(index),
            None => None
        }
    }

    fn remove_index(&mut self, index: usize) -> Option<(usize, DataEntry)> {
        let removed = self.data.remove(index);
        self.size -= removed.len();
        Some((index, removed.clone()))
    }

    fn contains(&mut self, key: Key) -> bool {
        match self.get(key) {
            Some(index) => true,
            None => false
        }
    }
}

// pub struct HashStorageStructure {
//     rid_map: HashMap<Key, usize>,
//     data: Vec<DataEntry>,
//     size: usize,
// }

// impl CacheStorageStructure for HashStorageStructure {
//     fn new() -> Self {
//         HashStorageStructure {
//             rid_map: HashMap::new(),
//             data: Vec::new(),
//             size: 0
//         }
//     }    

//     fn size(&self) -> usize {
//         self.size
//     }

//     fn get(&mut self, key: Key) -> Option<DataEntry> {
//         match self.rid_map.get(&key).cloned() {
//             Some(index) => Some(self.data[index].clone()),
//             None => None
//         }
//     }

//     fn set(&mut self, key: Key, value: Value) -> Result<usize, CacheError> {
//         self.size += value.len();                
        
//         let new = DataEntry::new(key.clone(), value);
//         match self.rid_map.get(&key).cloned() {
//             Some(index) => {
//                 self.data.remove(index);
//                 self.data.insert(index, new);
//                 Ok(index)
//             },
//             None => {
//                 self.data.push(new);
//                 self.rid_map.insert(key, self.data.len() - 1);
//                 Ok(self.data.len() - 1)
//             },
//         }
//     }

//     fn remove(&mut self, key: Key) -> Result<DataEntry, CacheError> {
//         match self.rid_map.get(&key).cloned() {
//             Some(index) => {
//                 // TODO: Figure out more efficient way
//                 let removed = self.data.remove(index);
//                 let start_index = self.rid_map.get(&key.clone()).cloned().unwrap();
//                 for k in self.data.iter_mut().skip(start_index) {
//                     let new_index = self.rid_map.get(&k.key).unwrap() - 1;
//                     self.rid_map.insert(k.key.clone(), new_index);
//                 }
//                 self.rid_map.remove(&key);
//                 self.size -= removed.len();

//                 Ok(removed)
//             },
//             None => Err(CacheError::KeyNotFound),
//         }
//     }

//     fn contains(&mut self, key: Key) -> bool {
//         self.rid_map.contains_key(&key)
//     }

//     fn get_index(&mut self, key: Key) -> Option<usize> {
//         self.rid_map.get(&key).cloned()
//     }

//     fn get_with_index(&mut self, index: usize) -> Option<DataEntry> {
//         self.data.get(index).cloned()
//     }

//     fn move_entry(&mut self, key: Key, index: usize) -> Result<(), CacheError> {
//         match self.rid_map.get(&key).cloned() {
//             Some(index) => {
//                 let removed = self.data.remove(index);
//                 self.data.insert(index, removed);
//                 Ok(())
//             },
//             None => Err(CacheError::KeyNotFound)
//         }
//     }
// }