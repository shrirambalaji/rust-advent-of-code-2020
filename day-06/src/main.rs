use std::collections::{HashMap, HashSet};
use std::{env, fs};

#[allow(dead_code)]
fn get_repeated_letter_count(s: &str) -> usize {
    let mut freq: HashMap<char, i32> = HashMap::new();
    s.chars().for_each(|letter| {
        freq.entry(letter).and_modify(|e| *e = *e + 1).or_insert(1);
    });

    freq.iter().filter(|(_letter, count)| *count > &1).count()
}

// remove duplicate characters from a String
fn dedup_chars(s: String) -> String {
    let mut set: HashSet<char> = HashSet::from(s.chars().collect());
    let deduped = set.drain().collect::<String>();
    return deduped;
}

fn get_count_for_answers_by_anyone(input: &str) -> i32 {
    let mut answered: Vec<String> = Vec::new();
    let mut empty_index = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            // lines from the previous empty_index till the current index
            let until = lines[empty_index..index].to_vec();
            let deduped = dedup_chars(until.join(""));
            if !deduped.is_empty() {
                answered.push(deduped);
            }

            empty_index = index;
        }
    }

    let sum_of_counts: i32 = answered
        .iter()
        .fold(0, |acc, group| acc + group.len() as i32);

    return sum_of_counts;
}

fn get_count_for_answers_by_everyone(input: &str) -> i32 {
    let mut empty_index = 0;
    let mut sum = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            // lines from the previous empty_index till the current index
            let mut until = lines[empty_index..index].to_vec();
            until.retain(|x| !x.is_empty());
            if until.len() == 1 {
                let deduped = dedup_chars(until.join(""));
                sum = sum + deduped.len();
            } else {
                // TODO: For groups with multilines, we need to check the number of common characters, and multiple that by `until.len()` which will be our count.
            }
            empty_index = index;
        }
    }

    return sum as i32;
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    println!(" -- Part 01 -- ");
    let count = get_count_for_answers_by_anyone(&input);
    println!("Sum of Counts: {:?}", count);

    println!(" -- Part 02 -- ");
    let count = get_count_for_answers_by_everyone(&input);
    println!("Sum of Counts: {:?}", count);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn should_get_count_for_answered_by_anyone() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b

        "#;
        let sum_of_counts = get_count_for_answers_by_anyone(input);
        assert_eq!(sum_of_counts, 11);
    }

    #[test]
    fn should_get_count_for_answered_by_everyone() {
        let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b

        "#;
        let sum_of_counts = get_count_for_answers_by_everyone(input);
        assert_eq!(sum_of_counts, 6);
    }
}
