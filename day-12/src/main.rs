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

enum NavigationTarget {
    Ship,
    ShipAndWaypoint,
}
struct Navigator<'a> {
    target: NavigationTarget,
    instructions: &'a Vec<NavigationInstruction>,
    coordinates: Coordinates,
}

fn get_manhattan_distance(source: &Coordinates, destination: &Coordinates) -> i32 {
    return (source.x - destination.x).abs() + (source.y - destination.y).abs();
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

fn turn(direction_before_turn: &Directions, turn: &Directions, angle: i32) -> Option<Directions> {
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

    match angle {
        90 => Some(turn_90(1)),
        180 => Some(turn_90(2)),
        270 => Some(turn_90(3)),
        _ => Some(direction_before_turn.clone()),
    }
}

fn move_along(direction: &Directions, units: i32, coordinates: &mut Coordinates) {
    match direction {
        Directions::North => coordinates.y += units,
        Directions::East => coordinates.x += units,
        Directions::West => coordinates.x += -units,
        Directions::South => coordinates.y += -units,
        _ => {}
    }
}

fn move_and_turn_waypoint(
    waypoint_coordinates: &mut Coordinates,
    turn_direction: &Directions,
    angle: i32,
) {
    let waypoint_x = waypoint_coordinates.x;
    let waypoint_y = waypoint_coordinates.y;

    let x_direction = if waypoint_x >= 0 {
        Directions::East
    } else {
        Directions::West
    };

    let y_direction = if waypoint_y >= 0 {
        Directions::North
    } else {
        Directions::South
    };

    let x_after_turn = turn(&x_direction, turn_direction, angle).unwrap();
    println!("{:?}", x_after_turn);
    let y_after_turn = turn(&y_direction, turn_direction, angle).unwrap();
    println!("{:?}", y_after_turn);

    let mut move_waypoint = |direction| {
        match direction {
            Directions::North => waypoint_coordinates.y = waypoint_y,
            Directions::East => waypoint_coordinates.x = waypoint_x,
            Directions::West => waypoint_coordinates.x = -waypoint_x,
            Directions::South => waypoint_coordinates.y = -waypoint_y,
            _ => {}
        };
    };

    move_waypoint(x_after_turn);
    move_waypoint(y_after_turn);
}

fn navigate(navigator: &mut Navigator) -> Coordinates {
    let mut direction = Directions::East;
    match navigator.target {
        NavigationTarget::Ship => {
            let mut ship_coordinates = navigator.coordinates.clone();
            for instruction in navigator.instructions.iter() {
                let units = instruction.units as i32;
                let current_direction = &instruction.direction;

                // Turn if the current direction mentioned is either left or right, skip to the next instruction.
                if *current_direction == Directions::Left || *current_direction == Directions::Right
                {
                    direction = turn(&direction, current_direction, units).unwrap();
                    continue;
                }

                if *current_direction == Directions::Forward {
                    move_along(&direction, units, &mut ship_coordinates);
                } else {
                    move_along(current_direction, units, &mut ship_coordinates);
                }
            }
            ship_coordinates
        }
        NavigationTarget::ShipAndWaypoint => {
            let mut ship_coordinates = Coordinates { x: 0, y: 0 };
            let mut waypoint_coordinates = navigator.coordinates.clone();
            for instruction in navigator.instructions.iter() {
                let units = instruction.units as i32;
                let current_direction = &instruction.direction;

                // Turn if the current direction mentioned is either left or right, skip to the next instruction.
                if *current_direction == Directions::Left || *current_direction == Directions::Right
                {
                    move_and_turn_waypoint(&mut waypoint_coordinates, current_direction, units);
                    continue;
                }

                if *current_direction == Directions::Forward {
                    ship_coordinates.x += waypoint_coordinates.x * units;
                    ship_coordinates.y += waypoint_coordinates.y * units;
                } else {
                    move_along(current_direction, units, &mut waypoint_coordinates);
                }
            }
            ship_coordinates
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let instructions = parse(&input);

    // -- Part 01 --
    let mut ship_navigator = Navigator {
        target: NavigationTarget::Ship,
        instructions: &instructions,
        coordinates: Coordinates { x: 0, y: 0 },
    };

    let destination = navigate(&mut ship_navigator);
    let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &destination);
    println!("Manhattan Distance: {}", manhattan_distance);

    // -- Part 02 --
    let mut waypoint_navigator = Navigator {
        target: NavigationTarget::ShipAndWaypoint,
        instructions: &instructions,
        coordinates: Coordinates { x: 10, y: 1 },
    };

    let destination = navigate(&mut waypoint_navigator);
    let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &destination);
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

        let instructions = parse(&input);
        let mut ship_navigator = Navigator {
            target: NavigationTarget::Ship,
            instructions: &instructions,
            coordinates: Coordinates { x: 0, y: 0 },
        };

        let destination = navigate(&mut ship_navigator);
        let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &destination);
        assert_eq!(manhattan_distance, 25);
    }

    #[test]
    fn should_navigate_ship_and_waypoint() {
        let input = r#"
        F10
        N3
        F7
        R90
        F11
        "#;

        let instructions = parse(&input);
        let mut waypoint_navigator = Navigator {
            target: NavigationTarget::ShipAndWaypoint,
            instructions: &instructions,
            coordinates: Coordinates { x: 10, y: 1 },
        };

        let destination = navigate(&mut waypoint_navigator);
        let manhattan_distance = get_manhattan_distance(&Coordinates { x: 0, y: 0 }, &destination);
        assert_eq!(manhattan_distance, 286);
    }
}
