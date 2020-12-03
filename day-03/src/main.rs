use std::{env, fs};

// Day 03
// https://adventofcode.com/2020/day/3

// Given a grid of #'s and .'s  find the number of #'s while traversing through the grid in the diagonal direction.
// The `#` are referred to as trees, and `.` are referred to as squares.
// Input: Diagonal Along right 3, down 1.
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#

// Output: 7#'s

// Algorithm
// * Parse the input and convert it into a Vec<Vec<char>> ie. Grid
// * Traverse from start of the array along the given direction.
// * If the element at the specified position is a #, increment `tree_counter`.

const TREE: char = '#';
const SQUARE: char = '.';

#[derive(Debug, PartialEq)]
struct Traverse {
    column: i32,
    row: i32,
}

fn create_grid<'a>(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let elements_in_row: Vec<char> = line.chars().collect();
        grid.push(elements_in_row);
    }

    grid
}

fn parse_direction(direction: &str) -> Traverse {
    let mut Traverse = Traverse { column: 0, row: 0 };

    let instructions: Vec<&str> = direction.split(",").collect();
    for instruction in instructions.iter() {
        let direction_and_steps: Vec<&str> = instruction.split_whitespace().collect();
        let direction = direction_and_steps
            .get(0)
            .expect(&format!("Invalid direction {}", direction));

        let step = direction_and_steps
            .get(1)
            .expect(&format!("Invalid step {}", direction));

        let step = step.parse::<i32>().unwrap();

        match *direction {
            "up" => Traverse.row = step,
            "right" => Traverse.column = step,
            "down" => Traverse.row = -step,
            "left" => Traverse.column = -step,
            _ => {}
        }
    }

    Traverse
}

fn find_number_of_trees(mut grid: Vec<Vec<char>>, direction: &str) -> i32 {
    let count = 0;
    let direction = parse_direction(direction);

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            let traversed_col_index = if direction.column > 0 {
                col_index + direction.column as usize
            } else {
                col_index - direction.column as usize
            };

            let traversed_row_index = if direction.row > 0 {
                row_index + direction.row as usize
            } else {
                row_index - direction.row as usize
            };

            let grid_row = grid.get(traversed_row_index).unwrap();
            let element = grid_row.get(traversed_col_index).unwrap();
            println!("{}", element);
        }
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("Input file cannot be empty!");
    let input = fs::read_to_string(filepath).expect("Something went wrong while reading input");
    let grid = create_grid(&input);

    let tree_count = find_number_of_trees(grid, "right 1, down 3");
    println!("Number of trees {}", tree_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn vec_compare<T: std::cmp::PartialEq>(vec1: &[T], vec2: &[T]) -> bool {
        (vec1.len() == vec2.len()) && vec1.iter().zip(vec2).all(|(a, b)| *a == *b)
    }

    #[test]
    fn should_create_grid() {
        let input = r###"..##
#...
.#.."###;

        let actual = create_grid(&input);
        let expected = vec![
            vec!['.', '.', '#', '#'],
            vec!['#', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
        ];
        assert!(vec_compare(&expected, &actual));
    }

    // vec!['.', '.', '#', '#',   '#', '.', '.', '.',    '.', '#', '.', '.'],

    #[test]
    fn should_parse_direction() {
        let direction = "right 3, down 1";
        let actual = parse_direction(direction);
        let expected = Traverse { column: 3, row: 1 };

        assert_eq!(actual, expected);
    }
}
