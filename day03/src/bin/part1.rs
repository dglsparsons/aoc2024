use regex::Regex;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the input file
    let input = read_to_string("./input.txt")?;

    // Create a regex pattern for valid mul instructions
    // Matches exactly mul(X,Y) where X and Y are 1-3 digits
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    // Find all valid matches and calculate sum
    let sum: i32 = pattern
        .captures_iter(&input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    println!("Sum of all multiplications: {}", sum);

    Ok(())
}
