use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64), // (X, Y) movement for button A
    button_b: (i64, i64), // (X, Y) movement for button B
    prize: (i64, i64),    // (X, Y) position of prize
}

impl ClawMachine {
    // Returns Some(tokens) if prize is winnable, None if impossible
    fn solve(&self) -> Option<i64> {
        // Try all combinations of button presses up to 100 each
        for a in 0..=100 {
            for b in 0..=100 {
                let x = a * self.button_a.0 + b * self.button_b.0;
                let y = a * self.button_a.1 + b * self.button_b.1;

                if x == self.prize.0 && y == self.prize.1 {
                    // Calculate total tokens: 3 per A press, 1 per B press
                    return Some(3 * a + b);
                }
            }
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
    // Read input file
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let machines = parse_input(&input);

    // Calculate total tokens needed for all winnable prizes
    let total_tokens: i64 = machines.iter().filter_map(|machine| machine.solve()).sum();

    println!("Total tokens needed: {}", total_tokens);
}
