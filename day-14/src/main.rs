#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::{env, fs};

#[derive(Debug)]
enum Command<'a> {
    Malloc(u64, u64),
    Mask(&'a str),
}

fn parse(input: &str) -> Vec<Command> {
    lazy_static! {
        static ref MEM_REGEX: Regex = Regex::new(r"mem\[(\d+)\]").unwrap();
    }

    let mut commands = Vec::new();
    let lines: Vec<&str> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    for line in lines {
        let (command_name, command_value): (&str, &str) =
            line.splitn(2, " = ").collect_tuple().unwrap();

        if command_name.starts_with("mask") {
            commands.push(Command::Mask(command_value))
        } else if command_name.starts_with("mem") {
            let captures = MEM_REGEX
                .captures(command_name)
                .expect("invalid memory allocation command");

            let address: u64 = captures[1].parse::<u64>().expect("invalid memory address");
            let value: u64 = command_value
                .parse::<u64>()
                .expect("invalid value to write in memory address");

            commands.push(Command::Malloc(address, value));
        }
    }

    commands
}

fn apply_mask(mask: &str, value: u64) -> u64 {
    let mut bits: Vec<char> = format!("{:036b}", value).chars().collect();

    for (index, mask_bit) in mask.chars().enumerate() {
        if mask_bit == '0' || mask_bit == '1' {
            bits[index] = mask_bit;
        }
    }

    let masked_value: String = bits.into_iter().collect::<String>();
    u64::from_str_radix(&masked_value, 2).unwrap()
}

fn execute(commands: Vec<Command>) -> HashMap<u64, u64> {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut bitmask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    for command in commands {
        match command {
            Command::Mask(value) => bitmask = value,
            Command::Malloc(address, value) => {
                let masked = apply_mask(bitmask, value);
                memory.insert(address, masked);
            }
        }
    }

    memory
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let commands = parse(&input);
    let memory = execute(commands);

    let sum: u64 = memory.values().sum();
    println!("Sum of all values left in the memory: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_input<'a>() -> &'a str {
        r#"
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
        "#
    }

    #[test]
    fn should_find_sum_of_values_in_memory() {
        let input = read_input();
        let commands = parse(input);
        let memory = execute(commands);

        let sum: u64 = memory.values().sum();
        assert_eq!(165, sum);
    }
}
