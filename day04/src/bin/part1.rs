use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count = count_xmas_occurrences(&grid);
    println!("XMAS appears {} times", count);
}

fn count_xmas_occurrences(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Define all possible directions to search
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (-1, 1),  // diagonal up-right
    ];

    // Search from each starting position
    for row in 0..rows {
        for col in 0..cols {
            // Try each direction from this position
            for (dx, dy) in directions.iter() {
                if check_xmas(grid, row, col, *dx, *dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_xmas(grid: &[Vec<char>], start_row: usize, start_col: usize, dx: i32, dy: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Check if the word would fit in this direction
    for i in 0..4 {
        let new_row = start_row as i32 + dx * i;
        let new_col = start_col as i32 + dy * i;

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        // Convert back to usize for indexing
        if grid[new_row as usize][new_col as usize] != target[i as usize] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            "MMMSXXMASM".chars().collect(),
            "MSAMXMSMSA".chars().collect(),
            "AMXSXMAAMM".chars().collect(),
            "MSAMASMSMX".chars().collect(),
            "XMASAMXAMM".chars().collect(),
            "XXAMMXXAMA".chars().collect(),
            "SMSMSASXSS".chars().collect(),
            "SAXAMASAAA".chars().collect(),
            "MAMMMXMMMM".chars().collect(),
            "MXMXAXMASX".chars().collect(),
        ];

        assert_eq!(count_xmas_occurrences(&input), 18);
    }
}
