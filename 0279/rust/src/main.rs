use std::cmp;

pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![n; n as usize + 1];

    dp[0] = 0;

    for target in 1..=n {
        for s in 1..=target {
            if target - s.pow(2) < 0 {
                break;
            }
            dp[target as usize] = cmp::min(dp[target as usize], 1 + dp[target as usize - s.pow(2) as usize])
        }
    }

    dp[n as usize]
}

fn main() {
    
}
