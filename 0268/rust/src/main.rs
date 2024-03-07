use std::collections::HashSet;

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let map = nums.iter().collect::<HashSet<_>>();

    for num in 1..=nums.len() as i32 {
        if !map.contains(&num) {
            return num;
        }
    }

    return 0;
}

fn main() {
    println!("Hello, world!");
}
