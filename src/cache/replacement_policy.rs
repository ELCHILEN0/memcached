use std::collections::{HashMap, VecDeque};
use linked_hash_map::LinkedHashMap;

use cache::key::Key;
use cache::value::Value;
use cache::error::CacheError;

pub trait CacheReplacementPolicy {
    fn new() -> Self;
    fn update(&mut self, index: usize);
    fn remove(&mut self, index: usize);
    fn evict_next(&mut self) -> Result<usize, CacheError>;
}

pub struct LRU {
    recently_used: VecDeque<usize>, 
}

pub struct Clock {
    hand: usize,
    referenced_list: Vec<bool>,
}

pub struct LFU {
    frequency_list: Vec<usize>, // (index, hits)
}

impl CacheReplacementPolicy for LRU {
    fn new() -> Self {
        LRU { 
            recently_used: VecDeque::new()
        }
    }

    fn update(&mut self, index: usize) {
        let mut target_index: usize = 0;
        for iter_index in self.recently_used.iter() {
            if *iter_index == index {
                break;
            }
            target_index += 1;
        }
        self.recently_used.remove(target_index);
        self.recently_used.push_back(target_index);
    }

    fn remove(&mut self, index: usize) {
        let mut target_index: usize = 0;
        for iter_index in self.recently_used.iter() {
            if *iter_index == index {
                break;
            }
            target_index += 1;
        }
        self.recently_used.remove(target_index);
    }
    
    fn evict_next(&mut self) -> Result<usize, CacheError> {
        match self.recently_used.pop_front() {
            Some(index) => Ok(index),
            None => Err(CacheError::EvictionFailure)
        }
    }
}

impl CacheReplacementPolicy for Clock {
    fn new() -> Self {
        Clock {
            hand: 0,
            referenced_list: Vec::new(),
         }
    }

    fn update(&mut self, index: usize) {
        if index < self.referenced_list.len() {
            self.referenced_list[index] = true;
        } else {
            self.referenced_list.insert(index, true);
        }
    }

    fn remove(&mut self, index: usize) {
        self.referenced_list.remove(index);
    }
    
    fn evict_next(&mut self) -> Result<usize, CacheError> {        
        'outer: loop {
            if self.referenced_list.len() == 0 {
                return Err(CacheError::EvictionFailure);
            }

            for value in self.referenced_list.iter_mut().skip(self.hand) {
                self.hand += 1;

                if *value {
                    *value = false;
                } else {
                    break 'outer;
                }
            }

            self.hand = 0;
        }

        self.referenced_list.remove(self.hand - 1);
        Ok(self.hand - 1)
    }
}

impl CacheReplacementPolicy for LFU {
    fn new() -> Self {
        LFU { 
            frequency_list: Vec::new()
        }
    }

    fn update(&mut self, index: usize) {
        if index < self.frequency_list.len() {
            self.frequency_list[index] = self.frequency_list[index] + 1;
        } else {
            self.frequency_list.insert(index, 1);
        }
    }

    fn remove(&mut self, index: usize) {
        self.frequency_list.remove(index);
    }
    
    fn evict_next(&mut self) -> Result<usize, CacheError> {
        if self.frequency_list.len() == 0 {
            return Err(CacheError::EvictionFailure);
        }

        let mut index = 0;
        let mut target_index = 0;
        for frequency in self.frequency_list.iter() {
            if *frequency < self.frequency_list[target_index] {
                target_index = index;
            }

            index += 1;
        }

        self.frequency_list.remove(target_index);
        Ok(target_index)
    }
}