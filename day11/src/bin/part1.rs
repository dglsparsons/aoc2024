use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let initial_stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut stones = initial_stones;
    for _ in 0..25 {
        stones = transform_stones(stones);
    }

    println!("Number of stones after 25 blinks: {}", stones.len());
}

fn transform_stones(stones: Vec<String>) -> Vec<String> {
    let mut new_stones = Vec::new();

    for stone in stones {
        // Rule 1: If stone is "0", replace with "1"
        if stone == "0" {
            new_stones.push("1".to_string());
            continue;
        }

        // Rule 2: If number has even number of digits, split it
        if stone.len() % 2 == 0 {
            let mid = stone.len() / 2;
            let left = stone[..mid].trim_start_matches('0');
            let right = stone[mid..].trim_start_matches('0');

            // Handle empty strings after trim (all zeros)
            let left = if left.is_empty() { "0" } else { left };
            let right = if right.is_empty() { "0" } else { right };

            new_stones.push(left.to_string());
            new_stones.push(right.to_string());
            continue;
        }

        // Rule 3: Multiply by 2024
        let num = stone.parse::<u64>().unwrap();
        new_stones.push((num * 2024).to_string());
    }

    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_single_blink() {
        let initial = vec![
            "0".to_string(),
            "1".to_string(),
            "10".to_string(),
            "99".to_string(),
            "999".to_string(),
        ];
        let expected = vec![
            "1".to_string(),
            "2024".to_string(),
            "1".to_string(),
            "0".to_string(),
            "9".to_string(),
            "9".to_string(),
            "2021976".to_string(),
        ];
        assert_eq!(transform_stones(initial), expected);
    }

    #[test]
    fn test_example_sequence() {
        let mut stones = vec!["125".to_string(), "17".to_string()];
        stones = transform_stones(stones);
        assert_eq!(stones, vec!["253000", "1", "7"]);

        stones = transform_stones(stones);
        assert_eq!(stones, vec!["253", "0", "2024", "14168"]);
    }
}
