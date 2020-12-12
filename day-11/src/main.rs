use std::{env, fs};

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
const FLOOR: char = '.';

fn create_seat_layout(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    grid
}

fn apply_seating_rules(seats: &mut Vec<Vec<char>>) {
    let get_seat = |row_idx: usize, col_idx: usize| -> Option<&char> {
        match seats.get(row_idx) {
            Some(row) => return row.get(col_idx),
            _ => {}
        }

        None
    };

    let is_seat_available = |row_idx: usize, col_idx: usize| -> bool {
        let positions = [
            (row_idx, col_idx + 1),
            (row_idx, col_idx - 1),
            (row_idx - 1, col_idx),
            (row_idx + 1, col_idx),
            (row_idx - 1, col_idx - 1),
            (row_idx - 1, col_idx + 1),
            (row_idx + 1, col_idx - 1),
            (row_idx + 1, col_idx + 1),
        ];

        positions.iter().all(|(r, c)| match get_seat(*r, *c) {
            Some(value) => *value != OCCUPIED_SEAT || *value != FLOOR,
            None => true,
        })
    };

    let mut changed = false;
    let mut new_arrangement: Vec<Vec<char>> = seats.clone();

    for (row_idx, row) in seats.iter().enumerate() {
        for (col_idx, mut col) in row.iter().enumerate() {
            let element = seats.get(row_idx).unwrap().get(col_idx).unwrap();
            match *element {
                EMPTY_SEAT => {
                    if is_seat_available(row_idx, col_idx) {
                        changed = true;
                        new_arrangement[row_idx][col_idx] = OCCUPIED_SEAT;
                    }
                }
                OCCUPIED_SEAT => {}
                FLOOR => {}
                _ => {}
            }
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let seats = create_seat_layout(&input);
}
