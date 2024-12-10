use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?;
    let safe_count = count_safe_reports(&input);
    println!("Number of safe reports: {}", safe_count);
    Ok(())
}

fn count_safe_reports(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| is_safe_report(line))
        .count()
}

fn is_safe_report(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    if numbers.len() < 2 {
        return false;
    }

    // Check if sequence is strictly increasing or decreasing
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];

        // Check if difference is between 1 and 3 (inclusive)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }

        // If neither increasing nor decreasing, sequence is invalid
        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() {
        let test_cases = [
            ("7 6 4 2 1", true),  // Safe - decreasing
            ("1 2 7 8 9", false), // Unsafe - jump too large
            ("9 7 6 2 1", false), // Unsafe - jump too large
            ("1 3 2 4 5", false), // Unsafe - not monotonic
            ("8 6 4 4 1", false), // Unsafe - no change
            ("1 3 6 7 9", true),  // Safe - increasing
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                is_safe_report(input),
                expected,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_full_example() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";
        assert_eq!(count_safe_reports(input), 2);
    }
}
