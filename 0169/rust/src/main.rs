pub fn majority_element(nums: Vec<i32>) -> i32 {
       let (mut candidate, mut votes) = (-1, 0);

       for num in nums {
            if votes == 0 {
                candidate = num;
                votes = 1;
            }else {
                if num == candidate {
                    votes += 1;
                }else {
                    votes -= 1;
                }
            }
       }

       candidate
}

fn main() {
    println!("Hello, world!");
}
    