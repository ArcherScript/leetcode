pub fn first_palindrome(words: Vec<String>) -> String {
    for w in words {
        if w.chars().eq(w.chars().rev()) {
            return w;
        }
    }
    return "".to_string();
}

fn main() {
    
}
