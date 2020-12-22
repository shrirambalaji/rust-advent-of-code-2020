use std::{env, fs};

fn find_earliest(timestamp: u64, bus_ids: Vec<&str>) -> (u64, u64) {
    let (chosen_bus, wait) = bus_ids
        .iter()
        .filter(|id| **id != "\'x\'")
        .map(|id| {
            let bus_id = id.parse::<u64>().unwrap();
            (bus_id, bus_id - timestamp % bus_id)
        })
        .min_by_key(|(_id, offset)| *offset)
        .unwrap();

    (chosen_bus, wait)
}

fn process(input: &str) -> (u64, Vec<&str>) {
    let notes: Vec<&str> = input.split('\n').collect();
    let timestamp = notes[0].parse::<u64>().expect("Invalid timestamp");
    let bus_ids: Vec<&str> = notes[1].split(',').collect();
    (timestamp, bus_ids)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    let (timestamp, bus_ids) = process(&input);

    // -- Part 01 --
    let (chosen_bus, wait) = find_earliest(timestamp, bus_ids);
    println!("Chosen Bus ID: {} * Wait Time In Minutes: {} =  {}", chosen_bus, wait, chosen_bus * wait);
}

#[test]
fn should_process_and_find_earliest() {
    let input = r#"939
7,13,'x','x',59,'x',31,19"#;

    let (timestamp, bus_ids) = process(input);
    let (chosen_bus, wait) = find_earliest(timestamp, bus_ids);
    assert_eq!(chosen_bus, 59);
    assert_eq!(wait, 5);
}
