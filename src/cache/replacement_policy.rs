use std::collections::{HashMap, VecDeque};
use linked_hash_map::LinkedHashMap;

use cache::key::Key;
use cache::value::Value;

pub trait CacheReplacementPolicy {
    fn new() -> Self;
    fn update(&mut self, index: usize);
    fn remove(&mut self, index: usize);
    fn evict_next(&mut self) -> Option<usize>;
    // fn evict_next(&mut self) -> Option<Key>;
}

pub struct LRU {
    recently_used: VecDeque<usize>, 
}
pub struct Clock {
    hand: usize,
    referenced: Vec<bool>,
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
    
    fn evict_next(&mut self) -> Option<usize> {
        self.recently_used.pop_front()
    }
}

// impl CacheReplacementPolicy for Clock {
//     fn new() -> Self {
//         Clock {
//             hand: 0,
//             referenced: Vec::new(),
//          }
//     }

//     fn update(&mut self, index: usize) -> Option<usize> {
//         if index < self.referenced.len() {
//             self.referenced[index] = true;
//         } else {
//             self.referenced.insert(index, true);
//         }
//         println!("update: {}, {:?}", index, self.referenced);
        
//         None
//     }

//     fn remove(&mut self, index: usize) {
//         self.referenced.remove(index);
//     }
    
//     fn evict_next(&mut self) -> Option<usize> {
//         println!("{}, {:?}", self.hand, self.referenced);
        
//         let target_key: Key;
//         'outer: loop {
//             for value in self.referenced.iter_mut().skip(self.hand) {
//                 self.hand += 1;

//                 if *value {
//                     *value = false;
//                 } else {
//                     break 'outer;
//                 }
//             }

//             self.hand = 0;
//         }

//         self.referenced.remove(self.hand - 1);
//         Some(self.hand - 1)
//     }
// }