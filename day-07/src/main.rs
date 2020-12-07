#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashMap, env, fs};

// Approach:
// Parse a rule into a HashMap<&str, Vec<(count, &str)>>
// Iterate through all the entries in the map

const NO_COLOR: &str = "no other bags";

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    count: u32,
}

#[derive(Debug)]
struct BaggyColorGraph {
    adjacency_list: HashMap<String, Vec<Bag>>,
}

impl BaggyColorGraph {
    fn new(adjacency_list: HashMap<String, Vec<Bag>>) -> BaggyColorGraph {
        BaggyColorGraph { adjacency_list }
    }

    fn add(&mut self, bag: &Bag) {
        let color = &bag.color;
        self.adjacency_list.insert(color.to_owned(), vec![]);
    }

    fn add_edge(&mut self, bag1: &Bag, bag2: &Bag) {
        match self.adjacency_list.get(&bag1.color) {
            Some(b1_edges) => {
                let color = &bag1.color;
                let other = bag2.clone();
                let mut edges: Vec<Bag> = b1_edges[..].to_vec();
                edges.push(other);
                self.adjacency_list.insert(color.to_owned(), edges);
            }
            None => {}
        }
    }

    fn count_edges_to(&mut self, bag: &Bag) {
        unimplemented!();
    }
}

// The number of values
fn process(input: &str) {
    lazy_static! {
        static ref COLOR_BAG_REGEX: Regex = Regex::new(r"(\d+)\s+(\w.*)bag").unwrap();
    }

    let mut graph = BaggyColorGraph::new(HashMap::new());
    let lines: Vec<&str> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    lines.iter().for_each(|line| {
        let rules: Vec<&str> = line
            .split("bags contain")
            .filter(|x| !x.is_empty())
            .collect();

        if !rules.is_empty() {
            let color = rules[0].trim();
            let outer_bag = Bag {
                color: color.to_owned(),
                count: 1,
            };

            let bag_colors_inside: Vec<String> = rules[1]
                .split(",")
                .map(|r| r.replace(".", "").trim().to_string())
                .filter(|c| c != NO_COLOR)
                .collect();

            if !bag_colors_inside.is_empty() {
                graph.add(&outer_bag);
                bag_colors_inside.iter().for_each(|count_and_color| {
                    let captures = COLOR_BAG_REGEX.captures(count_and_color).unwrap();
                    let count: u32 = captures[1].parse::<u32>().unwrap();
                    let bag_color: &str = &captures[2].trim();
                    let bag = Bag {
                        count,
                        color: bag_color.to_owned(),
                    };
                    graph.add_edge(&outer_bag, &bag);
                })
            }
        }
    });

    println!("{:?}", graph);
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    process(&input);
    println!("Hello, world!");
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_process() {
        let rules = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
        "#;

        process(rules);
    }
}
