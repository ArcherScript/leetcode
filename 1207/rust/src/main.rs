use std::collections::{HashMap, HashSet};


pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut hm = HashMap::new();

    for num in arr {
        *hm.entry(num).or_insert(0) += 1
    }

    let mut hs = HashSet::new();
    for num in hm.values() {
        if hs.insert(num) {
            return false;
        }
    }

   false
}   

fn main() {
    println!("Hello, world!");
}
