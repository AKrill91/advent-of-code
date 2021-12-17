use std::collections::HashMap;

pub fn run_a(_: i32, input: &[String]) -> String {
    format!("{}", run_days(80, input))
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("{}", run_days(256, input))
}

fn run_days(num_days: usize, input: &[String]) -> u64  {
    let mut fish = parse_fish(&input[0]);

    for _ in 0..num_days {
        fish = tick(fish);
    }

    fish.into_iter()
        .map(|(_, count)| count)
        .sum()
}

fn parse_fish(line: &str) -> HashMap<u8, u64> {
    line.split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .fold(HashMap::new(), |mut map, i| {
            let entry = map.entry(i).or_insert(0);

            *entry += 1;
            map
        })
}

fn tick(fish: HashMap<u8, u64>) -> HashMap<u8, u64> {
    let new_fish_count = *fish.get(&0).unwrap_or(&0);

    let mut out: HashMap<u8, u64> = HashMap::new();
    out.insert(8, new_fish_count);

    fish.iter()
        .map(|(timer, count)| {
            if *timer == 0 {
                (6, *count)
            } else {
                (*timer - 1, *count)
            }
        })
        .for_each(|(timer, count)| {
            let entry = out.entry(timer).or_default();
            *entry += count;
        });

    out
}

#[cfg(test)]
mod test {
    use crate::year2021::{run, run_day};
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse_fish() {
        let expected: HashMap<u8, u64> = vec![(1,1), (2,2), (3,3), (4,4)].into_iter()
            .collect();
        let actual = parse_fish("1,2,3,4,2,3,4,3,4,4");

        assert_eq!(expected, actual);
    }

    fn get_sample() -> Vec<String> {
        vec![
            "3,4,3,1,2"
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "5934";

        assert_eq!(expected, run_a(0, &input));
    }

    #[test]
    fn test_sample_a_small() {
        init();
        let input = get_sample();
        let expected = "26";
        assert_eq!(expected, run_days(18, &input).to_string());
    }

    #[test]
    fn test_sample_b() {
        init();

        let input = get_sample();

        let expected = "26984457539";

        assert_eq!(expected, run_b(0, &input));
    }
}