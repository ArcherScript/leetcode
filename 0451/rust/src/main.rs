use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut freq: HashMap<char, i32> = HashMap::with_capacity(s.len());

    for c in s.chars(){
        *freq.entry(c)
        .or_insert(0) += 1
    }
    let mut res = "".to_string();
    let mut vec:Vec<_> = freq.iter().collect();
    vec.sort_by(|a, b| a.1.cmp(b.1));

    for (x, y) in vec {
        res += &x.to_string().repeat(*y as usize);
    }

    res
}

fn main() {
    println!("Hello, world!");
}
