use std::{env, fs};

/// A Singly LinkedList of Cups, where each index holds the value to the next element, represented as a Vec.
/// Every index points to the next cup, i.e. cups[5] returns the neighbor of cup 5.alloc
///
/// For eg. The sequence `[3, 8, 9, 1, 2, 5, 4, 6, 7]` represented as a CupplyLinkedList becomes `[0, 2, 5, 8, 6, 4, 7, 3, 9, 1]`

type CupplyLinkedList = Vec<usize>;

fn play_game(cup_labels: &[u8], moves: u32, total_cups: usize) -> Vec<usize> {
    // The number of values needs to be +1 of total_cups because the head of the LinkedList is 0 and acts as a dummy node.
    let mut cups: CupplyLinkedList = vec![0usize; total_cups + 1];
    let mut prev = cup_labels[0] as usize;

    // Populate the Cupply LinkedList By setting the value at index n to point to its neighbour
    for &label in cup_labels.iter().skip(1) {
        cups[prev] = label as usize;
        prev = label as usize;
    }

    // If there are more total cups than cup labels, populate the rest of the values in the CLL
    // This is to handle the specific usecase in Part 02
    for value in cup_labels.len() + 1..total_cups + 1 {
        cups[prev] = value;
        prev = value;
    }

    // Reset cups[prev] to point to first value once CLL is populated
    cups[prev] = cup_labels[0] as usize;

    // the first label is automatically designated as the current candidate
    let mut candidate = cup_labels[0] as usize;
    let mut picked = [0; 3];

    for _ in 0..moves {
        let mut next_cup = cups[candidate];
        for p in picked.iter_mut() {
            *p = next_cup;
            next_cup = cups[next_cup];
        }

        // candidate becomes the cup right next to picked.
        cups[candidate] = next_cup;

        let mut destination = candidate - 1;
        while destination < 1 || picked.contains(&destination) {
            if destination == 0 {
                destination = total_cups;
            } else {
                destination -= 1;
            }
        }

        // point end of picked to the cup after destination
        cups[picked[2]] = cups[destination];

        // point destination to the start of picked
        cups[destination] = picked[0];

        // move to the next cup
        candidate = cups[candidate];
    }

    cups
}

fn get_cups(input: &str) -> Vec<u8> {
    let labels: Vec<u8> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    labels
}

// Part 01
fn get_labels_after(cup_label: usize, cups: CupplyLinkedList) -> String {
    let mut labels = String::new();
    let mut candidate = cups[cup_label];
    for _ in 0..8 {
        labels.push((b'0' + candidate as u8) as char);
        let next = cups[candidate];
        candidate = next;
    }

    labels
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    // Part 01
    let sequence = get_cups(&input);
    let result = play_game(&sequence, 100, sequence.len());
    let labels = get_labels_after(1, result);

    println!("Labels of Cups after 1: {:?}", labels);

    // Part 02
    let cups = play_game(&sequence, 10_000_000, 1_000_000);
    let cup1 = cups[1];
    let cup2 = cups[cup1];
    println!("Product of Cup1 and Cup2 {:?}", cup1 * cup2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_play_game() {
        let sequence = get_cups("389125467");
        let result = play_game(&sequence, 100, sequence.len());
        let labels = get_labels_after(1, result);
        assert_eq!(labels, "92658374");
    }
}
