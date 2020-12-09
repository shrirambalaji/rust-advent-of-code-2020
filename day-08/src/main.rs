use std::collections::HashSet;
use std::{env, fs};

fn parse_instruction(line: &str) -> (&str, i32) {
    let instruction = line.split_whitespace().collect::<Vec<&str>>();
    let operation = match instruction.get(0) {
        Some(operation) => operation,
        None => {
            panic!("Invalid operation")
        }
    };

    let argument = match instruction.get(1) {
        Some(argument) => argument.parse::<i32>().unwrap(),
        None => {
            panic!("Invalid argument");
        }
    };

    (operation, argument)
}

fn parse(input: &str) -> Vec<(&str, i32)> {
    let lines = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(parse_instruction)
        .collect::<Vec<(&str, i32)>>();

    lines
}

fn process_instructions(instructions: &Vec<(&str, i32)>) -> Result<i32, i32> {
    let mut processed: HashSet<isize> = HashSet::new();
    let mut accumulator = 0;
    let mut curr: isize = 0;

    loop {
        // bounds check
        if curr > instructions.len() as isize || curr < 0 {
            panic!("Invalid index. Out of bounds of Instruction Set");
        }

        // if the instruction has already been processed, then its an infinite loop. So break with an error, with the acc's value
        if processed.contains(&curr) {
            break Err(accumulator);
        }

        // we have reached the end of the bootcode. so the program can terminate.
        if curr == instructions.len() as isize {
            break Ok(accumulator);
        }

        processed.insert(curr);
        match instructions[curr as usize] {
            ("acc", argument) => {
                curr = curr + 1;
                accumulator = accumulator + argument;
            }
            ("jmp", argument) => {
                // we shouldn't increment the current index during a jump, so we decrement it by 1, before adding the argument.
                curr = curr + argument as isize;
            }
            ("nop", _) => {
                curr = curr + 1;
            }

            _ => {}
        }
    }
}

fn fix_bootcode_by_swap(instructions: &Vec<(&str, i32)>) -> i32 {
    // iterate through all instructions
    // swap out a single nop -> jmp, and a jmp -> nop
    // if the program is able to terminate sucssefully, we get an Ok(acc) with the accumulator value.
    for (index, &instruction) in instructions.iter().enumerate() {
        match instruction {
            ("acc", _) => continue,
            ("nop", val) => {
                let mut instructions = instructions.clone();
                instructions[index] = ("jmp", val);
                if let Ok(accumulator) = process_instructions(&instructions) {
                    return accumulator;
                }
            }
            ("jmp", val) => {
                let mut instructions = instructions.clone();
                instructions[index] = ("nop", val);
                if let Ok(accumulator) = process_instructions(&instructions) {
                    return accumulator;
                }
            }
            _ => continue,
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    // -- Part 01 --
    let instructions = parse(&input);
    let accumulator = process_instructions(&instructions);
    println!(
        "The accumulator value right before going into an infinite loop: {}",
        accumulator.unwrap_err()
    );

    // -- Part 02 --
    let accumulator = fix_bootcode_by_swap(&instructions);
    println!(
        "The accumulator value after the program terminates is: {}",
        accumulator
    );
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_return_acc_before_entering_infinite_loop() {
        let bootcode = r#"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
       "#;
        let instructions = parse(bootcode);
        let acc = process_instructions(&instructions);
        assert_eq!(acc, Err(5))
    }
}
