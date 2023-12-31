use bitvec::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

struct BloomFilter<T: Hash + Clone> {
    bit_vector: BitVec,
    size: usize,
    hash_functions: Vec<Box<dyn Fn(&T) -> usize>>,
    items: Vec<T>,
    phantom: PhantomData<T>,
}

impl<T: Hash + Clone> BloomFilter<T> {
    fn new(size: usize, hash_functions: Vec<Box<dyn Fn(&T) -> usize>>) -> Self {
        BloomFilter {
            bit_vector: bitvec![0; size],
            size,
            hash_functions,
            items: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn add(&mut self, item: &T) {
        if self.should_resize() {
            self.resize(self.calculate_new_size()); // Calculate new size based on some strategy
        }

        for hash_function in &self.hash_functions {
            let index = hash_function(item) % self.size;
            self.bit_vector.set(index, true);
        }
        self.items.push(item.clone());
    }

    pub fn contains(&self, item: &T) -> bool {
        self.hash_functions.iter().all(|hash_function| {
            let index = hash_function(item) % self.size;
            self.bit_vector[index]
        })
    }

    fn calculate_false_positive_rate(&self) -> f64 {
        let k = self.hash_functions.len() as f64;
        let m = self.size as f64;
        let n = self.items.len() as f64;
        println!("Calculating FPR with k: {}, n: {}, m: {}", k, n, m);


        (1.0 - (-k * n / m).exp()).powf(k)
    }

    fn should_resize(&self) -> bool {
        // Implement more sophisticated logic based on false positive rate and capacity
        self.items.len() >= self.size
    }

    fn resize(&mut self, new_size: usize) {
        let old_items = self.items.clone();

        self.size = new_size;
        self.bit_vector = bitvec![0; new_size];
        self.items.clear();

        for item in old_items {
            self.add(&item);
        }
    }

    fn calculate_new_size(&self) -> usize {
        // Implement logic to calculate new size based on current load, false positive rate, etc.
        self.size * 2
    }
}

fn default_hash<T: Hash>(item: &T, seed: u64) -> usize {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    (hasher.finish() ^ seed) as usize
}

fn main() {
    let size = 1000;
    let hash_functions: Vec<Box<dyn Fn(&String) -> usize>> = vec![
        Box::new(|item| default_hash(item, 0)),
        Box::new(|item| default_hash(item, 1)),
        Box::new(|item| default_hash(item, 2)),
    ];

    let mut bloom_filter = BloomFilter::new(size, hash_functions);

    // Add items and watch for resizing
    for i in 0..1500 {
        bloom_filter.add(&format!("Item{}", i));
    }

    println!("Current false positive rate: {}", bloom_filter.calculate_false_positive_rate());
    println!("Filter size after resizing: {}", bloom_filter.size);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_bloom_filter() -> BloomFilter<String> {
        let size = 10000;
        let hash_functions: Vec<Box<dyn Fn(&String) -> usize>> = vec![
            Box::new(|item| default_hash(item, 0)),
            Box::new(|item| default_hash(item, 1)),
            Box::new(|item| default_hash(item, 2)),
        ];

        BloomFilter::new(size, hash_functions)
    }




#[test]
fn test_add_and_query() {
    let mut filter = setup_bloom_filter();
    let items = ["item1", "item2", "item3"];

    for &item in &items {
        filter.add(&item.to_string());
        assert!(filter.contains(&item.to_string()), "Item added should be present.");
    }

    assert!(!filter.contains(&"nonexistent".to_string()), "Item not added should not be present.");
}

#[test]
fn test_false_positive_rate_with_trials() {
    let mut average_rate = 0.0;
    let trials = 10;
    for _ in 0..trials {
        let mut filter = setup_bloom_filter();
        let mut false_positives = 0;
        let total_checks = 10000;

        for i in 0..500 {
            filter.add(&format!("item{}", i));
        }

        for i in 500..total_checks {
            if filter.contains(&format!("item{}", i)) {
                false_positives += 1;
            }
        }

        let false_positive_rate = false_positives as f64 / (total_checks - 500) as f64;
        average_rate += false_positive_rate / trials as f64;
    }

    println!("Average false positive rate over {} trials: {}", trials, average_rate);

    let calculated_rate = setup_bloom_filter().calculate_false_positive_rate();
    let tolerance = 0.05; // Adjusted tolerance
    assert!(
        average_rate <= calculated_rate + tolerance,
        "Average false positive rate should be within tolerance of the calculated rate."
    );
}


#[test]
fn test_resize() {
    let mut filter = setup_bloom_filter();
    let initial_size = filter.size;
    let resize_trigger = initial_size + 1;  // Assuming the filter resizes when items_count >= size

    for i in 0..resize_trigger {
        filter.add(&format!("item{}", i));
    }

    assert!(
        filter.size > initial_size,
        "Filter should resize to a larger size when the capacity is exceeded."
    );

    // Check that items added before resizing are still reported as present
    for i in 0..resize_trigger {
        assert!(
            filter.contains(&format!("item{}", i)),
            "Items added before resizing should still be present."
        );
    }
}

#[test]
fn test_empty_filter() {
    let filter = setup_bloom_filter();
    let non_existent_items = ["ghost1", "ghost2", "ghost3"];

    for &item in &non_existent_items {
        assert!(
            !filter.contains(&item.to_string()),
            "Empty filter should not contain any items."
        );
    }
}
}