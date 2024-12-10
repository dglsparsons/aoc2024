use std::str::FromStr;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

impl FromStr for Equation {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid equation format".into());
        }

        let test_value = parts[0].trim().parse()?;
        let numbers: Result<Vec<i64>, _> = parts[1].split_whitespace().map(|n| n.parse()).collect();

        Ok(Equation {
            test_value,
            numbers: numbers?,
        })
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

fn evaluate(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        result = operators[i].apply(result, numbers[i + 1]);
    }
    result
}

fn generate_operator_combinations(len: usize) -> Vec<Vec<Operator>> {
    let mut result = Vec::new();
    let total_combinations = 2_i32.pow(len as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(len);
        for j in 0..len {
            if (i >> j) & 1 == 0 {
                combination.push(Operator::Add);
            } else {
                combination.push(Operator::Multiply);
            }
        }
        result.push(combination);
    }
    result
}

fn can_equation_be_true(equation: &Equation) -> bool {
    let operator_count = equation.numbers.len() - 1;
    let combinations = generate_operator_combinations(operator_count);

    combinations
        .iter()
        .any(|ops| evaluate(&equation.numbers, ops) == equation.test_value)
}

fn solve(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let equation = Equation::from_str(line).ok()?;
            if can_equation_be_true(&equation) {
                Some(equation.test_value)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("File to exist");
    let result = solve(&input);
    println!("Total calibration result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        assert_eq!(solve(input), 3749);
    }

    #[test]
    fn test_equation_parsing() {
        let eq = Equation::from_str("190: 10 19").unwrap();
        assert_eq!(eq.test_value, 190);
        assert_eq!(eq.numbers, vec![10, 19]);
    }

    #[test]
    fn test_evaluation() {
        let numbers = vec![10, 19];
        assert_eq!(evaluate(&numbers, &[Operator::Multiply]), 190);
        assert_eq!(evaluate(&numbers, &[Operator::Add]), 29);
    }
}