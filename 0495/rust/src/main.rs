pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    time_series
    .windows(2)
    .fold(duration, |acc, pair| acc + i32::min(duration, pair[1] - pair[0]))
}

fn main() {
    println!("Hello, world!");
}
