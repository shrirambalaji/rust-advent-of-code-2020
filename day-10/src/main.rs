use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn get_input_device_joltage(joltages: &Vec<u64>) -> u64 {
    let max_joltage = joltages.iter().max().unwrap();
    return max_joltage + 3;
}

struct JoltageDifference {
    one: u64,
    two: u64,
    three: u64,
}

fn get_joltage_differences(device_voltage: u64, adapters: &Vec<u64>) -> JoltageDifference {
    let mut difference = JoltageDifference {
        one: 1,
        two: 1,
        three: 1,
    };

    let joltages: HashSet<u64> = HashSet::from_iter(adapters.clone());
    let mut used_adapters: HashSet<u64> = HashSet::new();

    let mut use_joltage_adapter = |joltage: u64, increment: u64| {
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

/// recursively find the sum of all the possible arrangements
fn sum_possible_arrangements(
    joltage: u64,
    adapters: &Vec<u64>,
    memo: &mut HashMap<u64, u64>,
) -> u64 {
    let max_joltage = adapters.iter().max().unwrap();
    // there's only one possible arrangement when the joltage of a given adapter, equals the maximum possible adapter joltage itself.
    if joltage == *max_joltage {
        return 1;
    }

    // if the adapter has already been used, the number of possible arrangements when it was used is it's memoized and we return the memoized value instead.
    if memo.contains_key(&joltage) {
        return *memo.get(&joltage).unwrap();
    }

    // possible adapters are adapters whose joltages are <= current_joltage + 3
    // for eg. if the adapters are [1, 4, 5, 6, 7] when the current joltage is 4, the possible adapters are 5, 6 and 7.
    let possible_adapters = adapters
        .iter()
        .filter(|curr| {
            // an adapter can be used only if the voltage difference between them is less than or equal to 3.
            return **curr == joltage + 1 || **curr == joltage + 2 || **curr == joltage + 3;
        })
        .map(|x| x.to_owned())
        .collect::<Vec<u64>>();

    // recursively go through all the possible joltages, and find sum of all possible arrangements with the other adapters, for the specified joltage
    let results = possible_adapters
        .iter()
        .map(|possible_joltage| {
            let number_of_arrangements =
                sum_possible_arrangements(*possible_joltage, adapters, memo);
            memo.insert(*possible_joltage, number_of_arrangements);
            number_of_arrangements
        })
        .sum();

    results
}

fn find_distinct_possible_adapter_arrangements(
    charging_outlet_joltage: u64,
    adapters: &Vec<u64>,
) -> u64 {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    sum_possible_arrangements(charging_outlet_joltage, adapters, &mut memo)
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    let adapters: Vec<u64> = input
        .lines()
        .map(|x| {
            x.parse::<u64>()
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

    let sum = find_distinct_possible_adapter_arrangements(0, &adapters);
    println!("There are {} distinct ways to arrange the adapters", sum);
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

    #[test]
    fn should_find_distinct_ways_to_arrange_adapters() {
        let adapters = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
        let sum = find_distinct_possible_adapter_arrangements(0, &adapters);

        assert_eq!(sum, 8);

        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let sum = find_distinct_possible_adapter_arrangements(0, &adapters);

        assert_eq!(sum, 19208);
    }
}
