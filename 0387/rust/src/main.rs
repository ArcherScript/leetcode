pub fn first_uniq_char(s: String) -> i32 {
    unsafe {
        let mut hsh = vec![0; 32];
            
        for c in s.bytes() {
            *hsh.get_unchecked_mut(c as usize & 31) += 1;
        }

        for (i, c) in s.bytes().enumerate() {
            if *hsh.get_unchecked_mut(c as usize & 31) == 1 {return i as i32;}
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}
