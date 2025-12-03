use std::fmt::{Display, Formatter};
use std::str::FromStr;
use itertools::Itertools;

type Battery = u8;
type Joltage = u64;

const JOLTAGE_A: usize = 2;
const JOLTAGE_B: usize = 12;


struct BatteryBank {
    batteries: Vec<Battery>,
}

impl Display for BatteryBank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.batteries.iter().map(|i| char::from_digit((*i) as u32, 10).unwrap()).join(""))
    }
}

fn joltage(batteries: &[Battery], size: usize) -> Joltage {
    assert!(size <= batteries.len());
    let slice_end = batteries.len() - (size - 1);
    let possibly_first = &batteries[..slice_end];
    let largest_first_rating = *possibly_first.iter().max().unwrap();

    if size == 1 {
        return largest_first_rating as Joltage;
    }

    largest_first_rating as Joltage * Joltage::pow(10, (size - 1) as u32) +
        possibly_first.iter().enumerate().filter(|(_, &rating)| rating == largest_first_rating)
            .map(|(index, _)| {
                joltage(&batteries[index + 1..], size - 1)
            })
            .max()
            .unwrap()
}

impl FromStr for BatteryBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut batteries: Vec<Battery> = Vec::new();

        for c in s.chars() {
            batteries.push(c.to_digit(10).ok_or(())? as Battery);
        }

        Ok(BatteryBank::new(batteries))
    }
}

impl BatteryBank {
    fn new(batteries: Vec<Battery>) -> BatteryBank {
        BatteryBank { batteries }
    }

    fn joltage(&self, count: usize) -> Joltage {
        log::debug!("Calculating joltage for {} batteries from bank {}", count, self);
        joltage(&self.batteries, count)
    }
}

fn parse(input: &str) -> Vec<BatteryBank> {
    input.lines()
        .map(|l| BatteryBank::from_str(l).unwrap())
        .collect()
}
pub fn run_a(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|bank| bank.joltage(JOLTAGE_A))
        .sum()
}

pub fn run_b(input: &str) -> u64 {
    parse(input)
        .iter()
        .map(|bank| bank.joltage(JOLTAGE_B))
        .sum()
}

#[cfg(test)]
mod test {

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"987654321111111
811111111111119
234234234234278
818181911112111"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(357, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(3121910778619, super::run_b(example()));
    }

    mod battery_bank {
        use super::*;
        use super::super::*;
        use std::str::FromStr;

        #[test]
        fn example_one() {
            init();
            let bank = BatteryBank::from_str(example().lines().nth(0).unwrap()).unwrap();

            assert_eq!(98, bank.joltage(JOLTAGE_A));
            assert_eq!(987654321111, bank.joltage(JOLTAGE_B));
        }

        #[test]
        fn example_two() {
            init();
            let bank = BatteryBank::from_str(example().lines().nth(1).unwrap()).unwrap();

            assert_eq!(89, bank.joltage(JOLTAGE_A));
            assert_eq!(811111111119, bank.joltage(JOLTAGE_B));
        }

        #[test]
        fn example_three() {
            init();
            let bank = BatteryBank::from_str(example().lines().nth(2).unwrap()).unwrap();

            assert_eq!(78, bank.joltage(JOLTAGE_A));
            assert_eq!(434234234278, bank.joltage(JOLTAGE_B));
        }

        #[test]
        fn example_four() {
            init();
            let bank = BatteryBank::from_str(example().lines().nth(3).unwrap()).unwrap();

            assert_eq!(92, bank.joltage(JOLTAGE_A));
            assert_eq!(888911112111, bank.joltage(JOLTAGE_B));
        }
    }
}