pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

fn main() {
    println!("Hello, world!");
}
