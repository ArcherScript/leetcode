use std::collections::HashMap;


pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut hm: HashMap<String, i64> = HashMap::new();
        let mut ans = 0;
        
        for rectangle in rectangles {
            let (w,h) = (rectangle[0], rectangle[1]);
            let ratio = w as f64 / h as f64;

            *hm.entry((ratio).to_string())
            .or_insert(0) += 1
        }

        for value in hm.values() {
            ans += (value * (value-1)) / 2 as i64
        }

        println!("{:?}", hm);
        ans as i64
}

fn main() {
    println!("Hello, world!");
}
