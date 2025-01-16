use std::collections::{HashMap, VecDeque};
use crate::traits::Cache;

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    list: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LruCache<K, V>{
    pub fn new(capacity: usize) -> Self {
        LruCache { 
            capacity, 
            map: HashMap::new(), 
            list: VecDeque::new() }
    }
}

impl<K: Eq + std::hash::Hash + Clone, V> Cache<K, V> for LruCache<K, V> {
    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key){
            self.list.retain(|k| k != key);
            self.list.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V){
        if self.list.len() == self.capacity {
            if let Some(lru) = self.list.pop_front() {
                self.map.remove(&lru);
            }
        }

        self.list.push_back(key.clone());
        self.map.insert(key.clone(), value);
    }
}