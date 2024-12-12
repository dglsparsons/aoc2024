use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Stone {
    val: i64,
    amount: i64,
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let mut stones: Vec<Stone> = input
        .split_whitespace()
        .map(|s| Stone {
            val: s.parse().unwrap(),
            amount: 1,
        })
        .collect();

    for step in 0..75 {
        let mut new_stones = HashMap::new();

        for stone in stones {
            if stone.val == 0 {
                *new_stones.entry(1).or_insert(0) += stone.amount;
            } else if stone.val.to_string().len() % 2 == 0 {
                let s = stone.val.to_string();
                let mid = s.len() / 2;
                let first: i64 = s[..mid].parse().unwrap_or(0);
                let second: i64 = s[mid..].parse().unwrap_or(0);
                *new_stones.entry(first).or_insert(0) += stone.amount;
                *new_stones.entry(second).or_insert(0) += stone.amount;
            } else {
                *new_stones.entry(stone.val * 2024).or_insert(0) += stone.amount;
            }
        }

        stones = new_stones
            .into_iter()
            .map(|(val, amount)| Stone { val, amount })
            .collect();

        println!(
            "Step {}: {} stones",
            step + 1,
            stones.iter().map(|s| s.amount).sum::<i64>()
        );
    }

    let total = stones.iter().map(|s| s.amount).sum::<i64>();
    println!("Final number of stones: {}", total);
}
