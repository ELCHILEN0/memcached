use std::marker::PhantomData;

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

}