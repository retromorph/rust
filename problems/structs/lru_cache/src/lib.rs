#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    cache: HashMap<K, V>,
    last_access: HashMap<K, usize>,
    priority: BTreeMap<usize, K>,
    time: usize,
    capacity: usize,
    size: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Ord + Debug,
{
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity must be greater than 0");
        }

        Self {
            cache: HashMap::new(),
            last_access: HashMap::new(),
            priority: BTreeMap::new(),
            time: 0,
            capacity,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if !self.last_access.contains_key(key) {
            return None;
        }

        self.touch_key(key);
        self.cache.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if !self.last_access.contains_key(key) {
            return None;
        }

        self.touch_key(key);
        self.cache.get_mut(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.cache.contains_key(&key) {
            self.touch_key(&key);

            let old = self.cache.remove(&key);
            self.cache.insert(key, value);
            return old;
        }

        self.size += 1;
        self.touch_key(&key);
        self.cache.insert(key, value);

        if self.size > self.capacity {
            let priority_and_key = self.priority.pop_first().unwrap();
            self.cache.remove(&priority_and_key.1);
            self.last_access.remove(&priority_and_key.1);
            self.size -= 1;
        }

        None
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.last_access.clear();
        self.priority.clear();
        self.time = 0;
        self.size = 0;
    }

    fn touch_key(&mut self, key: &K) {
        if let Some(&time) = self.last_access.get(key) {
            self.priority.remove(&time);
        }

        self.priority.insert(self.time, key.clone());
        self.last_access.insert(key.clone(), self.time);
        self.time += 1;
    }
}
