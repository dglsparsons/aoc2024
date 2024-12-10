use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Instruction {
    operation_type: InstructionType,
    position: usize,
}

#[derive(Debug)]
enum InstructionType {
    Multiply(i32, i32),
    Do,
    Dont,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the input file
    let input = read_to_string("./input.txt")?;

    // Create regex patterns for all instruction types
    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let do_pattern = Regex::new(r"do\(\)")?;
    let dont_pattern = Regex::new(r"don't\(\)")?;

    // Collect all instructions with their positions
    let mut instructions = Vec::new();

    // Find multiplication instructions
    for cap in mul_pattern.captures_iter(&input) {
        let x: i32 = cap[1].parse()?;
        let y: i32 = cap[2].parse()?;
        let pos = cap.get(0).unwrap().start();
        instructions.push(Instruction {
            operation_type: InstructionType::Multiply(x, y),
            position: pos,
        });
    }

    // Find do() instructions
    for m in do_pattern.find_iter(&input) {
        instructions.push(Instruction {
            operation_type: InstructionType::Do,
            position: m.start(),
        });
    }

    // Find don't() instructions
    for m in dont_pattern.find_iter(&input) {
        instructions.push(Instruction {
            operation_type: InstructionType::Dont,
            position: m.start(),
        });
    }

    // Sort instructions by position
    instructions.sort_by_key(|i| i.position);

    // Process instructions in order
    let mut sum = 0;
    let mut multiplications_enabled = true;

    for instruction in instructions {
        match instruction.operation_type {
            InstructionType::Multiply(x, y) => {
                if multiplications_enabled {
                    sum += x * y;
                }
            }
            InstructionType::Do => {
                multiplications_enabled = true;
            }
            InstructionType::Dont => {
                multiplications_enabled = false;
            }
        }
    }

    println!("Sum of enabled multiplications: {}", sum);

    Ok(())
}
