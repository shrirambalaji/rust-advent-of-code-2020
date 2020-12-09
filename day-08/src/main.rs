use std::collections::HashMap;
use std::{env, fs};

// fn crop_letters(s: &str, pos: usize) -> &str {
//     match s.char_indices().nth(pos) {
//         Some((pos, _)) => &s[pos..],
//         None => "",
//     }
// }

fn process_instructions(input: &str) -> Result<i32, String> {
    let mut accumulator = 0;
    let known_operations = vec!["acc", "nop", "jmp"];
    let lines = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let mut processed: HashMap<i32, bool> = HashMap::new();
    let mut curr = 0;

    for (_index, _line) in lines.iter().enumerate() {
        let line = match lines.get(curr as usize) {
            Some(line) => line,
            None => panic!("Invalid index".to_owned()),
        };

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

        if !known_operations.contains(operation) {
            panic!(format!("Unknown operation {}", operation));
        }

        if processed.contains_key(&curr) {
            break;
        } else {
            processed.insert(curr, true);
        }

        curr = curr + 1;
        match *operation {
            "acc" => {
                accumulator = accumulator + argument;
            }
            "jmp" => {
                curr = curr - 1 + argument;
            }
            _ => {}
        }
    }

    Ok(accumulator)
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    // -- Part 01 --
    let accumulator = process_instructions(&input);
    println!("The acccumulator value right before going into an infinite loop: {}", accumulator.unwrap());
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_process_boot_code_instructions() {
        let instructions = r#"
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
        let acc = process_instructions(&instructions);
        assert_eq!(acc, Ok(5))
    }
}
