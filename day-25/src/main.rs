use std::{env, fs};

// Steps:
// TRANSFORMATION_CONSTANT = 20201227
// x = 1
// Transform a Subject Number (7) => loop x times => x*=subject. x = x % TRANFORMATOIN_CONSTANT
// find loop size

const TRANSFORMATION_CONSTANT: usize = 20201227;
type LoopSize = usize;
type EncryptionKey = usize;

fn transform_subject_number(subject_number: usize, loop_size: usize) -> usize {
    let mut value: usize = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= TRANSFORMATION_CONSTANT;
    }

    value
}

fn get_loop_size(subject: usize, candidate_public_key: usize) -> LoopSize {
    let mut value = 1;
    let mut loop_size: LoopSize = 1;

    loop {
        value *= subject;
        value %= TRANSFORMATION_CONSTANT;
        if value == candidate_public_key {
            break loop_size;
        }
        loop_size += 1;
    }
}

fn find_encryption_key(card_public_key: usize, door_public_key: usize) -> EncryptionKey {
    let card_loop_size = get_loop_size(7, card_public_key);
    let door_loop_size = get_loop_size(7, door_public_key);

    let key1 = transform_subject_number(door_public_key, card_loop_size);
    let key2 = transform_subject_number(card_public_key, door_loop_size);

    if key1 == key2 {
        return key1;
    }

    panic!("Encryption key's dont match. Failed to unlock the door");
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    let public_keys: Vec<usize> = input
        .split('\n')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let encryption_key = find_encryption_key(public_keys[0], public_keys[1]);
    println!("Encryption Key: {:?}", encryption_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_loop_size() {
        let loop_size = get_loop_size(7, 5764801);
        assert_eq!(loop_size, 8);
        let loop_size = get_loop_size(7, 17807724);
        assert_eq!(loop_size, 11);
    }

    #[test]
    fn should_find_encryption_key() {
        assert_eq!(find_encryption_key(5764801, 17807724), 14897079);
    }
}
