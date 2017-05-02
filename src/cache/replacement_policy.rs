use cache::key::Key;
use cache::value::Value;

pub trait CacheReplacementPolicy {
    fn new() -> Self;
    fn is_candidate(value: Value) -> bool;
    fn evict(key: Key, value: Value);
}

pub struct LRU;

impl CacheReplacementPolicy for LRU {
    fn new() -> Self {
        LRU {}
    }

    fn is_candidate(value: Value) -> bool {
        unimplemented!();
    }
    
    fn evict(key: Key, value: Value) {
        unimplemented!();
    }
}