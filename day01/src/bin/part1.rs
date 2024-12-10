use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    // Read input from stdin
    let input = read_to_string("./input.txt")?;

    // Parse input into two vectors
    let (left, right): (Vec<i32>, Vec<i32>) = parse_input(&input);

    // Calculate total distance
    let total_distance = calculate_distance(left, right);

    println!("Total distance: {}", total_distance);

    Ok(())
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(l), Ok(r)) = (numbers[0].parse(), numbers[1].parse()) {
                left.push(l);
                right.push(r);
            }
        }
    }

    (left, right)
}

fn calculate_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    // Sort both lists
    left.sort_unstable();
    right.sort_unstable();

    // Calculate total distance by pairing corresponding elements
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        let (left, right) = parse_input(input);
        assert_eq!(calculate_distance(left, right), 11);
    }
}
