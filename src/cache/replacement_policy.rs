use std::collections::{HashMap, VecDeque};

use cache::key::Key;
use cache::value::Value;

pub trait CacheReplacementPolicy {
    fn new() -> Self;
    fn update(&mut self, key: Key);
    fn remove(&mut self, key: Key);
    fn next_candidate(&mut self) -> Option<Key>;
}

pub struct LRU {
    key_map: HashMap<Key, usize>,
    key_history: VecDeque<Key>,
}

impl CacheReplacementPolicy for LRU {
    fn new() -> Self {
        LRU {
            key_map: HashMap::new(),
            key_history: VecDeque::new(),
        }
    }

    fn update(&mut self, key: Key) {
        match self.key_map.get(&key).cloned() {
            Some(index) => {
                self.key_history.remove(index);
            },
            None => {}
        };
        self.key_history.push_back(key.clone());
        self.key_map.insert(key, self.key_history.len() - 1);
    }

    fn remove(&mut self, key: Key) {
        match self.key_map.get(&key).cloned() {
            Some(index) => {
                self.key_history.remove(index);
                self.key_map.remove(&key);
            },
            None => {}
        };
    }
    
    fn next_candidate(&mut self) -> Option<Key> {
        // TODO: Remove from key_map
        // self.key_map.remove(self.key_history.get(0))
        match self.key_history.pop_front() {
            Some(key) => {
                self.key_map.remove(&key);
                Some(key)
            },
            None => None
        }
    }
}