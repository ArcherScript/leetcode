use std::collections::HashSet;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let tuple_set: HashSet<(char, char)> = s.chars().zip(t.chars()).collect();
    let s_set: HashSet<char> = s.chars().collect();
    let t_set: HashSet<char> = t.chars().collect();

    tuple_set.len() == s_set.len() && tuple_set.len() == t_set.len()
}

fn main() {
    println!("Hello, world!");
}
