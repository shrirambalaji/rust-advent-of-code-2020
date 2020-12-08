#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{collections::HashMap, env, fs};

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

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.adjacency_list.len()
    }

    fn add_vertex(&mut self, bag: &Bag) {
        let color = &bag.color;
        self.adjacency_list.insert(color.to_owned(), vec![]);
    }

    fn add_edge(&mut self, bag1: &Bag, bag2: &Bag) {
        let color = bag1.color.to_string();
        let bag2 = bag2.clone();

        if bag2.color != NO_COLOR {
            self.adjacency_list
                .entry(color)
                .and_modify(|e| e.push(bag2))
                .or_insert(vec![]);
        } else {
            self.adjacency_list.entry(color).or_insert(vec![]);
        }
    }

    fn count_bags_inside(&self, color: &str, mut count: u32) -> u32 {
        let curr = count;
        if let Some(bags) = self.adjacency_list.get(color) {
            for bag in bags.iter() {
                count += bag.count + (bag.count * self.count_bags_inside(&bag.color, curr));
            }
        }

        count
    }

    fn dfs(&self, source: &str, visited: &mut HashMap<String, bool>) {
        let mut stack: Vec<&str> = Vec::new();
        stack.push(source);
        visited.insert(source.to_owned(), true);

        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            if self.adjacency_list.contains_key(node) {
                let bags = self.adjacency_list.get(node).unwrap();
                for bag in bags {
                    if !(visited.contains_key(&bag.color)) {
                        stack.push(&bag.color);
                        let neighbor = bag.color.to_owned();
                        visited.insert(neighbor, true);
                    }
                }
            }
        }
    }

    fn has_edge(&self, source: &str, destination: &str) -> bool {
        let mut visited: HashMap<String, bool> = HashMap::new();
        self.dfs(source, &mut visited);
        visited.contains_key(destination) && visited.get(destination).unwrap() == &true
    }

    fn count_edges_to(&mut self, color_to_find: &str) -> u32 {
        let count = self.adjacency_list.keys().fold(0, |acc, color| {
            if color != color_to_find {
                if self.has_edge(color, color_to_find) {
                    return acc + 1;
                }
            }
            acc
        });
        count
    }

    #[allow(dead_code)]
    fn print_colors(&mut self) {
        for (color, bags_inside) in self.adjacency_list.iter() {
            println!(
                "{} => {:?}",
                color,
                bags_inside
                    .iter()
                    .map(|x| x.color.clone())
                    .collect::<Vec<String>>()
            );
        }
    }
}

fn create_graph(input: &str) -> BaggyColorGraph {
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
                .collect();

            if !bag_colors_inside.is_empty() {
                graph.add_vertex(&outer_bag);
                bag_colors_inside.iter().for_each(|count_and_color| {
                    if !count_and_color.contains(NO_COLOR) {
                        let captures = COLOR_BAG_REGEX.captures(count_and_color).unwrap();
                        let count: u32 = captures[1].parse::<u32>().unwrap();
                        let bag_color: &str = &captures[2].trim();
                        let bag = Bag {
                            count,
                            color: bag_color.to_owned(),
                        };
                        graph.add_edge(&outer_bag, &bag);
                    } else {
                        let bag = Bag {
                            count: 0,
                            color: NO_COLOR.to_string(),
                        };
                        graph.add_edge(&outer_bag, &bag);
                    }
                })
            }
        }
    });

    return graph;
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    let mut graph = create_graph(&input);
    let bag_color = "shiny gold";
    let count = graph.count_edges_to(bag_color);
    if count > 0 {
        println!("Number of bags which can contain {}: {}", bag_color, count)
    } else {
        println!("No bags contain the {}", bag_color)
    }

    let bags_inside = graph.count_bags_inside("shiny gold", 0);
    println!("{} can contain {} other bags", bag_color, bags_inside);
}

#[cfg(test)]

mod tests {
    use super::*;

    fn get_rules() -> String {
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
        return rules.to_owned();
    }

    #[test]
    fn should_create_graph() {
        let graph = create_graph(&get_rules());
        assert_eq!(graph.len(), 9);
    }

    #[test]
    fn should_count_edges_to_color_in_graph() {
        let mut graph = create_graph(&get_rules());
        assert_eq!(graph.count_edges_to("shiny gold"), 4);
    }

    #[test]
    fn should_contain_bags_inside() {
        let graph = create_graph(&get_rules());
        // let visited = HashMap::new();
        let bags_inside = graph.count_bags_inside("shiny gold", 0);
        assert_eq!(bags_inside, 32);
    }
}
