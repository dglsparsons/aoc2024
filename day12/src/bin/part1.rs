use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        vec![
            Point {
                row: self.row - 1,
                col: self.col,
            }, // up
            Point {
                row: self.row + 1,
                col: self.col,
            }, // down
            Point {
                row: self.row,
                col: self.col - 1,
            }, // left
            Point {
                row: self.row,
                col: self.col + 1,
            }, // right
        ]
    }
}

fn find_region(grid: &[Vec<char>], start: Point, visited: &mut HashSet<Point>) -> (i32, i32) {
    // Returns (area, perimeter)
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let target_char = grid[start.row as usize][start.col as usize];

    let mut queue = VecDeque::new();
    let mut region = HashSet::new();
    queue.push_back(start);
    region.insert(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        for neighbor in current.neighbors() {
            if neighbor.row >= 0 && neighbor.row < rows && neighbor.col >= 0 && neighbor.col < cols
            {
                let neighbor_char = grid[neighbor.row as usize][neighbor.col as usize];
                if !visited.contains(&neighbor) && neighbor_char == target_char {
                    queue.push_back(neighbor);
                    region.insert(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
    }

    let area = region.len() as i32;
    let mut perimeter = 0;

    for point in &region {
        for neighbor in point.neighbors() {
            if neighbor.row < 0
                || neighbor.row >= rows
                || neighbor.col < 0
                || neighbor.col >= cols
                || grid[neighbor.row as usize][neighbor.col as usize] != target_char
            {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}

fn main() {
    // Read input from file
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // Convert input to grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    if grid.is_empty() {
        println!("Empty input!");
        return;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut visited = HashSet::new();
    let mut total_price = 0;

    // Process each cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            let point = Point { row, col };
            if !visited.contains(&point) {
                let (area, perimeter) = find_region(&grid, point, &mut visited);
                let price = area * perimeter;
                total_price += price;
            }
        }
    }

    println!("Total price of fencing: {}", total_price);
}
