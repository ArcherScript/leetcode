pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut dict = vec![0 as u8; 32];

        // build the dictionary
        for b in magazine.into_bytes() {
            dict[b as usize % 32] += 1;
        }

        for n in ransom_note.into_bytes() {
            if dict[n as usize % 32] <= 0 {
                return false;
            }else {
                dict[n as usize % 32] -= 1;
            }
        }

        true
}

fn main() {
    println!("Hello, world!");
}
