use std::cmp::Ordering;


pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let num_potions = potions.len() as i32;
    potions.sort_unstable();

    let mut result = vec![0; spells.len()];

    for (i, spell) in spells.into_iter().enumerate() {
        let left = potions
            .binary_search_by(|&potion| match (potion as i64 * spell as i64).cmp(&success) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Greater,
                Ordering::Greater => Ordering::Greater
            })
            .unwrap_err() as i32;

    result[i] = (num_potions - left) as i32;
    }
    result
}

fn main() {
    println!("Hello, world!");
}
