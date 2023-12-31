use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct BloomFilter {
    bit_vector: Vec<bool>,
    size: usize,
    hash_functions: usize,
}

impl BloomFilter {
    fn new(size: usize, hash_functions: usize) -> Self {
        BloomFilter {
            bit_vector: vec![false; size],
            size,
            hash_functions,
        }
    }

    fn hash<T: Hash>(&self, item: &T, seed: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        (hasher.finish() ^ seed) as usize % self.size
    }

    pub fn add<T: Hash>(&mut self, item: &T) {
        for i in 0..self.hash_functions {
            let index = self.hash(item, i as u64);
            self.bit_vector[index] = true;
        }
    }

    pub fn contains<T: Hash>(&self, item: &T) -> bool {
        for i in 0..self.hash_functions {
            let index = self.hash(item, i as u64);
            if !self.bit_vector[index] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let size = 1000;
    let hash_functions = 3;
    let mut bloom_filter = BloomFilter::new(size, hash_functions);

    bloom_filter.add(&"Hello, world!");
    println!("Contains 'Hello, world!': {}", bloom_filter.contains(&"Hello, world!"));  // true
    println!("Contains 'Goodbye, world!': {}", bloom_filter.contains(&"Goodbye, world!"));  // false (probably)
}
