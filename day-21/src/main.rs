#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env, fs,
};

#[derive(Debug)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: Vec<&'a str>,
}

fn process_food(food: &str) -> Food {
    lazy_static! {
        static ref FOOD_REGEX: Regex = Regex::new(r"(\w.*)\(contains\s+(\w+.*)\)").unwrap();
    }

    let food = food.trim();
    let captures = match FOOD_REGEX.captures(food) {
        Some(captures) => captures,
        None => panic!("Invalid food item {}", food),
    };

    let ingredients: HashSet<&str> = captures
        .get(1)
        .unwrap()
        .as_str()
        .split_ascii_whitespace()
        .collect();

    let allergens: Vec<&str> = captures.get(2).unwrap().as_str().split(", ").collect();

    Food {
        ingredients,
        allergens,
    }
}

struct Processed<'a> {
    ingredients_without_allergens: HashMap<&'a str, u32>,
    ingredients_with_allergens: Vec<&'a str>,
}

fn process_food_items(input: &str) -> Processed {
    let lines: Vec<&str> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.trim())
        .collect();

    let mut foods: Vec<Food> = lines.iter().map(|x| process_food(*x)).collect();

    // dairy, fish, soy
    let mut unknown_allergens: HashSet<&str> = foods
        .iter()
        .map(|f| f.allergens.iter())
        .flatten()
        .cloned()
        .collect();

    // we use a BTreeMap in order for the keys to be sorted
    // this is necessary for Part 02
    let mut known_allergens: BTreeMap<&str, &str> = BTreeMap::new();

    // iterate through all the unknown allergens, until there are none left.
    'outer: loop {
        for allergen in &unknown_allergens.clone() {
            // find all the possible foods that have the specified unknown allergen
            let foods_with_allergen: Vec<&Food> = foods
                .iter()
                .filter(|f| f.allergens.contains(&allergen))
                .collect();

            // we need to find ingredients that match across foods, and have the specified allergen.
            // for that we essentially intersect between the ingredients across foods ie. find the common ingredient across foods with the same allergen.
            let init: HashSet<&str> = foods_with_allergen[0].ingredients.clone();
            let candidate_ingredients: HashSet<&str> =
                foods_with_allergen.iter().fold(init.to_owned(), |i, f| {
                    i.intersection(&f.ingredients).cloned().collect()
                });

            // when we have narrowed down a single candidate ingredient
            // we have found the ingredient responsible for the allergen
            // so we will remove it from our foods ingredients, so that only the foods without allergens remain in it.
            // also since we know the ingredient, we add it to the `known_allergens` list
            if candidate_ingredients.len() == 1 {
                let ingredient = candidate_ingredients.iter().next().unwrap();
                for f in foods.iter_mut() {
                    f.ingredients.remove(ingredient);
                }
                known_allergens.insert(allergen, ingredient);
                unknown_allergens.remove(allergen);
            }

            if unknown_allergens.is_empty() {
                break 'outer;
            }
        }
    }

    // as mentioned earlier, after the above loop is complete
    // the foods list only has ingredients that DONT have allergens
    // we need to sum of count of these ingredients as the result for Part 01.
    let ingredients_without_allergens: HashMap<&str, u32> = foods
        .iter()
        .map(|f| f.ingredients.clone())
        .flatten()
        .fold(HashMap::new(), |mut acc, value| {
            acc.entry(value).and_modify(|e| *e += 1).or_insert(1);
            acc
        });

    let danger_list: Vec<&str> = known_allergens.values().cloned().collect();

    Processed {
        ingredients_with_allergens: danger_list,
        ingredients_without_allergens,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input file");

    let Processed {
        ingredients_without_allergens,
        ingredients_with_allergens,
    } = process_food_items(&input);

    // -- Part One --
    let sum: u32 = ingredients_without_allergens
        .values()
        .map(|x| x.to_owned())
        .sum();

    println!("Sum of ingredients without allergens: {}", sum);

    // -- Part Two --
    println!(
        "Canonical Dangerous List: {:?}",
        ingredients_with_allergens.join(",")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_process_food() {
        process_food("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)");
    }

    #[test]
    fn should_process_food_items_list() {
        let input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)"#;
        process_food_items(input);
    }
}
