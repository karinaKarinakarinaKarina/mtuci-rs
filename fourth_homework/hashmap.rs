use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V> {
    fn new() -> Self {
        let bucket_count = 16; // количество "ведер"
        let mut buckets = Vec::with_capacity(bucket_count);
        for _ in 0..bucket_count {
            buckets.push(Vec::new());
        }
        
        HashMap {
            buckets,
            size: 0,
        }
    }
    
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
    
    fn insert(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        let bucket_index = (hash % self.buckets.len() as u64) as usize;
        
        for item in &mut self.buckets[bucket_index] {
            if item.0 == key {
                item.1 = value; // обновить значение, если ключ уже существует
                return;
            }
        }
        
        self.buckets[bucket_index].push((key, value));
        self.size += 1;
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let bucket_index = (hash % self.buckets.len() as u64) as usize;
        
        for item in &self.buckets[bucket_index] {
            if item.0 == *key {
                return Some(&item.1);
            }
        }
        
        None
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let bucket_index = (hash % self.buckets.len() as u64) as usize;
        
        let bucket = &mut self.buckets[bucket_index];
        for i in 0..bucket.len() {
            if bucket[i].0 == *key {
                self.size -= 1;
                return Some(bucket.remove(i).1);
            }
        }
        
        None
    }
}

fn main() {
    let mut map: HashMap<u32, &str> = HashMap::new();
    
    map.insert(1, "Hello");
    map.insert(2, "World");
    
    println!("{:?}", map.get(&1)); // Some("Hello")
    println!("{:?}", map.get(&2)); // Some("World")
    
    map.remove(&1);
    
    println!("{:?}", map.get(&1)); // None
    println!("{:?}", map.get(&2)); // Some("World")
}
