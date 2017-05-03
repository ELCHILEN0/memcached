use cache::key::Key;
use cache::value::Value;
use cache::data_entry::DataEntry;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;
use cache::error::CacheError;

pub struct CacheMetrics {
    pub evictions: u64,
    pub hit_count_get: u64,
    pub hit_count_set: u64,
    pub hit_count_delete: u64,
    pub miss_count_get: u64,
    pub miss_count_set: u64,
    pub miss_count_delete: u64,
}

pub struct Cache<T, R> {
    pub capacity: usize,
    pub item_lifetime: u64,
    pub max_key_len: usize,
    pub max_val_len: usize,
    pub storage_structure: T,
    pub replacement_policy: R,
    pub metrics: CacheMetrics,
}

impl CacheMetrics {
    pub fn new() -> CacheMetrics {
        CacheMetrics {
            evictions: 0,
            hit_count_get: 0,
            hit_count_set: 0,
            hit_count_delete: 0,
            miss_count_get: 0,
            miss_count_set: 0,
            miss_count_delete: 0,
        }
    }
}

impl <T: CacheStorageStructure, R: CacheReplacementPolicy> Cache<T, R> {
    pub fn new(capacity: usize, storage_structure: T, replacement_policy: R) -> Cache<T, R> {
        Cache {
                capacity: capacity,
                item_lifetime: 60 * 1000,
                max_key_len: 256,
                max_val_len: 512,
                storage_structure: storage_structure,
                // replacement_policy: LRU::new(),
                // replacement_policy: Clock::new(),
                replacement_policy: replacement_policy,
                metrics: CacheMetrics::new()
        }
    }

    pub fn get(&mut self, key: Key) -> Option<DataEntry> {
        match self.storage_structure.get(key) {
            Some((index, entry)) => {
                self.replacement_policy.update(index);
                self.metrics.hit_count_get += 1;
                Some(entry)
            },
            None => {
                self.metrics.miss_count_get += 1;
                None
            }
        }
    }

    pub fn set(&mut self, key: Key, value: Value) -> Result<(), CacheError> {
        let entry = DataEntry::new(key.clone(), value);

        // Retrieve the current size of the entry in the cache
        let current_elem_size = match self.storage_structure.get(key) {
            Some((_, curr_entry)) => curr_entry.len(),
            None => 0,
        };

        if current_elem_size == 0 {
            self.metrics.miss_count_set += 1;
        } else {
            self.metrics.hit_count_set += 1;
        }

        // Evict until there is sufficient space
        loop {
            if self.storage_structure.size() + entry.len() - current_elem_size <= self.capacity {
                break;
            }

            try!(self.evict_next());
            self.metrics.evictions += 1;
        }

        // Set the value in the cache
        let (index, _) = self.storage_structure.set(entry);
        // Update replacement policy
        self.replacement_policy.update(index);
        
        Ok(())
    }

    pub fn remove(&mut self, key: Key) {
        match self.storage_structure.remove(key) {
            Some((index, _)) => {
                self.replacement_policy.remove(index);
                self.metrics.hit_count_delete += 1;
            },
            None => {
                self.metrics.miss_count_delete += 1;
            },
        };
    }

    pub fn contains(&mut self, key: Key) -> bool {
        self.storage_structure.contains(key)
    }

    fn evict_next(&mut self) -> Result<(), CacheError> {
        // Disasociate the index from the replacement policy
        match self.replacement_policy.evict_next() {
            Ok(evict_index) => {
                // Remove the index from the cache
                match self.storage_structure.remove_index(evict_index) {
                    Some((_, _)) => Ok(()),
                    None => Err(CacheError::EvictionFailure)
                }
            },
            Err(err) => Err(err)
        }
    }
}