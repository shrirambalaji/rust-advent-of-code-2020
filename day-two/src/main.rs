use std::collections::HashMap;
use std::{env, fs};

pub trait ValidatePassword {
    fn validate(&self, password: &str) -> bool;
}

struct OldPasswordRule {
    min: i32,
    max: i32,
    letter: char,
}

impl ValidatePassword for OldPasswordRule {
    fn validate(&self, password: &str) -> bool {
        let mut occurences: HashMap<char, i32> = HashMap::new();

        for character in password.chars() {
            // increments entry by 1, if exists else inserts 1.
            occurences
                .entry(character)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let has_letter = occurences.contains_key(&self.letter);

        if !has_letter {
            return false;
        } else {
            let count = occurences.get(&self.letter).unwrap();
            if count < &self.min {
                return false;
            }
            if count > &self.max {
                return false;
            }
        }
        true
    }
}

struct NewPasswordRule {
    first_position: usize,
    last_position: usize,
    letter: char,
}

impl ValidatePassword for NewPasswordRule {
    fn validate(&self, password: &str) -> bool {
        let mut has_char_at_first_position = false;
        let mut has_char_at_last_position = false;

        match password.chars().nth(self.first_position) {
            Some(char_at_first_position) => {
                has_char_at_first_position = char_at_first_position == self.letter;
            }
            None => {}
        }

        match password.chars().nth(self.last_position) {
            Some(char_at_last_position) => {
                has_char_at_last_position = char_at_last_position == self.letter;
            }
            None => {}
        }

        if has_char_at_first_position && has_char_at_last_position {
            return false;
        } else if has_char_at_first_position && !has_char_at_last_position {
            return true;
        } else if !has_char_at_first_position && has_char_at_last_position {
            return true;
        }

        false
    }
}

#[derive(PartialEq)]
enum PolicyType {
    Old,
    New,
}

fn crop_letters_after(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[..pos],
        None => "",
    }
}
fn read_input_to_vec(filepath: &str) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    let contents = fs::read_to_string(filepath).expect(&format!(
        "Something went wrong file reading the file at {}",
        filepath
    ));
    for line in contents.lines() {
        input.push(line.to_owned())
    }
    return input;
}

fn is_valid(policy_password: &str, policy_type: &PolicyType) -> bool {
    let values: Vec<&str> = policy_password.split(" ").collect::<Vec<&str>>();
    let allowed_password_range = values[0].split("-").collect::<Vec<&str>>();
    let letter = crop_letters_after(&values[1], 1);
    let letter = letter
        .parse::<char>()
        .expect("policy letter should be a char.");

    let password = &values[2];

    match *policy_type {
        PolicyType::Old => {
            let range_min = allowed_password_range[0]
                .parse::<i32>()
                .expect("invalid range min");

            let range_max = allowed_password_range[1]
                .parse::<i32>()
                .expect("invalid range max");

            let policy = OldPasswordRule {
                min: range_min,
                max: range_max,
                letter,
            };

            return policy.validate(password);
        }
        PolicyType::New => {
            let first_position = allowed_password_range[0]
                .parse::<usize>()
                .expect("invalid first position");

            let last_position = allowed_password_range[1]
                .parse::<usize>()
                .expect("invalid last position");

            let policy = NewPasswordRule {
                first_position: first_position - 1,
                last_position: last_position - 1,
                letter,
            };

            return policy.validate(password);
        }
    }
}

fn find_valid_password_count(policies_and_passwords: &Vec<String>, policy_type: PolicyType) -> i32 {
    let mut count = 0;

    for p in policies_and_passwords {
        if is_valid(p, &policy_type) {
            count += 1;
        }
    }

    count
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = match args.get(1) {
        Some(path) => path,
        None => panic!("Input cannot be empty!"),
    };

    let policies_and_passwords = read_input_to_vec(input_path);

    let old_count = find_valid_password_count(&policies_and_passwords, PolicyType::Old);
    println!("Number of valid passwords by Old Policy: {}", old_count);

    let count = find_valid_password_count(&policies_and_passwords, PolicyType::New);
    println!("Number of valid passwords by New Policy: {}", count);
}
