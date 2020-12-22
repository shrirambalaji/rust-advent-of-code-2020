use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    env, fs,
    hash::{Hash, Hasher},
};

type Deck = VecDeque<u32>;

#[derive(Debug, PartialEq)]
enum Winner {
    Player1,
    Player2,
}

fn get_score(deck: &VecDeque<u32>) -> u32 {
    let num_cards = deck.len() as u32;
    deck.iter()
        .enumerate()
        .fold(0, |acc, (idx, card)| acc + card * (num_cards - idx as u32))
}

// Returns the hash for any given collection T
fn get_hash<T: Hash>(deque: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    deque.hash(&mut hasher);
    hasher.finish()
}

// Part 01
fn combat(deck1: &mut Deck, deck2: &mut Deck) -> Winner {
    if deck1.len() != deck2.len() {
        panic!("Players don't have equal number of cards. Invalid Game!")
    }

    loop {
        if deck1.is_empty() {
            break Winner::Player2;
        } else if deck2.is_empty() {
            break Winner::Player1;
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
}

// Part 02
fn recursive_combat(deck1: &mut Deck, deck2: &mut Deck) -> Winner {
    // Store the hashes of decks from previous rounds, used for later comparison
    let mut previous_rounds = HashSet::new();

    // Start the Game Loop
    loop {
        let deck1_hash = get_hash(&deck1);
        let deck2_hash = get_hash(&deck2);

        // Check if the current decks have been seen in previous rounds
        if previous_rounds.contains(&deck1_hash) || previous_rounds.contains(&deck2_hash) {
            break Winner::Player1;
        } else {
            previous_rounds.insert(deck1_hash);
            previous_rounds.insert(deck2_hash);
        }

        // Get the cards from the top of the deck for comparison
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        // we need to run a subgame to find out the winner
        let winner = if card1 <= deck1.len() as u32 && card2 <= deck2.len() as u32 {
            let mut sub_deck1 = deck1.clone();
            sub_deck1.truncate(card1 as usize);

            let mut sub_deck2 = deck2.clone();
            sub_deck2.truncate(card2 as usize);

            // recursively run a subgame to find out the winner for the current round
            recursive_combat(&mut sub_deck1, &mut sub_deck2)
        } else if card1 > card2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        // based on who won the round, the cards are moved to round winner's deck
        match winner {
            Winner::Player1 => {
                deck1.push_back(card1);
                deck1.push_back(card2);
            }
            Winner::Player2 => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        }

        // if any one of the decks are empty, declare the winner
        if deck1.is_empty() {
            break Winner::Player2;
        }

        if deck2.is_empty() {
            break Winner::Player1;
        }
    }
}

fn get_decks(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let lines: Vec<&str> = input.lines().collect();
    let empty_index = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("Invalid Game Input! Expected Player decks to be separated by an empty line.");

    // 0th element is the Player 1: title
    let player1_deck = lines[1..empty_index]
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<VecDeque<u32>>();

    // empty_index + 1 has the Player 2: title
    let player2_deck = lines[empty_index + 2..]
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<VecDeque<u32>>();

    (player1_deck, player2_deck)
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    // -- Part 01: Combat ---
    let (mut player1_deck, mut player2_deck) = get_decks(&input);
    let winner = combat(&mut player1_deck, &mut player2_deck);
    let score = match winner {
        Winner::Player1 => get_score(&player1_deck),
        Winner::Player2 => get_score(&player2_deck),
    };

    println!("Combat Winner: {:?}, Score: {:?}", winner, score);

    // -- Part 02: Recursive Combat --
    let (mut player1_deck, mut player2_deck) = get_decks(&input);
    let winner = recursive_combat(&mut player1_deck, &mut player2_deck);
    let score = match winner {
        Winner::Player1 => get_score(&player1_deck),
        Winner::Player2 => get_score(&player2_deck),
    };
    println!("Recursive Combat Winner: {:?}, Score: {:?}", winner, score);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_combat() {
        let input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
        let (mut player1_deck, mut player2_deck) = get_decks(&input);
        let winner = combat(&mut player1_deck, &mut player2_deck);
        assert_eq!(winner, Winner::Player2);
        assert_eq!(get_score(&player2_deck), 306);
    }

    #[test]
    fn should_recursively_combat() {
        let input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#;
        let (mut player1_deck, mut player2_deck) = get_decks(&input);
        let winner = recursive_combat(&mut player1_deck, &mut player2_deck);
        assert_eq!(winner, Winner::Player2);
        assert_eq!(get_score(&player2_deck), 291);
    }
}
