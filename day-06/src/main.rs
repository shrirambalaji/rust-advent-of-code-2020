use array_tool::vec::*;
use std::{env, fs, collections::HashSet};

// remove duplicate characters from a String
fn dedup_chars(s: String) -> String {
    let mut set: HashSet<char> = HashSet::from(s.chars().collect());
    let deduped = set.drain().collect::<String>();
    return deduped;
}

fn count_answers_by_anyone(input: &str) -> i32 {
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

    sum_of_counts
}

fn count_common_answers(answers: Vec<&str>) -> u32 {
    if answers.is_empty() {
        return 0;
    }

    let starting_answers: Vec<char> = answers[0].chars().collect();
    answers
        .iter()
        .fold(starting_answers, |common_answers, answer_by_person| {
            common_answers.intersect(answer_by_person.chars().collect())
        })
        .len() as u32
}

fn count_answers_by_everyone(input: &str) -> u32 {
    let mut empty_index = 0;
    let mut sum: u32 = 0;
    let lines = input.lines().collect::<Vec<&str>>();

    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            // lines from the previous empty_index till the current index
            let mut until = lines[empty_index..index].to_vec();
            until.retain(|x| !x.is_empty());
            if until.len() == 1 {
                let deduped = dedup_chars(until.join(""));
                sum = sum + deduped.len() as u32;
            } else {
                let count = count_common_answers(until);
                sum = sum + count;
            }
            empty_index = index;
        }
    }

    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    println!(" -- Part 01 -- ");
    let count = count_answers_by_anyone(&input);
    println!("Sum of Counts: {:?}", count);

    println!(" -- Part 02 -- ");
    let count = count_answers_by_everyone(&input);
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
        let sum_of_counts = count_answers_by_anyone(input);
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
        let sum_of_counts = count_answers_by_everyone(input);
        assert_eq!(sum_of_counts, 6);
    }
}
