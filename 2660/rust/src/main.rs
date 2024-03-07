pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
    
    if player1.len() < 3 || player2.len() < 3 {
        let sum1: i32 = player1.iter().sum();
        let sum2: i32 = player2.iter().sum();
        
        // Apply the bonus logic if the first element is 10
        let bonus1 = if player1.get(0) == Some(&10) { sum1 } else { 0 };
        let bonus2 = if player2.get(0) == Some(&10) { sum2 } else { 0 };
        
        return if sum1 + bonus1 > sum2 + bonus2 {
            1
        } else if sum2 + bonus2 > sum1 + bonus1 {
            2
        } else {
            0
        };
    }

    let ans = player1
    .windows(3)
    .zip(player2.windows(3))
    .fold((0, 0), |(mut acc_1, mut acc_2), (ply_1 ,ply_2)| {
        acc_1 += &ply_1.iter().sum();
        acc_2 += &ply_2.iter().sum();

        if ply_1[0] == 10 {
            acc_1 += &ply_1.iter().skip(1).sum();
        }

        if ply_2[0] == 10 {
            acc_2 += &ply_2.iter().skip(1).sum();
        }

        return (acc_1, acc_2);
    });

    if ans.0 > ans.1 {
        return 1;
    } else if ans.1 > ans.0 {
        return 2;
    }

    0 
}

fn main() {
    println!("Hello, world!");
}
