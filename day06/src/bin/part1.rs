use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn parse_input(input: &str) -> ((i32, i32, Direction), Vec<Vec<char>>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start = None;

    for (row, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.trim().chars().collect();
        for (col, &c) in chars.iter().enumerate() {
            if c == '^' {
                start = Some((row as i32, col as i32, Direction::Up));
            }
        }
        grid.push(chars);
    }

    (start.unwrap(), grid)
}

fn is_in_bounds(row: i32, col: i32, grid: &[Vec<char>]) -> bool {
    row >= 0 && (row as usize) < grid.len() && col >= 0 && (col as usize) < grid[0].len()
}

fn simulate_guard_path(start: (i32, i32, Direction), grid: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut row = start.0;
    let mut col = start.1;
    let mut dir = start.2;

    visited.insert((row, col));

    loop {
        let (delta_row, delta_col) = dir.get_delta();
        let next_row = row + delta_row;
        let next_col = col + delta_col;

        if !is_in_bounds(next_row, next_col, grid) {
            // We're about to step out of bounds - stop here
            break;
        }

        if grid[next_row as usize][next_col as usize] == '#' {
            // Hit obstacle - turn right and continue
            dir = dir.turn_right();
        } else {
            // Move forward
            row = next_row;
            col = next_col;
            visited.insert((row, col));
        }
    }

    visited.len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let (start, grid) = parse_input(&input);
    let result = simulate_guard_path(start, &grid);
    println!("The guard visits {} distinct positions.", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let (start, grid) = parse_input(input);
        let result = simulate_guard_path(start, &grid);
        assert_eq!(result, 41);
    }
}
