pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
      match nums.binary_search(&target) {
        Ok(n) => n as i32,
        Err(n) => n as i32
      }
}

fn main() {
    println!("Hello, world!");
}
