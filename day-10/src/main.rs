use std::iter::FromIterator;
use std::{collections::HashSet, env, fs};

fn get_input_device_joltage(joltages: &Vec<u32>) -> u32 {
    let max_joltage = joltages.iter().max().unwrap();
    return max_joltage + 3;
}

struct JoltageDifference {
    one: u32,
    two: u32,
    three: u32,
}

fn get_joltage_differences(device_voltage: u32, adapters: &Vec<u32>) -> JoltageDifference {
    let mut difference = JoltageDifference {
        one: 1,
        two: 1,
        three: 1,
    };

    let joltages: HashSet<u32> = HashSet::from_iter(adapters.clone());
    let mut used_adapters: HashSet<u32> = HashSet::new();

    let mut use_joltage_adapter = |joltage: u32, increment: u32| {
        let target = &(joltage + increment);
        let is_compatible = joltages.contains(target)
            && !used_adapters.contains(&joltage)
            && *target <= device_voltage;
        if is_compatible {
            used_adapters.insert(joltage);
            match increment {
                1 => difference.one += 1,
                2 => difference.two += 1,
                3 => difference.three += 1,
                _ => {}
            }
        }
    };

    for joltage in &joltages {
        use_joltage_adapter(*joltage, 1);
        use_joltage_adapter(*joltage, 2);
        use_joltage_adapter(*joltage, 3);
    }

    return difference;
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    let adapters: Vec<u32> = input
        .lines()
        .map(|x| {
            x.parse::<u32>()
                .expect("invalid adapter joltage - expected to be a number")
        })
        .collect();

    let device_joltage = get_input_device_joltage(&adapters);
    let differences = get_joltage_differences(device_joltage, &adapters);

    println!(
        "There are {} differences by 1 jolts and {} differences by 3 jolts",
        differences.one, differences.three
    );

    println!(
        "Product of 1 jolt and 3 jolt differences {}",
        differences.one * differences.three
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_joltage_differences() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let differences = get_joltage_differences(22, &adapters);
        assert_eq!(differences.one, 7);
        assert_eq!(differences.three, 5);

        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let differences = get_joltage_differences(52, &adapters);
        assert_eq!(differences.one, 22);
        assert_eq!(differences.three, 10);
    }
}
