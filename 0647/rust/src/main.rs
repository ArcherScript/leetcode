pub fn is_palindrome(s: String) -> bool {
    s.chars().rev().collect::<String>() == s
}

pub fn generate_combinations(so_far: &str, rest: &str) -> i32{
    let mut ans = 0;
    if rest.is_empty() {

    }else {
        if is_palindrome(so_far.to_string()) || so_far.len {
            ans += 1;
        }

        generate_combinations(&format!("{}{}",  so_far, &rest[0..1]), &rest[1..]);
        generate_combinations(so_far, &rest[1..]);
    }

    ans
}

pub fn count_substrings(s: String) -> i32 {
    generate_combinations("", &s)
}

fn main() {
    println!("Hello, world!");
}
