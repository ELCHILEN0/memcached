use cache::key::Key;
use cache::data_entry::DataEntry;

/**
 * With the current layout there must be two highly associated data sructures for maintaining the
 * cache.  The storage_structure maintains the data while the replacement_policy maintains indexed
 * data regarding the state of the entries in the cache.  With some replacment policies it is
 * possible to obtain better storage space by removing the dependency on the replacement policy
 * structure (e.g. LRU reorders structure entries on insert and the policy simply returns 0).
 * However, this tends to make the dependencies between the two traits tricky to manage and for
 * more practical replacement policies the extra overhead is often needed regardless.  Also keep in
 * mind that for different types of data structures the indexing scheme might change and may not
 * be able to support a reordering replacement policy.
 */
pub trait CacheStorageStructure {
    fn new() -> Self;

    fn size(&self) -> usize;
    
    /**
     * Returns the index and entry if it exists
     */
    fn get(&mut self, key: Key) -> Option<(usize, DataEntry)>;
    fn get_index(&mut self, index: usize) -> Option<(usize, DataEntry)>;

    /**
     * Set a key, value pair and return the new index and the removed entry if it exists
     */
    fn set(&mut self, entry: DataEntry) -> (usize, Option<DataEntry>);
    fn set_index(&mut self, index: usize, entry: DataEntry) -> (usize, Option<DataEntry>);

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

    fn set(&mut self, entry: DataEntry) -> (usize, Option<DataEntry>) {
        self.size += entry.len();  

        match self.remove(entry.key.clone()) {
            Some((index, old_entry)) => {
                self.data.insert(index, entry);
                (index, Some(old_entry))
            },
            None => {
                self.data.push(entry);
                (self.data.len() - 1, None)
            }
        }
    }

    fn set_index(&mut self, index: usize, entry: DataEntry) -> (usize, Option<DataEntry>) {
        unimplemented!()
    }    

    fn remove(&mut self, key: Key) -> Option<(usize, DataEntry)> {
        match self.get(key) {
            Some((index, _)) => self.remove_index(index),
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
            Some(_) => true,
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