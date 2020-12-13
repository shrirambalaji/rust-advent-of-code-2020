use std::{env, fs};

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
const FLOOR: char = '.';

fn create_seat_layout(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let chars: Vec<char> = line.trim().chars().collect();
        grid.push(chars);
    }

    grid
}

#[derive(PartialEq)]
enum SeatingRuleTypes {
    One,
    Two,
}
struct SeatingArrangement {
    rule_type: SeatingRuleTypes,
}

pub trait SeatingRules {
    fn apply_seating_rules(&self, seats: &mut Vec<Vec<char>>) -> (Vec<Vec<char>>, bool);
}

impl SeatingRules for SeatingArrangement {
    fn apply_seating_rules(&self, seats: &mut Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
        let get_seat = |row_idx: usize, col_idx: usize| -> Option<&char> {
            match seats.get(row_idx) {
                Some(row) => return row.get(col_idx),
                _ => {}
            }

            None
        };

        let directions = [
            (0, 1),
            (1, 0),
            (1, 1),
            (-1, 0),
            (0, -1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];

        let get_neighbor_positions =
            |row_idx: i32, col_idx: i32, handle_floor: bool| -> Vec<(i32, i32)> {
                let mut positions = Vec::new();

                if handle_floor {
                    for direction in directions.iter() {
                        let mut position = (row_idx + direction.0, col_idx + direction.1);
                        loop {
                            // Bounds Check
                            if row_idx < 0
                                || col_idx < 0
                                || row_idx as usize > seats.len() - 1
                                || col_idx as usize > seats[0].len() - 1
                            {
                                break;
                            }

                            // When the current position is a floor, we move forward in the same direction, skipping the current position.
                            match get_seat(position.0 as usize, position.1 as usize) {
                                Some(seat) => {
                                    if *seat == FLOOR {
                                        position =
                                            (position.0 + direction.0, position.1 + direction.1);
                                        continue;
                                    } else if *seat == OCCUPIED_SEAT || *seat == EMPTY_SEAT {
                                        positions.push(position);
                                        break;
                                    }
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    for direction in directions.iter() {
                        positions.push((row_idx + direction.0, col_idx + direction.1))
                    }
                }

                positions.to_vec()
            };

        let has_empty_neighbors = |neighbors: Vec<(i32, i32)>| -> bool {
            neighbors
                .iter()
                .all(|(r, c)| match get_seat(*r as usize, *c as usize) {
                    Some(value) => *value != OCCUPIED_SEAT,
                    _ => true,
                })
        };

        let count_occupied_neighbors = |neighbors: Vec<(i32, i32)>| -> usize {
            neighbors
                .iter()
                .filter(|(r, c)| match get_seat(*r as usize, *c as usize) {
                    Some(value) => return *value == OCCUPIED_SEAT,
                    None => false,
                })
                .count()
        };

        let mut modified = false;
        let mut new_arrangement: Vec<Vec<char>> = seats.clone();

        for (row_idx, row) in seats.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {
                let element = seats.get(row_idx).unwrap().get(col_idx).unwrap();
                let neighbors = get_neighbor_positions(
                    row_idx as i32,
                    col_idx as i32,
                    self.rule_type == SeatingRuleTypes::Two,
                );

                let occupied_neighbor_limit = if self.rule_type == SeatingRuleTypes::Two {
                    5
                } else {
                    4
                };

                match *element {
                    EMPTY_SEAT => {
                        if has_empty_neighbors(neighbors) {
                            modified = true;
                            new_arrangement[row_idx][col_idx] = OCCUPIED_SEAT;
                        }
                    }
                    OCCUPIED_SEAT => {
                        if count_occupied_neighbors(neighbors) >= occupied_neighbor_limit {
                            modified = true;
                            new_arrangement[row_idx][col_idx] = EMPTY_SEAT;
                        }
                    }
                    _ => {}
                }
            }
        }

        return (new_arrangement, modified);
    }
}

fn count_occupied_seats(seats: &mut Vec<Vec<char>>) -> usize {
    seats
        .iter()
        .flatten()
        .filter(|seat| **seat == OCCUPIED_SEAT)
        .count()
}

fn count_occupied_seats_after_chaos(
    mut seats: &mut Vec<Vec<char>>,
    rules: &SeatingArrangement,
) -> usize {
    let (mut new_arrangement, is_modified) = rules.apply_seating_rules(&mut seats);
    if !is_modified {
        return count_occupied_seats(&mut new_arrangement);
    } else {
        return count_occupied_seats_after_chaos(&mut new_arrangement, &rules);
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let mut seats = create_seat_layout(&input);

    // -- Part 01 --
    let part_01_arrangement = SeatingArrangement {
        rule_type: SeatingRuleTypes::One,
    };

    let count = count_occupied_seats_after_chaos(&mut seats, &part_01_arrangement);
    println!(
        "Number of occupied seats after chaos stabilises by Seating Rule 01: {}",
        count
    );

    // -- Part 02 --
    let part_02_arrangement = SeatingArrangement {
        rule_type: SeatingRuleTypes::Two,
    };

    let count = count_occupied_seats_after_chaos(&mut seats, &part_02_arrangement);
    println!(
        "Number of occupied seats after chaos stabilises by Seating Rule 02: {}",
        count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// used to compare vectors of the same type
    fn vec_compare<T: std::cmp::PartialEq>(vec1: &[T], vec2: &[T]) -> bool {
        (vec1.len() == vec2.len()) && vec1.iter().zip(vec2).all(|(a, b)| *a == *b)
    }

    #[test]
    fn should_create_seating_layout_grid() {
        let input = r###"L.LL.LL
LLLLLLL
L.L.L.."###;

        let expected: Vec<Vec<char>> = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.'],
        ];

        let layout = create_seat_layout(input);
        assert!(vec_compare(&expected, &layout));
    }

    #[test]
    fn should_apply_seating_rules_for_rule_one() {
        let mut seat_layout: Vec<Vec<char>> = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        ];

        let rules = SeatingArrangement {
            rule_type: SeatingRuleTypes::One,
        };
        let (new_arrangement, _) = rules.apply_seating_rules(&mut seat_layout);
        let expected = vec![
            vec!['#', '.', '#', '#', '.', '#', '#', '.', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '.', '#', '#'],
            vec!['#', '.', '#', '.', '#', '.', '.', '#', '.', '.'],
            vec!['#', '#', '#', '#', '.', '#', '#', '.', '#', '#'],
            vec!['#', '.', '#', '#', '.', '#', '#', '.', '#', '#'],
            vec!['#', '.', '#', '#', '#', '#', '#', '.', '#', '#'],
            vec!['.', '.', '#', '.', '#', '.', '.', '.', '.', '.'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '#', '#', '#', '#', '#', '#', '.', '#'],
            vec!['#', '.', '#', '#', '#', '#', '#', '.', '#', '#'],
        ];

        assert!(vec_compare(&expected, &new_arrangement));
    }

    #[test]
    fn should_count_occupied_seats_after_chaos_with_rule_one() {
        let mut seat_layout: Vec<Vec<char>> = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        ];

        let rules = SeatingArrangement {
            rule_type: SeatingRuleTypes::One,
        };

        let count = count_occupied_seats_after_chaos(&mut seat_layout, &rules);
        assert_eq!(count, 37);
    }

    #[test]
    fn should_count_occupied_seats_after_chaos_with_rule_two() {
        let mut seat_layout: Vec<Vec<char>> = vec![
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', '.', 'L', '.', '.', 'L', '.', '.'],
            vec!['L', 'L', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', '.', 'L', 'L', '.', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
            vec!['.', '.', 'L', '.', 'L', '.', '.', '.', '.', '.'],
            vec!['L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', 'L', '.', 'L'],
            vec!['L', '.', 'L', 'L', 'L', 'L', 'L', '.', 'L', 'L'],
        ];

        let rules = SeatingArrangement {
            rule_type: SeatingRuleTypes::Two,
        };

        let count = count_occupied_seats_after_chaos(&mut seat_layout, &rules);
        assert_eq!(count, 26);
    }
}
