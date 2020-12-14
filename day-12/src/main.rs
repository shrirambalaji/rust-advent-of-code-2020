#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{env, fs};

#[derive(PartialEq, Clone, Debug)]
enum Directions {
    North,
    East,
    West,
    South,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct NavigationInstruction {
    direction: Directions,
    units: u32,
}

#[derive(Debug, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

fn parse_as_instruction(line: &str) -> NavigationInstruction {
    lazy_static! {
        static ref NAV_REGEX: Regex = Regex::new(r"(\w{1})(\d+)").unwrap();
    }

    let captures = NAV_REGEX.captures(line).unwrap();

    let direction_str = &captures[1];
    let units_str = &captures[2];

    let direction = match direction_str {
        "N" => Directions::North,
        "E" => Directions::East,
        "W" => Directions::West,
        "S" => Directions::South,
        "L" => Directions::Left,
        "R" => Directions::Right,
        "F" => Directions::Forward,
        _ => {
            panic!("Invalid instruction Direction. Should be one of N, E, W, S, L, R, F")
        }
    };

    let units = &units_str
        .parse::<u32>()
        .expect("Invalid instruction units. Units should be a number");

    return NavigationInstruction {
        direction,
        units: *units,
    };
}

fn parse(input: &str) -> Vec<NavigationInstruction> {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(parse_as_instruction)
        .collect()
}

fn maybe_turn(direction: &Directions, turn: &Directions, angle: &u32) -> Option<Directions> {
    let opposite = || {
        if *direction == Directions::North {
            Directions::South
        } else if *direction == Directions::South {
            Directions::North
        } else if *direction == Directions::West {
            Directions::East
        } else if *direction == Directions::East {
            Directions::West
        } else {
            panic!("Unknown Direction")
        }
    };

    let at_right_angles = || match *turn {
        Directions::Left => {
            if *direction == Directions::North || *direction == Directions::South {
                Directions::West
            } else if *direction == Directions::East {
                Directions::North
            } else if *direction == Directions::West {
                Directions::South
            } else {
                panic!("Unknown Turn Direction")
            }
        }
        Directions::Right => {
            if *direction == Directions::North || *direction == Directions::South {
                Directions::East
            } else if *direction == Directions::East {
                Directions::South
            } else if *direction == Directions::West {
                Directions::North
            } else {
                panic!("Unknown Turn Direction")
            }
        }
        _ => panic!("Unknown Turn Direction"),
    };

    match *angle {
        90 => Some(at_right_angles()),
        180 => Some(opposite()),
        _ => Some(direction.clone()),
    }
}

fn travel_along(coordinates: &mut Coordinates, direction: &Directions, units: i32) {
    match direction {
        Directions::North => coordinates.y += units,
        Directions::East => coordinates.x += units,
        Directions::West => coordinates.x += -units,
        Directions::South => coordinates.y += -units,
        _ => {}
    }
}

fn get_manhattan_distance(source: &Coordinates, destination: &Coordinates) -> i32 {
    let x1 = source.x;
    let x2 = destination.x;
    let y1 = source.y;
    let y2 = destination.y;

    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}

fn navigate(mut start: &mut Coordinates, instructions: &Vec<NavigationInstruction>) -> Coordinates {
    let mut direction = Directions::East;
    for instruction in instructions.iter() {
        let units = instruction.units;
        let current_direction = &instruction.direction;
        if *current_direction == Directions::Left || *current_direction == Directions::Right {
            direction = maybe_turn(&direction, current_direction, &units).unwrap();
            continue;
        } else if *current_direction == Directions::Forward {
            travel_along(&mut start, &direction, units as i32);
        } else {
            travel_along(&mut start, current_direction, units as i32);
        }
    }

    return start.clone();
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let mut start: Coordinates = Coordinates { x: 0, y: 0 };
    let instructions = parse(&input);
    let destination = navigate(&mut start, &instructions);

    println!("{:?}", destination);
    let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &destination);

    println!("Manhattan Distance: {}", manhattan_distance);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_navigate() {
        let input = r#"
        F10
        N3
        F7
        R90
        F11
        "#;

        let mut start: Coordinates = Coordinates { x: 0, y: 0 };
        let instructions = parse(&input);
        let destination = navigate(&mut start, &instructions);
        let manhattan_distance = get_manhattan_distance(&start, &destination);
        assert_eq!(manhattan_distance, 25);
    }
}
