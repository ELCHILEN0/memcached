use std::marker::PhantomData;

use cache::key::Key;
use cache::value::Value;
use cache::storage_structure::CacheStorageStructure;
use cache::replacement_policy::CacheReplacementPolicy;

pub struct Cache<T, R> {
    pub capacity: usize,
    pub item_lifetime: u64,
    pub max_key_len: usize,
    pub max_val_len: usize,
    pub storage_structure: T,
    pub phantom: PhantomData<R>

}

impl <R: CacheReplacementPolicy, T: CacheStorageStructure<R>> Cache<T, R> {
    pub fn get(&mut self, key: Key) -> Option<Value> {
        self.storage_structure.get(key)
    }

    pub fn set(&mut self, key: Key, value: Value) {
        self.storage_structure.set(key, value).unwrap()
    }

    pub fn remove(&mut self, key: Key) {
        self.storage_structure.remove(key)
    }

    pub fn contains(&mut self, key: Key) -> bool {
        self.storage_structure.contains(key)
    }
}