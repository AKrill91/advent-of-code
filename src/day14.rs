use std::cmp::max;
use std::collections::HashMap;
use std::fmt;

use regex::Regex;

const ORE_STR: &str = "ORE";
const FUEL_STR: &str = "FUEL";

struct Quantity<'a> {
    ingredient: &'a str,
    amount: i32,
}

impl fmt::Debug for Quantity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\"{}\": {}}}", self.ingredient, self.amount)
    }
}

struct RecipeBook<'a> {
    recipes: HashMap<&'a str, Recipe<'a>>
}

impl RecipeBook<'_> {
    pub fn new(input: &Vec<String>) -> RecipeBook {
        let mut recipes = HashMap::new();
        recipes.insert(ORE_STR, Recipe::ore());

        let mut recipe_inputs: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
        let mut recipe_outputs: HashMap<&str, Quantity> = HashMap::new();
        let pattern = Regex::new(r"(\d{1,5}) ([A-Z]{1,9})").unwrap();

        for line in input {
            let mut inputs = HashMap::new();
            let mut output = Quantity { ingredient: "", amount: 0 };
            let mut part_num = 0;
            for part in line.split("=>") {
                if part_num == 0 {
                    for ingredient in part.split(",") {
                        if !pattern.is_match(ingredient) {
                            panic!();
                        }

                        let captures = pattern.captures(ingredient).unwrap();

                        inputs.insert(
                            captures.get(2).unwrap().as_str(),
                            captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        );
                    }
                } else {
                    if !pattern.is_match(part) {
                        panic!();
                    }

                    let captures = pattern.captures(part).unwrap();

                    output = Quantity {
                        ingredient: captures.get(2).unwrap().as_str(),
                        amount: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    };
                }

                part_num += 1;
            }

            recipe_inputs.insert(output.ingredient, inputs);
            recipe_outputs.insert(output.ingredient, output);
        }

        while !recipes.contains_key(FUEL_STR) {
            debug!("{} recipes in book so far", recipes.len());
            for (output, inputs) in &recipe_inputs {
                if recipes.contains_key(output) {
                    continue;
                }

                let mut recipe_level = 0;
                let mut all_ingredients_added = true;

                for (ingredient, amount) in inputs {
                    if !recipes.contains_key(ingredient) {
                        all_ingredients_added = false;
                        break;
                    }

                    let ingredient_recipe = recipes.get(ingredient).unwrap();

                    recipe_level = max(recipe_level, ingredient_recipe.level + 1);
                }

                if all_ingredients_added {
                    let inputs = inputs.iter()
                        .map(|(ingredient, amount)| Quantity { ingredient, amount: *amount })
                        .collect();

                    recipes.insert(
                        output,
                        Recipe {
                            inputs,
                            output: recipe_outputs.remove(output).unwrap(),
                            level: recipe_level,
                        },
                    );
                }
            }
        }

        RecipeBook {
            recipes
        }
    }

    pub fn num_recipes(&self) -> usize {
        self.recipes.len()
    }

    pub fn get_basic_ingredients(&self) -> Vec<&str> {
        self.recipes.values()
            .filter(|recipe| recipe.is_basic())
            .map(|recipe| recipe.output.ingredient)
            .collect()
    }

    pub fn get_max_level(&self) -> usize {
        self.recipes.values()
            .map(|recipe| recipe.level)
            .max()
            .unwrap()
    }

    pub fn get_fuel_recipe(&self) -> &Recipe {
        self.recipes.get(FUEL_STR).unwrap()
    }

    pub fn recipes_for_level(&self, level: usize) -> Vec<&Recipe> {
        self.recipes.values()
            .filter(|recipe| recipe.level == level)
            .collect()
    }
}

#[derive(Debug)]
struct Recipe<'a> {
    inputs: Vec<Quantity<'a>>,
    output: Quantity<'a>,
    level: usize,
}

