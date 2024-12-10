use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let count = count_xmas_patterns(&grid);
    println!("X-MAS appears {} times", count);
}

fn count_xmas_patterns(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // For each possible center point of the X
    for row in 1..rows - 1 {
        // Need room for top and bottom
        for col in 1..cols - 1 {
            // Need room for left and right
            if grid[row][col] == 'A' {
                // Center must be 'A'
                count += check_x_pattern(grid, row, col);
            }
        }
    }

    count
}

fn check_x_pattern(grid: &[Vec<char>], center_row: usize, center_col: usize) -> usize {
    let mut valid_patterns = 0;

    // Check all possible combinations of forward/backward MAS on each diagonal
    let top_left = [
        grid[center_row - 1][center_col - 1],
        grid[center_row][center_col],
        grid[center_row + 1][center_col + 1],
    ];
    let top_right = [
        grid[center_row - 1][center_col + 1],
        grid[center_row][center_col],
        grid[center_row + 1][center_col - 1],
    ];

    // For each diagonal, check both forward and backward MAS
    let valid_top_left_forward = check_mas(&top_left, false);
    let valid_top_left_backward = check_mas(&top_left, true);
    let valid_top_right_forward = check_mas(&top_right, false);
    let valid_top_right_backward = check_mas(&top_right, true);

    // Count valid combinations
    if valid_top_left_forward {
        if valid_top_right_forward {
            valid_patterns += 1;
        }
        if valid_top_right_backward {
            valid_patterns += 1;
        }
    }
    if valid_top_left_backward {
        if valid_top_right_forward {
            valid_patterns += 1;
        }
        if valid_top_right_backward {
            valid_patterns += 1;
        }
    }

    valid_patterns
}

fn check_mas(chars: &[char], backward: bool) -> bool {
    if backward {
        chars[0] == 'S' && chars[1] == 'A' && chars[2] == 'M'
    } else {
        chars[0] == 'M' && chars[1] == 'A' && chars[2] == 'S'
    }
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

        let count = count_xmas_patterns(&input);
        println!("Found {} patterns", count); // For debugging
        assert_eq!(count, 9); // Based on the example output
    }
}
