use std::{env, fs};

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';

fn create_seat_layout(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let chars: Vec<char> = line.trim().chars().collect();
        grid.push(chars);
    }

    grid
}

fn apply_seating_rules(seats: &mut Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let get_seat = |row_idx: usize, col_idx: usize| -> Option<&char> {
        match seats.get(row_idx) {
            Some(row) => return row.get(col_idx),
            _ => {}
        }

        None
    };

    let get_neighbor_positions = |row_idx: i32, col_idx: i32| -> Vec<(i32, i32)> {
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

        positions.to_vec()
    };

    let has_empty_neighbors = |row_idx: usize, col_idx: usize| -> bool {
        let row_idx = row_idx as i32;
        let col_idx = col_idx as i32;
        let positions = get_neighbor_positions(row_idx, col_idx);
        positions
            .iter()
            .all(|(r, c)| match get_seat(*r as usize, *c as usize) {
                Some(value) => *value != OCCUPIED_SEAT,
                _ => true,
            })
    };

    let count_occupied_neighbors = |row_idx: usize, col_idx: usize| -> usize {
        let row_idx = row_idx as i32;
        let col_idx = col_idx as i32;

        let positions = get_neighbor_positions(row_idx, col_idx);
        positions
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
            match *element {
                EMPTY_SEAT => {
                    if has_empty_neighbors(row_idx, col_idx) {
                        modified = true;
                        new_arrangement[row_idx][col_idx] = OCCUPIED_SEAT;
                    }
                }
                OCCUPIED_SEAT => {
                    if count_occupied_neighbors(row_idx, col_idx) >= 4 {
                        modified = true;
                        new_arrangement[row_idx][col_idx] = EMPTY_SEAT;
                    }
                }
                // FLOOR => {}
                _ => {}
            }
        }
    }

    return (new_arrangement, modified);
}

fn count_occupied_seats(seats: &mut Vec<Vec<char>>) -> usize {
    seats
        .iter()
        .flatten()
        .filter(|seat| **seat == OCCUPIED_SEAT)
        .count()
}

fn count_occupied_seats_after_chaos(seats: &mut Vec<Vec<char>>) -> usize {
    let (mut new_arrangement, is_modified) = apply_seating_rules(seats);
    if !is_modified {
        return count_occupied_seats(&mut new_arrangement);
    } else {
        return count_occupied_seats_after_chaos(&mut new_arrangement);
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let mut seats = create_seat_layout(&input);
    // -- Part 01 --

    let count = count_occupied_seats_after_chaos(&mut seats);
    println!("Number of occupied seats after chaos stabilises {}", count);
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
    fn should_apply_seating_rules() {
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

        let (new_arrangement, _) = apply_seating_rules(&mut seat_layout);
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
    fn should_count_occupied_seats_after_chaos() {
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

        let count = count_occupied_seats_after_chaos(&mut seat_layout);
        assert_eq!(count, 37);
    }
}
