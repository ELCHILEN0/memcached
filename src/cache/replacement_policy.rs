use std::collections::{HashMap, VecDeque};
use linked_hash_map::LinkedHashMap;

use cache::key::Key;
use cache::value::Value;

pub trait CacheReplacementPolicy {
    fn new() -> Self;
    fn update(&mut self, index: usize) -> Option<usize>;
    fn remove(&mut self, index: usize);
    fn evict_next(&mut self) -> Option<usize>;
    // fn evict_next(&mut self) -> Option<Key>;
}

pub struct LRU;
pub struct Clock {
    hand: usize,
    referenced: Vec<bool>,
}

impl CacheReplacementPolicy for LRU {
    fn new() -> Self {
        LRU { }
    }

    fn update(&mut self, index: usize) -> Option<usize> {
        Some(0)
    }

    fn remove(&mut self, index: usize) {

    }
    
    fn evict_next(&mut self) -> Option<usize> {
        Some(0)
    }
}

impl CacheReplacementPolicy for Clock {
    fn new() -> Self {
        Clock {
            hand: 0,
            referenced: Vec::new(),
         }
    }

    fn update(&mut self, index: usize) -> Option<usize> {
        if index < self.referenced.len() {
            self.referenced[index] = true;
        } else {
            self.referenced.insert(index, true);
        }
        None
    }

    fn remove(&mut self, index: usize) {
        self.referenced.remove(index);
    }
    
    fn evict_next(&mut self) -> Option<usize> {
        println!("{}, {:?}", self.hand, self.referenced);
        
        let target_key: Key;
        'outer: loop {
            for value in self.referenced.iter_mut().skip(self.hand) {
                self.hand += 1;

                if *value {
                    *value = false;
                } else {
                    break 'outer;
                }
            }

            self.hand = 0;
        }

        self.referenced.remove(self.hand - 1);
        Some(self.hand - 1)
    }
}