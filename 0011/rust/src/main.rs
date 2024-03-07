use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut p_1, mut p_2) = (0, height.len() - 1);
    let mut max: i32 = 0;

    while p_1 < p_2 {
        let h = cmp::min(height[p_2], height[p_1]);
        let w = (p_2 - p_1) as i32;

        max = cmp::max((h * w) as i32, max);
        if height[p_1] < height[p_2] {
            p_1 += 1;
        }else {
            p_2 -= 1;
        }
    }

    max
}

fn main() {
    println!("Hello, world!");
}
