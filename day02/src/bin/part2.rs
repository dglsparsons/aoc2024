use std::fs::read_to_string;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = read_to_string("input.txt")?;
    let safe_count = count_safe_reports_with_dampener(&input);
    println!(
        "Number of safe reports with Problem Dampener: {}",
        safe_count
    );
    Ok(())
}

fn count_safe_reports_with_dampener(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| is_safe_report_with_dampener(line))
        .count()
}

fn is_safe_sequence(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

fn is_safe_report_with_dampener(line: &str) -> bool {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    // First check if it's safe without removing any number
    if is_safe_sequence(&numbers) {
        return true;
    }

    // Try removing each number one at a time
    for i in 0..numbers.len() {
        let mut modified_numbers = numbers.clone();
        modified_numbers.remove(i);

        if is_safe_sequence(&modified_numbers) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases_with_dampener() {
        let test_cases = [
            ("7 6 4 2 1", true),  // Safe without removing any level
            ("1 2 7 8 9", false), // Unsafe regardless of removal
            ("9 7 6 2 1", false), // Unsafe regardless of removal
            ("1 3 2 4 5", true),  // Safe by removing 3
            ("8 6 4 4 1", true),  // Safe by removing one 4
            ("1 3 6 7 9", true),  // Safe without removing any level
        ];

        for (input, expected) in test_cases {
            assert_eq!(
                is_safe_report_with_dampener(input),
                expected,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_full_example_with_dampener() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";
        assert_eq!(count_safe_reports_with_dampener(input), 4);
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_safe_report_with_dampener("1")); // Too short
        assert!(!is_safe_report_with_dampener("")); // Empty
        assert!(is_safe_report_with_dampener("1 2")); // Minimal valid case
    }
}
