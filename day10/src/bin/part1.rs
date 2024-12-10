use std::collections::{HashSet, VecDeque};
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
    println!("Sum of trailhead scores: {}", result);
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

fn get_neighbors(point: Point, grid: &[Vec<u32>]) -> Vec<Point> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    directions
        .iter()
        .map(|(dr, dc)| Point {
            row: point.row + dr,
            col: point.col + dc,
        })
        .filter(|p| p.row >= 0 && p.row < rows && p.col >= 0 && p.col < cols)
        .collect()
}

fn calculate_trailhead_score(start: Point, grid: &[Vec<u32>]) -> usize {
    let mut reachable_nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Store the current height along with each point
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current, height)) = queue.pop_front() {
        // If we've reached a 9, add it to our set of reachable nines
        if grid[current.row as usize][current.col as usize] == 9 {
            reachable_nines.insert(current);
        }

        // Check all neighbors
        for next in get_neighbors(current, grid) {
            let next_height = grid[next.row as usize][next.col as usize];

            // Only continue if this forms a valid hiking trail
            // (height increases by exactly 1)
            if next_height == height + 1 && !visited.contains(&next) {
                queue.push_back((next, next_height));
                visited.insert(next);
            }
        }
    }

    reachable_nines.len()
}

fn solve(grid: &[Vec<u32>]) -> usize {
    let trailheads = find_trailheads(grid);
    trailheads
        .iter()
        .map(|&start| calculate_trailhead_score(start, grid))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "89010123\n\
                     78121874\n\
                     87430965\n\
                     96549874\n\
                     45678903\n\
                     32019012\n\
                     01329801\n\
                     10456732";

        let grid = parse_input(input);
        assert_eq!(solve(&grid), 36);
    }

    #[test]
    fn test_small_example() {
        let input = "0123\n\
                     1234\n\
                     8765\n\
                     9876";

        let grid = parse_input(input);
        assert_eq!(solve(&grid), 1);
    }
}
