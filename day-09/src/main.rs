use std::{env, fs};

/// checks if a contigous sequence of numbers sum up to a target sum
/// essentially a two-pointer with start and stop pointers, incremented based on whether the sum is less than or greater than target.
fn find_contiguos_sum(target: u64, numbers: &Vec<u64>) -> Vec<u64> {
    let mut start = 0;
    let mut end = 1;
    loop {
        let sum: u64 = numbers[start..=end].iter().sum();
        if sum == target {
            // TODO: can we return a slice instead of a .to_vec?
            break numbers[start..=end].to_vec();
        } else if sum > target {
            start += 1;
        } else {
            end += 1;
        }

        continue;
    }
}

/// if n = contiguos numbers that add-up to invalid_number, weakness score -> smallest(n) + largest(n)
fn get_encryption_weakness_score(invalid_number: u64, numbers: &Vec<u64>) -> u64 {
    let mut numbers_with_contigous_sum = find_contiguos_sum(invalid_number, numbers);
    numbers_with_contigous_sum.sort();

    let len = numbers_with_contigous_sum.len();
    let smallest = numbers_with_contigous_sum[0];
    let largest = numbers_with_contigous_sum[len - 1];

    return smallest + largest;
}

/// checks if a target sum is present when adding any of the two numbers in a given list
fn has_target_sum(target: u64, numbers: &[u64]) -> bool {
    for num in numbers {
        let complement = (target as i64 - *num as i64) as u64;
        // TODO: `.contains` is likely a O(n) search, could this probably be optimized?
        if numbers.contains(&complement) && &complement != num {
            return true;
        } else {
            continue;
        }
    }
    false
}

/// a number that disobeys the preamble, is a number who's value doesnt equal to any of the
/// the preamble is an array of numbers from 0..=preamble_len
fn find_number_that_disobeys_preamble(numbers: &Vec<u64>, preamble_len: usize) -> u64 {
    // rest of all the numbers that follow the preamble.
    let numbers_after_preamble = &numbers[preamble_len..];
    for (index, num) in numbers_after_preamble.iter().enumerate() {
        let start = 0 + index;
        let end = index + preamble_len;

        let preamble = &numbers[start..end];
        if has_target_sum(*num as u64, preamble) {
            continue;
        } else {
            return *num;
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    let numbers: Vec<u64> = input
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let invalid_number = find_number_that_disobeys_preamble(&numbers, 25);
    println!("Invalid number: {}", invalid_number);

    let weakness_score = get_encryption_weakness_score(invalid_number, &numbers);
    println!("Encryption Weakness Score {}", weakness_score);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn vec_compare<T: std::cmp::PartialEq>(vec1: &[T], vec2: &[T]) -> bool {
        (vec1.len() == vec2.len()) && vec1.iter().zip(vec2).all(|(a, b)| *a == *b)
    }

    #[test]
    fn should_find_weaker_number() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let expected = find_number_that_disobeys_preamble(&numbers, 5);
        assert_eq!(expected, 127);
    }

    #[test]
    fn should_find_contigous_sum_and_weakness_score() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let expected = vec![15, 25, 47, 40];
        assert!(vec_compare(&expected, &find_contiguos_sum(127, &numbers)));
        assert_eq!(62, get_encryption_weakness_score(127, &numbers));
    }
}
