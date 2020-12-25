#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{env, fs};

type Ticket = Vec<u32>;

#[derive(Debug, Clone)]
struct RuleRange {
    min: u32,
    max: u32,
}
#[derive(Debug, Clone)]
struct Rule {
    name: String,
    ranges: (RuleRange, RuleRange),
}

impl Rule {
    fn check(&self, ticket_value: u32) -> bool {
        let (range_one, range_two) = &self.ranges;
        ticket_value >= range_one.min && ticket_value <= range_one.max
            || (ticket_value >= range_two.min && ticket_value <= range_two.max)
    }
}

fn parse_rule(rule: &str) -> Rule {
    lazy_static! {
        static ref RULE_REGEX: Regex =
            Regex::new(r"(.*\w+):\s(\d+)-(\d+)\sor\s(\d+)\-(\d+)").unwrap();
    }

    let captures = RULE_REGEX.captures(rule).unwrap();
    let name: String = captures[1].to_string();
    let range_one = RuleRange {
        min: captures[2].parse::<u32>().unwrap(),
        max: captures[3].parse::<u32>().unwrap(),
    };
    let range_two = RuleRange {
        min: captures[4].parse::<u32>().unwrap(),
        max: captures[5].parse::<u32>().unwrap(),
    };

    let ranges = (range_one, range_two);
    Rule { name, ranges }
}

fn parse_ticket(string: &str) -> Ticket {
    string
        .trim()
        .split(',')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect()
}

fn is_valid_ticket_value(rules: &[Rule], ticket_value: u32) -> bool {
    rules.iter().any(|rule| rule.check(ticket_value))
}

fn find_ticket_scanning_error_rate(rules: &[Rule], tickets: &[Ticket]) -> u32 {
    tickets
        .iter()
        .flatten()
        .filter(|ticket| !is_valid_ticket_value(rules, **ticket))
        .copied()
        .sum()
}
#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn parse_input(input: &str) -> Notes {
    let clumps: Vec<&str> = input.trim().split("\n\n").collect();
    let rules: Vec<Rule> = clumps[0].trim().lines().map(parse_rule).collect();
    let my_ticket: Ticket = parse_ticket(clumps[1].trim().lines().nth(1).unwrap());
    let other_tickets: Vec<Ticket> = clumps[2].trim().lines().skip(1).map(parse_ticket).collect();

    Notes {
        rules,
        my_ticket,
        other_tickets,
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let notes = parse_input(&input);
    let nearby = &notes.other_tickets;

    // -- Part 01 --
    let error_rate = find_ticket_scanning_error_rate(&notes.rules, &nearby);
    println!("Ticket Scanning Error Rate: {}", error_rate);
}

#[cfg(test)]
mod tests {
    use super::*;
    /// used to compare vectors of the same type
    fn vec_compare<T: std::cmp::PartialEq>(vec1: &[T], vec2: &[T]) -> bool {
        (vec1.len() == vec2.len()) && vec1.iter().zip(vec2).all(|(a, b)| *a == *b)
    }

    fn get_test_input<'a>() -> &'a str {
        r#"
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"#
    }

    #[test]
    fn should_parse_input() {
        let input = get_test_input();
        let notes = parse_input(input);
        let nearby: Vec<u32> = notes
            .other_tickets
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<u32>>();

        vec_compare(&[7, 1, 14], &notes.my_ticket);
        vec_compare(&[7, 3, 47, 40, 4, 50, 55, 2, 20, 38, 6, 12], &nearby);
    }

    #[test]
    fn should_find_ticket_scanning_error_rate() {
        let input = get_test_input();
        let notes = parse_input(input);
        let nearby = notes.other_tickets;
        assert_eq!(find_ticket_scanning_error_rate(&notes.rules, &nearby), 71);
    }
}
