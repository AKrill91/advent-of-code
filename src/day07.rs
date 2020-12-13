use std::collections::HashMap;

use regex::Regex;
use std::cell::RefCell;

pub fn run_a(input: &Vec<String>) -> i64 {
    let rules = BagRules::from(input);

    rules.num_containing("shiny gold") as i64
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let rules = BagRules::from(input);

    rules.num_inside("shiny gold") as i64
}

struct BagRules {
    bags: HashMap<String, Bag>,
    count_cache: RefCell<HashMap<String, i64>>
}

impl From<&Vec<String>> for BagRules {
    fn from(input: &Vec<String>) -> Self {
        let mut bags = HashMap::with_capacity(input.len());

        for line in input {
            let bag = Bag::from(line.as_str());

            bags.insert(bag.color.clone(), bag);
        }

        BagRules {
            bags,
            count_cache: RefCell::new(HashMap::new())
        }
    }
}

impl BagRules {
    pub fn num_containing(&self, color: &str) -> usize {
        let mut count = 0;

        for bag in self.bags.values() {
            if bag.can_contain(self, color) {
                count += 1;
            }
        }

        count
    }

    pub fn num_inside(&self, color: &str) -> i64 {
        let contains = {
            self.count_cache.borrow().get(color).cloned()
        };

        if let Some(count) = contains {
            count
        } else {
            let bag = self.bags.get(color).unwrap();
            let count = bag.num_inside(self);
            self.count_cache.borrow_mut().insert(color.to_string(), count);

            count
        }
    }
}

struct Bag {
    color: String,
    contents: HashMap<String, usize>,
}

impl From<&str> for Bag {
    fn from(input: &str) -> Self {
        let pattern = Regex::new(r"^([a-z ]+) bags contain (.*)\.").unwrap();

        if !pattern.is_match(input) {
            panic!();
        }

        let captures = pattern.captures(input).unwrap();

        let color = captures.get(1).unwrap().as_str().to_string();
        let contents = captures.get(2).unwrap().as_str();

        let parts = contents.split(",");
        let mut bags = HashMap::new();

        for part in parts {
            let pattern = Regex::new(r"(no|\d+) ([a-z ]+) bags?[.,]?").unwrap();

            if !pattern.is_match(part) {
                panic!();
            }

            let captures = pattern.captures(part).unwrap();

            let amount_str = captures.get(1).unwrap().as_str();

            if amount_str == "no" {
                continue;
            }

            let amount = amount_str.parse::<usize>().unwrap();

            bags.insert(captures.get(2).unwrap().as_str().to_string(), amount);
        }

        Bag {
            color,
            contents: bags,
        }
    }
}

impl Bag {
    pub fn can_contain(&self, rules: &BagRules, color: &str) -> bool {
        if self.contents.contains_key(color) {
            return true;
        } else {
            for (bag_color, _) in &self.contents {
                let bag = rules.bags.get(bag_color).unwrap();

                if bag.can_contain(rules, color) {
                    return true;
                }
            }
        }

        false
    }

    pub fn num_inside(&self, rules: &BagRules) -> i64 {
        let mut count = 0;

        for (color, amount) in &self.contents {
            let c = rules.num_inside(color.as_str());

            count += (c + 1) * (*amount as i64);
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<String> {
        vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags."),
        ]
    }

    pub fn get_sample_1() -> Vec<String> {
        vec![
            String::from("shiny gold bags contain 2 dark red bags."),
            String::from("dark red bags contain 2 dark orange bags."),
            String::from("dark orange bags contain 2 dark yellow bags."),
            String::from("dark yellow bags contain 2 dark green bags."),
            String::from("dark green bags contain 2 dark blue bags."),
            String::from("dark blue bags contain 2 dark violet bags."),
            String::from("dark violet bags contain no other bags."),
        ]
    }

    #[test]
    pub fn bag_from_str() {
        let input = get_sample()[0].clone();

        let bag = Bag::from(input.as_str());

        let color = String::from("light red");
        let contents_1 = String::from("bright white");
        let contents_2 = String::from("muted yellow");

        assert_eq!(2, bag.contents.len());
        assert_eq!(color, bag.color);
        assert!(bag.contents.contains_key(&contents_1));
        assert!(bag.contents.contains_key(&contents_2));
    }

    #[test]
    pub fn bag_rules_colors() {
        let input = get_sample();

        let rules = BagRules::from(&input);
        let colors = rules.colors();

        assert_eq!(13, colors.len());
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(4, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(32, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_1();

        assert_eq!(126, run_b(&input));
    }
}