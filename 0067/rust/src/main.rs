pub fn add_binary(a: String, b: String) -> String {
    let len = std::cmp::max(a.len(), b.len());

    // Converting both strings to reverse iterators
    // of the same length, padding them with '0'
    let ai = a.bytes().rev().chain(std::iter::repeat(b'0')).take(len);
    let bi = b.bytes().rev().chain(std::iter::repeat(b'0')).take(len);

    let mut carry = false;
    let mut res: Vec<u8> = ai.zip(bi).map(|(c1, c2)| {
        match (c1, c2, carry) {
            (b'0', b'0', false) => { carry = false; b'0' },
            (b'0', b'1', false) => { carry = false; b'1' },
            (b'1', b'0', false) => { carry = false; b'1' },
            (b'1', b'1', false) => { carry = true;  b'0' },
            (b'0', b'0', true) =>  { carry = false; b'1' },
            (b'0', b'1', true) =>  { carry = true;  b'0' },
            (b'1', b'0', true) =>  { carry = true;  b'0' },
            (b'1', b'1', true) =>  { carry = true;  b'1' },
            _ => unreachable!()
        }
    }).collect();
    if carry { res.push(b'1'); }

    String::from_utf8(res.into_iter().rev().collect()).unwrap()
}

fn main() {
    println!("Hello, world!");
}
