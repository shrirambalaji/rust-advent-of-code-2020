use std::collections::HashMap;
use std::{env, fs};

// Problem 1: Fixing your expense report.
// https://adventofcode.com/2020/day/1
// Given a list of numbers, find the numbers that sum to 2020, and return what their product would be.

const TARGET_SUM: i32 = 2020;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = &args[1];

    println!("--- Part 1 ---");
    let mut entries: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, usize> = HashMap::new();
    let input = fs::read_to_string(filepath).unwrap();
    for line in input.lines() {
        let entry = line.parse::<i32>().unwrap();
        entries.push(entry);
    }

    // Hint: Two Sum
    // Given an array of numbers, find its pair that equals a target
    for (index, entry) in entries.iter().enumerate() {
        let complement = TARGET_SUM - entry;
        if map.contains_key(&complement) {
            let chosen_one_index = map.get(&complement).unwrap();
            let chosen_two_index = &index;

            let chosen_one = entries.get(*chosen_one_index as usize).unwrap();
            let chosen_two = entries.get(*chosen_two_index as usize).unwrap();

            println!("2 entries that sum to 2020: {}, {}", chosen_one, chosen_two);
            println!("Product Of two entries: {}", chosen_one * chosen_two);
        } else {
            map.insert(*entry, index);
        }
    }

    println!("");
    println!("-- Part 1 Extended --");
    // Hint: 3Sum
    // Find three numbers that sum to target
    entries.sort();

    for (i, entry) in entries.iter().enumerate() {
        let mut low = i + 1;
        let mut high = entries.len() - 1;

        while low < high {
            let current_sum = &entries[low] + &entries[high] + entry;
            // since there's only one such entry based on the question, we can break here.
            // otherwise we'd typically push these into a Vec<u8> | HashSet<u8> to deal with duplicates.
            if current_sum == TARGET_SUM {
                println!(
                    "3 Entries that sum to 2020: {}, {}, {}",
                    &entries[low], &entries[high], entry
                );

                println!(
                    "Product of 3 Entries: {}",
                    &entries[low] * &entries[high] * entry
                );

                break;
            } else if current_sum < TARGET_SUM {
                low += 1;
            } else {
                high -= 1;
            }
        }
    }
}
