use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let grid = parse_input(&input);
    let result = solve(&grid);
    println!("Sum of trailhead ratings: {}", result);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn find_trailheads(grid: &[Vec<u32>]) -> Vec<Point> {
    let mut trailheads = Vec::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 {
                trailheads.push(Point {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
    }
    trailheads
}

fn get_valid_next_steps(
    point: Point,
    current_height: u32,
    grid: &[Vec<u32>],
    visited: &HashSet<Point>,
) -> Vec<Point> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    directions
        .iter()
        .map(|(dr, dc)| Point {
            row: point.row + dr,
            col: point.col + dc,
        })
        .filter(|p| {
            p.row >= 0
                && p.row < rows
                && p.col >= 0
                && p.col < cols
                && !visited.contains(p)
                && grid[p.row as usize][p.col as usize] == current_height + 1
        })
        .collect()
}

fn count_distinct_trails(current: Point, grid: &[Vec<u32>], visited: &mut HashSet<Point>) -> usize {
    let current_height = grid[current.row as usize][current.col as usize];

    // If we've reached height 9, we've found a valid trail
    if current_height == 9 {
        return 1;
    }

    // Mark current position as visited
    visited.insert(current);

    // Get all valid next steps
    let next_steps = get_valid_next_steps(current, current_height, grid, visited);

    // Count trails from each valid next step
    let total_trails: usize = next_steps
        .iter()
        .map(|&next| {
            let trails = count_distinct_trails(next, grid, visited);
            visited.remove(&next); // Backtrack
            trails
        })
        .sum();

    total_trails
}

fn calculate_trailhead_rating(start: Point, grid: &[Vec<u32>]) -> usize {
    let mut visited = HashSet::new();
    count_distinct_trails(start, grid, &mut visited)
}

fn solve(grid: &[Vec<u32>]) -> usize {
    let trailheads = find_trailheads(grid);
    trailheads
        .iter()
        .map(|&start| calculate_trailhead_rating(start, grid))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_trailhead_three_paths() {
        let input = "\
            .....0.\n\
            ..4321.\n\
            ..5..2.\n\
            ..6543.\n\
            ..7..4.\n\
            ..8765.\n\
            ..9....";
        let grid = parse_input(input);
        let trailheads = find_trailheads(&grid);
        assert_eq!(calculate_trailhead_rating(trailheads[0], &grid), 3);
    }

    #[test]
    fn test_larger_example() {
        let input = "\
            89010123\n\
            78121874\n\
            87430965\n\
            96549874\n\
            45678903\n\
            32019012\n\
            01329801\n\
            10456732";
        let grid = parse_input(input);
        assert_eq!(solve(&grid), 81);
    }

    #[test]
    fn test_complex_example() {
        let input = "\
            012345\n\
            123456\n\
            234567\n\
            345678\n\
            4.6789\n\
            56789.";
        let grid = parse_input(input);
        let trailheads = find_trailheads(&grid);
        assert_eq!(calculate_trailhead_rating(trailheads[0], &grid), 227);
    }
}
