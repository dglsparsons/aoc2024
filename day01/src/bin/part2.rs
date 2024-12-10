use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let (left, right) = parse_input(reader)?;
    let similarity_score = calculate_similarity_score(&left, &right);

    println!("Similarity score: {}", similarity_score);

    Ok(())
}

fn parse_input<R: BufRead>(reader: R) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
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

    Ok((left, right))
}

fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i64 {
    // Create frequency map for right list
    let right_freq: HashMap<i32, i32> = right.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    // Calculate similarity score
    left.iter()
        .map(|&num| {
            let freq = right_freq.get(&num).copied().unwrap_or(0);
            i64::from(num) * i64::from(freq)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        let reader = BufReader::new(Cursor::new(input));
        let (left, right) = parse_input(reader).unwrap();
        assert_eq!(calculate_similarity_score(&left, &right), 31);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let reader = BufReader::new(Cursor::new(input));
        let (left, right) = parse_input(reader).unwrap();
        assert_eq!(calculate_similarity_score(&left, &right), 0);
    }

    #[test]
    fn test_no_matches() {
        let input = "1   2\n3   4\n";
        let reader = BufReader::new(Cursor::new(input));
        let (left, right) = parse_input(reader).unwrap();
        assert_eq!(calculate_similarity_score(&left, &right), 0);
    }
}
