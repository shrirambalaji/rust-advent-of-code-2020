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

fn turn(direction_before_turn: &Directions, turn: &Directions, angle: &u32) -> Option<Directions> {
    // turn by 90deg by a specified number of times along the given direction
    let turn_90 = |times: i32| {
        let mut direction_after_turn = direction_before_turn.clone();

        for _ in 0..times {
            direction_after_turn = match turn {
                Directions::Left => {
                    if direction_after_turn == Directions::North {
                        Directions::West
                    } else if direction_after_turn == Directions::East {
                        Directions::North
                    } else if direction_after_turn == Directions::West {
                        Directions::South
                    } else if direction_after_turn == Directions::South {
                        Directions::East
                    } else {
                        panic!("Unknown Turn Direction")
                    }
                }
                Directions::Right => {
                    if direction_after_turn == Directions::North {
                        Directions::East
                    } else if direction_after_turn == Directions::East {
                        Directions::South
                    } else if direction_after_turn == Directions::West {
                        Directions::North
                    } else if direction_after_turn == Directions::South {
                        Directions::West
                    } else {
                        panic!("Unknown Turn Direction")
                    }
                }
                _ => panic!("Unknown Turn Direction"),
            };
        }

        return direction_after_turn;
    };

    match *angle {
        90 => Some(turn_90(1)),
        180 => Some(turn_90(2)),
        270 => Some(turn_90(3)),
        _ => Some(direction_before_turn.clone()),
    }
}

fn move_ship(direction: &Directions, units: i32, coordinates: &mut Coordinates) {
    match direction {
        Directions::North => coordinates.y += units,
        Directions::East => coordinates.x += units,
        Directions::West => coordinates.x += -units,
        Directions::South => coordinates.y += -units,
        _ => {}
    }
}

fn get_manhattan_distance(source: &Coordinates, destination: &Coordinates) -> i32 {
    return (source.x - destination.x).abs() + (source.y - destination.y).abs();
}

fn navigate_ship(mut coordinates: &mut Coordinates, instructions: &Vec<NavigationInstruction>) {
    // Initial direction is East
    let mut direction = Directions::East;
    for instruction in instructions.iter() {
        let units = instruction.units;
        let current_direction = &instruction.direction;
        // Turn if the current direction mentioned is either left or right, skip to the next instruction.
        if *current_direction == Directions::Left || *current_direction == Directions::Right {
            direction = turn(&direction, current_direction, &units).unwrap();
            continue;
        }

        if *current_direction == Directions::Forward {
            move_ship(&direction, units as i32, &mut coordinates);
        } else {
            move_ship(current_direction, units as i32, &mut coordinates);
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let mut coordinates: Coordinates = Coordinates { x: 0, y: 0 };
    let instructions = parse(&input);
    navigate_ship(&mut coordinates, &instructions);
    let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &coordinates);
    println!("Manhattan Distance: {}", manhattan_distance);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn should_navigate_ship() {
        let input = r#"
        F10
        N3
        F7
        R90
        F11
        "#;

        let mut coordinates: Coordinates = Coordinates { x: 0, y: 0 };
        let instructions = parse(&input);
        navigate_ship(&mut coordinates, &instructions);
        let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &coordinates);
        assert_eq!(manhattan_distance, 25);
    }
}
