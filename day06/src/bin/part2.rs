use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    row: i32,
    col: i32,
    direction: Direction,
}

fn parse_input(input: &str) -> (State, Vec<Vec<char>>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut guard_state = None;

    for (row, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.trim().chars().collect();
        for (col, &c) in chars.iter().enumerate() {
            if c == '^' {
                guard_state = Some(State {
                    row: row as i32,
                    col: col as i32,
                    direction: Direction::Up,
                });
            }
        }
        grid.push(chars);
    }

    (guard_state.unwrap(), grid)
}

fn is_in_bounds(row: i32, col: i32, grid: &[Vec<char>]) -> bool {
    row >= 0 && (row as usize) < grid.len() && col >= 0 && (col as usize) < grid[0].len()
}

fn check_position_creates_loop(start: State, test_pos: (i32, i32), grid: &[Vec<char>]) -> bool {
    let mut visited_states = HashSet::new();
    let mut path = Vec::new(); // Keep track of the path for verification
    let mut current = start;
    let max_steps = grid.len() * grid[0].len() * 4;
    let mut steps = 0;

    while steps < max_steps {
        steps += 1;

        // Store the current state in path
        path.push(current);

        // Check if we've seen this state before
        if !visited_states.insert(current) {
            // Found a potential loop - verify it's a valid one
            let loop_start_idx = path.iter().position(|&state| state == current).unwrap();

            // Check if any position in the loop would exit the grid
            let loop_segment = &path[loop_start_idx..];
            let mut next_pos;
            for state in loop_segment {
                let (delta_row, delta_col) = state.direction.get_delta();
                next_pos = (state.row + delta_row, state.col + delta_col);

                if !is_in_bounds(next_pos.0, next_pos.1, grid) {
                    return false;
                }
            }
            return true;
        }

        // Calculate next position
        let (delta_row, delta_col) = current.direction.get_delta();
        let next_row = current.row + delta_row;
        let next_col = current.col + delta_col;

        // Check if we would exit the grid
        if !is_in_bounds(next_row, next_col, grid) {
            return false;
        }

        // Check if we hit an obstacle or test position
        if grid[next_row as usize][next_col as usize] == '#'
            || (next_row == test_pos.0 && next_col == test_pos.1)
        {
            // Turn right
            current.direction = current.direction.turn_right();
        } else {
            // Move forward
            current.row = next_row;
            current.col = next_col;
        }
    }

    false
}

fn find_loop_positions(start: State, grid: Vec<Vec<char>>) -> usize {
    // Pre-calculate empty positions to test
    let empty_positions: Vec<(i32, i32)> = (0..grid.len())
        .flat_map(|row| (0..grid[0].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| {
            grid[row][col] == '.' && (row as i32 != start.row || col as i32 != start.col)
        })
        .map(|(row, col)| (row as i32, col as i32))
        .collect();

    // Process positions in parallel
    empty_positions
        .par_iter()
        .filter(|&&pos| check_position_creates_loop(start, pos, &grid))
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let (start, grid) = parse_input(&input);
    let result = find_loop_positions(start, grid);
    println!("Found {} positions that create loops.", result);
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
        let result = find_loop_positions(start, grid);
        assert_eq!(result, 6);
    }
}