impl<'a> Recipe<'a> {
    pub fn ingredient(&self) -> &'a str {
        self.output.ingredient
    }

    pub fn amount_required(&self, ingredient: &str) -> i32 {
        let input: Vec<&Quantity<'a>> = self.inputs.iter()
            .filter(|qty| qty.ingredient == ingredient)
            .collect();

        input.get(0).unwrap().amount
    }

    pub fn is_basic(&self) -> bool {
        self.inputs.len() == 1 &&
            self.inputs.iter().any(|qty| qty.ingredient == ORE_STR)
    }

    pub fn makes_fuel(&self) -> bool {
        self.output.ingredient == "FUEL"
    }

    fn ore() -> Recipe<'a> {
        Recipe {
            inputs: vec![
                Quantity {
                    ingredient: ORE_STR,
                    amount: 1,
                }
            ],
            output: Quantity {
                ingredient: ORE_STR,
                amount: 1,
            },
            level: 0,
        }
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let recipe_book = RecipeBook::new(input);

    let basic_ingredients: Vec<&str> = recipe_book.get_basic_ingredients();

    info!("Found {} recipes and {} basic ingredients: {:?}", recipe_book.num_recipes(), basic_ingredients.len(), basic_ingredients);

    let fuel_recipe = recipe_book.get_fuel_recipe();

    debug!("Fuel recipe: {:?}", fuel_recipe);

    let mut shopping_list = HashMap::new();
    shopping_list.insert(FUEL_STR, 1);

    let mut ore_needed = 0;

    for level in (1..recipe_book.get_max_level() + 1).rev() {
        let recipes = recipe_book.recipes_for_level(level);

        info!("{} level {} recipes - {:?}", recipes.len(), level, recipes);

        for recipe in recipes {
            let ingredient = recipe.ingredient();
            let amount_needed = shopping_list.get(ingredient);

            if amount_needed.is_none() {
                info!("Don't need to make any {}", ingredient);
                continue;
            }

            let amount_needed = *amount_needed.unwrap();
            let amount_per_synth = recipe.output.amount;

            let num_synths = match amount_needed % amount_per_synth {
                0 => {
                    amount_needed / amount_per_synth
                }
                _ => {
                    (amount_needed / amount_per_synth) + 1
                }
            };

            debug!("{} synths needed", num_synths);

            if recipe.is_basic() {
                let ore_per_synth = recipe.amount_required(ORE_STR);
                ore_needed += ore_per_synth * num_synths;
            } else {
                for quantity in &recipe.inputs {
                    let ingredient = quantity.ingredient;
                    let amount_per_synth = quantity.amount;
                    let amount = num_synths * amount_per_synth;
                    debug!("Adding {} {} to shopping list", ingredient, amount);

                    *shopping_list.entry(ingredient).or_insert(0) += amount;
                }
            }
        }
    }

    ore_needed
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("10 ORE => 10 A"),
            String::from("1 ORE => 1 B"),
            String::from("7 A, 1 B => 1 C"),
            String::from("7 A, 1 C => 1 D"),
            String::from("7 A, 1 D => 1 E"),
            String::from("7 A, 1 E => 1 FUEL"),
        ];

        assert_eq!(31, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![
            String::from("9 ORE => 2 A"),
            String::from("8 ORE => 3 B"),
            String::from("7 ORE => 5 C"),
            String::from("3 A, 4 B => 1 AB"),
            String::from("5 B, 7 C => 1 BC"),
            String::from("4 C, 1 A => 1 CA"),
            String::from("2 AB, 3 BC, 4 CA => 1 FUEL"),
        ];

        assert_eq!(165, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![
            String::from("157 ORE => 5 NZVS"),
            String::from("165 ORE => 6 DCFZ"),
            String::from("44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL"),
            String::from("12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ"),
            String::from("179 ORE => 7 PSHF"),
            String::from("177 ORE => 5 HKGWZ"),
            String::from("7 DCFZ, 7 PSHF => 2 XJWVT"),
            String::from("165 ORE => 2 GPVTF"),
            String::from("3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"),
        ];

        assert_eq!(13312, run_a(&input));
    }

    #[test]
    pub fn sample_input_3_a() {
        let input = vec![
            String::from("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG"),
            String::from("17 NVRVD, 3 JNWZP => 8 VPVL"),
            String::from("53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL"),
            String::from("22 VJHF, 37 MNCFX => 5 FWMGM"),
            String::from("139 ORE => 4 NVRVD"),
            String::from("144 ORE => 7 JNWZP"),
            String::from("5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC"),
            String::from("5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV"),
            String::from("145 ORE => 6 MNCFX"),
            String::from("1 NVRVD => 8 CXFTF"),
            String::from("1 VJHF, 6 MNCFX => 4 RFSQX"),
            String::from("176 ORE => 6 VJHF"),
        ];

        assert_eq!(180697, run_a(&input));
    }

    #[test]
    pub fn sample_input_4_a() {
        let input = vec![
            String::from("171 ORE => 8 CNZTR"),
            String::from("7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL"),
            String::from("114 ORE => 4 BHXH"),
            String::from("14 VRPVC => 6 BMBT"),
            String::from("6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL"),
            String::from("6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT"),
            String::from("15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW"),
            String::from("13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW"),
            String::from("5 BMBT => 4 WPTQ"),
            String::from("189 ORE => 9 KTJDG"),
            String::from("1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP"),
            String::from("12 VRPVC, 27 CNZTR => 2 XDBXC"),
            String::from("15 KTJDG, 12 BHXH => 5 XCVML"),
            String::from("3 BHXH, 2 VRPVC => 7 MZWV"),
            String::from("121 ORE => 7 VRPVC"),
            String::from("7 XCVML => 6 RJRHP"),
            String::from("5 BHXH, 4 VRPVC => 5 LTCX"),
        ];

        assert_eq!(2210736, run_a(&input));
    }
}