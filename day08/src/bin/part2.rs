use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Number of unique antinode locations: {}", result);
}

fn solve(input: &str) -> usize {
    // Parse input and group antennas by frequency
    let mut frequency_groups: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                frequency_groups
                    .entry(ch)
                    .or_default()
                    .push(Point::new(x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();
    let max_x = input.lines().next().unwrap().len() as i32;
    let max_y = input.lines().count() as i32;

    // Process each frequency group
    for (freq, antennas) in frequency_groups.iter() {
        // Only process frequencies with at least 2 antennas
        if antennas.len() < 2 {
            continue;
        }

        println!("Processing frequency '{}'", freq);

        // Check each point in the grid
        for y in 0..max_y {
            for x in 0..max_x {
                let p = Point::new(x, y);

                // For each point, check if it's collinear with any pair of antennas
                for i in 0..antennas.len() {
                    for j in (i + 1)..antennas.len() {
                        if are_collinear(&antennas[i], &antennas[j], &p) {
                            antinodes.insert(p);
                            break; // Once we find one collinear pair, we can stop checking this point
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn are_collinear(p1: &Point, p2: &Point, p3: &Point) -> bool {
    // Check if three points are collinear using cross product
    let dx1 = p2.x - p1.x;
    let dy1 = p2.y - p1.y;
    let dx2 = p3.x - p1.x;
    let dy2 = p3.y - p1.y;

    // If cross product is 0, points are collinear
    dx1 * dy2 == dx2 * dy1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_example() {
        let input = "T....#....\n\
                     ...T......\n\
                     .T....#...\n\
                     .........#\n\
                     ..#.......\n\
                     ..........\n\
                     ...#......\n\
                     ..........\n\
                     ....#.....\n\
                     ..........";
        assert_eq!(solve(input), 9);
    }

    #[test]
    fn test_full_example() {
        let input = "............\n\
                     ........0...\n\
                     .....0......\n\
                     .......0....\n\
                     ....0.......\n\
                     ......A.....\n\
                     ............\n\
                     ............\n\
                     ........A...\n\
                     .........A..\n\
                     ............\n\
                     ............";
        assert_eq!(solve(input), 34);
    }
}
