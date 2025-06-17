use std::collections::{HashMap, LinkedList};

// Design and implement a data structure for a Least Frequently Used (LFU) cache:
// * LFUCache(int capacity) Initializes the object with the capacity of the data structure.
// * int get(int key) Gets the value of the key if the key exists in the cache. Otherwise, returns -1.
// * void put(int key, int value) Update the value of the key if present, or inserts the key if not 
// already present. When the cache reaches its capacity, it should invalidate and remove the least 
// frequently used key before inserting a new item. For this problem, when there is a tie (i.e., two 
// or more keys with the same frequency), the least recently used key would be invalidated.
//
// To determine the least frequently used key, a use counter is maintained for each key in the cache. 
// The key with the smallest use counter is the least frequently used key.
// When a key is first inserted into the cache, its use counter is set to 1 (due to the put operation). 
// The use counter for a key in the cache is incremented either a get or put operation is called on it.
// The functions get and put must each run in O(1) average time complexity.

pub struct LFUCache {
    capacity: usize,
    min_freq: i32,
    // Maps key to (value, frequency)
    key_to_val_freq: HashMap<i32, (i32, i32)>,
    // Maps frequency to list of keys (front = most recent, back= least recent)
    freq_to_keys: HashMap<i32, LinkedList<i32>>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self { 
            capacity: capacity as usize , 
            min_freq: 0,
            key_to_val_freq: HashMap::new(), 
            freq_to_keys: HashMap::new(), 
        }
    }
            
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&(value, freq)) = self.key_to_val_freq.get(&key) {
            self.update_freq(key, freq);
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if let Some(&(_, freq)) = self.key_to_val_freq.get(&key) {
            // Update existing key
            self.key_to_val_freq.insert(key, (value, freq));
            self.update_freq(key, freq);

        } else {
            // Insert new key

            if self.key_to_val_freq.len() >= self.capacity {
                self.evict_lfu();
            }

            // Add new key with frequency 1
            self.key_to_val_freq.insert(key, (value, 1));
            self.freq_to_keys.entry(1)
            .or_insert_with(LinkedList::new)
            .push_front(key);

            self.min_freq = 1;
        }
    }

    fn update_freq(&mut self, key: i32, old_freq: i32) {
        let new_freq = old_freq + 1;
        
        // Remove key from old frequency list
        if let Some(old_list) = self.freq_to_keys.get_mut(&old_freq) {
            // Find and remove the key from the list
            let mut remaining = LinkedList::new();
            while let Some(k) = old_list.pop_front() {
                if k != key {
                    remaining.push_back(k);
                }
            }
            *old_list = remaining;
            
            // Update min_freq if this frequency list is now empty
            if old_freq == self.min_freq && // we are updating the key with the minimun frequency (=)
                old_list.is_empty() { // the key list corresponding to minimum frequency is empty
                self.min_freq += 1;
            }
        }
        
        // Update the key's frequency in the main map
        if let Some((value, _)) = self.key_to_val_freq.get_mut(&key) {
            *self.key_to_val_freq.get_mut(&key).unwrap() = (*value, new_freq);
        }
        
        // Add key to new frequency list (at front for most recent)
        self.freq_to_keys.entry(new_freq)
            .or_insert_with(LinkedList::new)
            .push_front(key);
    }

    fn evict_lfu(&mut self) {
        // Get the least recently used key from the minimum frequency list
        if let Some(min_list) = self.freq_to_keys.get_mut(&self.min_freq) {
            if let Some(lru_key) = min_list.pop_back() {
                self.key_to_val_freq.remove(&lru_key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfu_cache_example() {
        let mut lfu = LFUCache::new(2);
        
        lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
        lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
        assert_eq!(lfu.get(1), 1); // return 1, cache=[1,2], cnt(2)=1, cnt(1)=2
        
        lfu.put(3, 3);   // 2 is the LFU key, invalidate 2, cache=[3,1], cnt(3)=1, cnt(1)=2
        assert_eq!(lfu.get(2), -1); // return -1 (not found)
        assert_eq!(lfu.get(3), 3);  // return 3, cache=[3,1], cnt(3)=2, cnt(1)=2
        
        lfu.put(4, 4);   // Both 1 and 3 have cnt=2, but 1 is LRU, invalidate 1
        assert_eq!(lfu.get(1), -1); // return -1 (not found)
        assert_eq!(lfu.get(3), 3);  // return 3, cache=[3,4], cnt(4)=1, cnt(3)=3
        assert_eq!(lfu.get(4), 4);  // return 4, cache=[4,3], cnt(4)=2, cnt(3)=3
    }

    #[test]
    fn test_capacity_zero() {
        let mut lfu = LFUCache::new(0);
        lfu.put(1, 1);
        assert_eq!(lfu.get(1), -1);
    }

    #[test]
    fn test_single_capacity() {
        let mut lfu = LFUCache::new(1);
        lfu.put(1, 1);
        assert_eq!(lfu.get(1), 1);
        lfu.put(2, 2);
        assert_eq!(lfu.get(1), -1);
        assert_eq!(lfu.get(2), 2);
    }

    #[test]
    fn test_update_existing_key() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(1, 10);
        assert_eq!(lfu.get(1), 10);
    }

    #[test]
    fn test_frequency_updates() {
        let mut lfu = LFUCache::new(3);
        
        // Add three items
        lfu.put(1, 1);
        lfu.put(2, 2);
        lfu.put(3, 3);
        
        // Access item 1 multiple times to increase its frequency
        lfu.get(1);
        lfu.get(1);
        
        // Access item 2 once
        lfu.get(2);
        
        // Now frequencies are: 1->3, 2->2, 3->1
        // Adding a new item should evict 3 (lowest frequency)
        lfu.put(4, 4);
        
        assert_eq!(lfu.get(3), -1); // 3 should be evicted
        assert_eq!(lfu.get(1), 1);  // 1 should still be there
        assert_eq!(lfu.get(2), 2);  // 2 should still be there
        assert_eq!(lfu.get(4), 4);  // 4 should be there
    }
}