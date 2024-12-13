use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize: (i128, i128),
}

// Chinese Remainder Theorem approach to solve the system
fn solve_system(
    a1: i128,
    b1: i128,
    c1: i128,
    a2: i128,
    b2: i128,
    c2: i128,
) -> Option<(i128, i128)> {
    // Solve a1*x + b1*y = c1 and a2*x + b2*y = c2

    // Calculate determinant
    let det = a1 * b2 - a2 * b1;
    if det == 0 {
        return None;
    }

    // Use Cramer's rule to solve the system
    let x = (c1 * b2 - c2 * b1) / det;
    let y = (a1 * c2 - a2 * c1) / det;

    // Check if the solution consists of integers
    if x * det != (c1 * b2 - c2 * b1) || y * det != (a1 * c2 - a2 * c1) {
        return None;
    }

    // Check if solution is non-negative
    if x >= 0 && y >= 0 {
        Some((x, y))
    } else {
        None
    }
}

impl ClawMachine {
    fn solve(&self) -> Option<i128> {
        let offset = 10_000_000_000_000_i128;
        let target_x = self.prize.0 + offset;
        let target_y = self.prize.1 + offset;

        // Solve the system of equations:
        // a*button_a.0 + b*button_b.0 = target_x
        // a*button_a.1 + b*button_b.1 = target_y
        if let Some((a, b)) = solve_system(
            self.button_a.0,
            self.button_b.0,
            target_x,
            self.button_a.1,
            self.button_b.1,
            target_y,
        ) {
            return Some(3 * a + b);
        }
        None
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut machines = Vec::new();

    for captures in re.captures_iter(input) {
        machines.push(ClawMachine {
            button_a: (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            button_b: (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            prize: (captures[5].parse().unwrap(), captures[6].parse().unwrap()),
        });
    }

    machines
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let machines = parse_input(&input);

    let mut total_tokens = 0;
    let mut solvable_count = 0;

    for (i, machine) in machines.iter().enumerate() {
        if let Some(tokens) = machine.solve() {
            println!("Machine {} is solvable with {} tokens", i + 1, tokens);
            total_tokens += tokens;
            solvable_count += 1;
        }
    }

    println!("\nSolvable machines: {}", solvable_count);
    println!("Total tokens needed: {}", total_tokens);
}

