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

#[derive(Debug, PartialEq)]
struct Jump {
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

fn parse_direction(direction: &str) -> Jump {
    let mut jump = Jump { column: 0, row: 0 };

    let navigation_instructions: Vec<&str> = direction.split(",").collect();
    for instruction in navigation_instructions.iter() {
        let direction_and_steps: Vec<&str> = instruction.split_whitespace().collect();
        let direction = direction_and_steps
            .get(0)
            .expect(&format!("Invalid direction {}", direction));

        let step = direction_and_steps
            .get(1)
            .expect(&format!("Invalid step {}", direction));

        let step = step.parse::<i32>().unwrap();

        match *direction {
            "up" => jump.row = step,
            "right" => jump.column = step,
            "down" => jump.row = step,
            "left" => jump.column = -step,
            _ => {}
        }
    }

    jump
}

fn find_number_of_trees(grid: &mut Vec<Vec<char>>, direction: &str) -> i32 {
    let mut count = 0;
    let jump = parse_direction(direction);

    let col_len = grid[0].len();

    let mut col_index = 0;
    let mut row_index = 0;

    for row in grid.iter() {
        col_index = col_index + jump.column as usize;

        if row_index >= row.len() {
            row_index = row_index + jump.row as usize % row.len();
        } else {
            row_index = row_index + jump.row as usize;
        }

        if let Some(new_row) = grid.get(row_index) {
            if let Some(value) = new_row.get(col_index % col_len) {
                if *value == TREE {
                    count += 1
                }
            }
        }
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = args.get(1).expect("Input file cannot be empty!");
    let input = fs::read_to_string(filepath).expect("Something went wrong while reading input");
    let mut grid = create_grid(&input);

    // -- Part one --
    println!("-- Part one --");
    let tree_count_r3_d1 = find_number_of_trees(&mut grid, "right 3, down 1");
    println!(
        "Number of trees for Slope - right 3; down 1: {}",
        tree_count_r3_d1
    );

    // -- Part Two --
    let slopes = vec![
        "right 1, down 1",
        "right 3, down 1",
        "right 5, down 1",
        "right 7, down 1",
        "right 1, down 2",
    ];

    println!("");
    println!("-- Part two --");
    // Product of trees with slopes
    let product: i64 = slopes.iter().fold(1, |acc: i64, direction| {
        let count = find_number_of_trees(&mut grid, direction);
        println!("{}: {}", direction, count);
        acc * count as i64
    });

    println!("Product of all slopes: {:?}", product);
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

    #[test]
    fn should_parse_direction() {
        let direction = "right 3, down 1";
        let actual = parse_direction(direction);
        let expected = Jump { column: 3, row: 1 };

        assert_eq!(actual, expected);
    }
}
