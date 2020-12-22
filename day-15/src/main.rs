use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

fn find_nth_number_in_game(numbers: &[u64], n: u64) -> u64 {
    let mut memory: HashMap<u64, VecDeque<u64>> = HashMap::new();
    let mut last_number: u64 = 0;
    let mut last_turn: u64 = 0;

    for (index, starting_number) in numbers.iter().enumerate() {
        last_number = *starting_number;
        last_turn = (index + 1) as u64;
        memory.insert(*starting_number, vec![last_turn].into_iter().collect());
    }

    // TODO: Maybe parallelize for larger values of n?
    let turns: Vec<u64> = (last_turn + 1..=n).collect();
    turns.iter().for_each(|turn| {
        if memory.contains_key(&last_number) {
            let spoken_before: &mut VecDeque<u64> = memory.get_mut(&last_number).unwrap();
            if spoken_before.len() <= 1 {
                last_number = 0;
            } else {
                let last_turn = spoken_before.pop_back().unwrap();
                if let Some(second_last_turn) = spoken_before.pop_back() {
                    let diff: i32 = (last_turn - second_last_turn) as i32;
                    last_number = diff.abs() as u64;
                }
                spoken_before.push_back(last_turn);
            }
        }

        memory
            .entry(last_number)
            .and_modify(|e| {
                e.push_back(*turn);
            })
            .or_insert_with(|| {
                let mut new_turn: VecDeque<u64> = VecDeque::new();
                new_turn.push_back(*turn);
                new_turn
            });
    });

    last_number
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let starting_numbers: Vec<u64> = input
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // -- Part 01 --
    let n = 2020;
    let nth_number = find_nth_number_in_game(&starting_numbers, n);
    println!("{}th number: {}", n, nth_number);

    // -- Part 02 --
    let n = 30_000_000;
    let nth_number = find_nth_number_in_game(&starting_numbers, n);
    println!("{}th number: {}", n, nth_number);
}

#[test]
fn should_find_nth_number() {
    assert_eq!(find_nth_number_in_game(&[0, 3, 6], 2020), 436);
    assert_eq!(find_nth_number_in_game(&[1, 3, 2], 2020), 1);
    assert_eq!(find_nth_number_in_game(&[2, 1, 3], 2020), 10);
    assert_eq!(find_nth_number_in_game(&[3, 1, 2], 2020), 1836);
    assert_eq!(find_nth_number_in_game(&[0, 3, 6], 30_000_000), 175594);
}
