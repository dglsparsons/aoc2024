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

    fn squared_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
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
        println!("Processing frequency '{}'", freq);
        // Check all pairs of antennas with the same frequency
        for i in 0..antennas.len() {
            for j in (i + 1)..antennas.len() {
                let a1 = &antennas[i];
                let a2 = &antennas[j];

                // Check each potential point in the grid
                for y in 0..max_y {
                    for x in 0..max_x {
                        let p = Point::new(x, y);

                        // Calculate squared distances
                        let d1 = p.squared_distance(a1);
                        let d2 = p.squared_distance(a2);

                        // Check if point satisfies both conditions:
                        // 1. One antenna is exactly twice as far as the other (squared distances in 4:1 ratio)
                        // 2. Point is collinear with both antennas
                        if (d1 == 4 * d2 || d2 == 4 * d1) && are_collinear(a1, a2, &p) {
                            // Check if point lies between antennas or outside them
                            if lies_between(a1, a2, &p) {
                                // Skip points that lie between the antennas
                                continue;
                            }
                            antinodes.insert(p);
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

fn lies_between(p1: &Point, p2: &Point, p3: &Point) -> bool {
    // Check if p3 lies between p1 and p2
    // First confirm points are collinear
    if !are_collinear(p1, p2, p3) {
        return false;
    }

    // Check if point lies within the bounding box of the two antennas
    p3.x >= p1.x.min(p2.x)
        && p3.x <= p1.x.max(p2.x)
        && p3.y >= p1.y.min(p2.y)
        && p3.y <= p1.y.max(p2.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let input = "...........\n\
                     ...#.......\n\
                     ..........\n\
                     ....a.....\n\
                     ..........\n\
                     .....a....\n\
                     ..........\n\
                     ......#...\n\
                     ..........\n\
                     ..........";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_example() {
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
        assert_eq!(solve(input), 14);
    }
}
