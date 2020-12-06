#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{env, fs, process};

#[derive(Debug)]
struct Seat {
    row: i32,
    column: i32,
}

#[derive(Debug)]
struct RangeDelimiters {
    upper: char,
    lower: char,
}

struct SeatRange {
    start: i32,
    end: i32,
}

const TOTAL_ROWS: i32 = 128;
const TOTAL_COLUMNS: i32 = 7;

fn get_range_from_seat(seat: &str, delimeters: RangeDelimiters, max_count: i32) -> SeatRange {
    let mut range = SeatRange {
        start: 0,
        end: max_count,
    };

    let RangeDelimiters { lower, upper } = delimeters;

    for char in seat.chars() {
        if char == lower {
            range.end = (range.start + range.end) / 2;
        } else if char == upper {
            range.start = range.start + (range.end - range.start) / 2;
        }
    }

    range
}

fn parse_seat(seat: &str) -> Result<Seat, String> {
    lazy_static! {
        static ref SEAT_REGEX: Regex = Regex::new(r"(\w{7})(\w{3})").unwrap();
    }

    if seat.len() < 10 {
        return Err("Invalid seat".to_owned());
    }

    let captures = SEAT_REGEX.captures(seat).unwrap();
    let rows: &str = &captures[1];
    let columns: &str = &captures[2];

    if rows.len() < 7 || columns.len() < 3 {
        return Err("Invalid seat".to_owned());
    }

    let row_range_delimiters = RangeDelimiters {
        upper: 'B',
        lower: 'F',
    };

    let column_range_delimiters = RangeDelimiters {
        upper: 'R',
        lower: 'L',
    };

    let SeatRange {
        start: seat_row, ..
    } = get_range_from_seat(rows, row_range_delimiters, TOTAL_ROWS);

    let SeatRange {
        end: seat_column, ..
    } = get_range_from_seat(columns, column_range_delimiters, TOTAL_COLUMNS);

    let seat = Seat {
        row: seat_row,
        column: seat_column,
    };

    Ok(seat)
}

fn get_seat_id(seat: Seat) -> i32 {
    seat.row * 8 + seat.column
}

fn get_missing_id(mut ids: Vec<i32>) -> i32 {
    ids.sort();

    let mut prev = ids[0];
    let ids: Vec<i32> = ids[1..].to_vec();
    for curr in ids {
        if prev != curr - 1 {
            break;
        }
        prev += 1;
    }
    prev + 1
}

fn process(input: &str) -> (i32, i32) {
    let ids: Vec<i32> = input
        .lines()
        .map(|line| {
            let seat = match parse_seat(line) {
                Ok(seat) => seat,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            };

            get_seat_id(seat)
        })
        .collect::<Vec<i32>>();

    let max = *ids.iter().max().unwrap();
    return (max, get_missing_id(ids));
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input = fs::read_to_string(filepath).expect("Something went wrong while reading the file");

    let (max, seat_id) = process(&input);
    println!("{:?} {}", max, seat_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_seat() {
        let seat = parse_seat("FBFBBFFRLR").unwrap();
        assert_eq!(seat.row, 44);
        assert_eq!(seat.column, 5);
    }
}
