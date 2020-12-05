#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::{env, fs};


// Day 04
// Given a batch of lines, indicated as passports validated if the passports have the necessary fields, and if the field-values are valid.

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn validate_field(field: &str, value: &str) -> bool {
    lazy_static! {
        static ref HEIGHT_REGEX: Regex = Regex::new(r"(\d+)(\w+)").unwrap();
    }

    let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    match field {
        "byr" => {
            value.len() == 4 && {
                let year = value.parse::<i32>().unwrap();
                return year >= 1920 && year <= 2002;
            }
        }
        "iyr" => {
            value.len() == 4 && {
                let year = value.parse::<i32>().unwrap();
                return year >= 2010 && year <= 2020;
            }
        }
        "eyr" => {
            value.len() == 4 && {
                let year = value.parse::<i32>().unwrap();
                return year >= 2020 && year <= 2030;
            }
        }
        "hgt" => {
            let captures = HEIGHT_REGEX.captures(value).unwrap();
            let height: i32 = captures[1].parse::<i32>().unwrap();
            let unit: &str = &captures[2];

            match unit {
                "in" => height >= 59 && height <= 76,
                "cm" => height >= 150 && height <= 193,
                _ => false,
            }
        }
        "hcl" => {
            if !value.starts_with("#") {
                return false;
            }

            // Remove # from start
            let value = crop_letters(value, 1);
            value.len() == 6
                && value.chars().all(|x| {
                    if x.is_alphabetic() {
                        return x <= 'f';
                    }
                    return x.is_digit(10);
                })
        }
        "ecl" => eye_colors.contains(&value.trim()),
        "pid" => value.len() == 9,
        _ => true,
    }
}

fn validate_passport(passport: &str, should_validate_fields: bool) -> bool {
    let passport_fields: Vec<&str> = passport.split_whitespace().collect();
    let mut required_field_map: HashMap<&str, (i32, &str)> = [
        ("byr", (0, "")),
        ("iyr", (0, "")),
        ("eyr", (0, "")),
        ("hgt", (0, "")),
        ("hcl", (0, "")),
        ("ecl", (0, "")),
        ("pid", (0, "")),
    ]
    .iter()
    .cloned()
    .collect();

    for passport_field in passport_fields.iter() {
        let passport_field_vec = passport_field.split(':').collect::<Vec<&str>>();
        let field = passport_field_vec.get(0).unwrap();
        let value = passport_field_vec.get(1).unwrap();
        required_field_map.entry(field).and_modify(|e| {
            e.0 = 1;
            e.1 = value;
        });
    }

    required_field_map.iter().all(|(field, (count, value))| {
        if !should_validate_fields {
            return count > &0;
        }
        return count > &0 && validate_field(field, value);
    })
}

fn process(input: &str, should_validate_fields: bool) -> i32 {
    let mut passports: Vec<String> = Vec::new();
    let mut empty_index = 0;
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            let mut until = lines[empty_index..index].to_vec();
            until.retain(|x| !x.is_empty());
            passports.push(until.join(" "));
            empty_index = index;
        }
    }

    passports.retain(|x| !x.is_empty());

    let count = passports
        .iter()
        .filter(|x| validate_passport(x, should_validate_fields))
        .count();

    count as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("Input file cannot be empty!");
    let input = fs::read_to_string(filepath).expect("Something went wrong while reading input");

    // -- Part 01 --
    let valid = process(&input, false);
    println!("Number of valid passports: {}", valid);

    // -- Part 02 --
    let valid = process(&input, true);
    println!("Number of valid passports after stricter validation: {}", valid);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_validate_passports_without_validating_fields() {
        let input = r#"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in

        "#;

        assert_eq!(process(input, false), 2);
    }

    #[test]
    fn should_validate_passports_with_fields() {
        let input = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

        "#;

        assert_eq!(process(input, true), 4);
    }

    #[test]

    fn should_validate_fields() {
        assert!(validate_field("byr", "2002"));
        assert_eq!(validate_field("byr", "2003"), false);

        assert_eq!(validate_field("hgt", "60in"), true);
        assert_eq!(validate_field("hgt", "190cm"), true);

        assert_eq!(validate_field("hgt", "190in"), false);
        assert_eq!(validate_field("hgt", "190"), false);

        assert_eq!(validate_field("hcl", "#123abc"), true);
        assert_eq!(validate_field("hcl", "#123abz"), false);
        assert_eq!(validate_field("hcl", "123abc"), false);

        assert_eq!(validate_field("ecl", "brn"), true);
        assert_eq!(validate_field("ecl", "wat"), false);

        assert_eq!(validate_field("pid", "000000001"), true);
        assert_eq!(validate_field("pid", "0123456789"), false);
    }
}
