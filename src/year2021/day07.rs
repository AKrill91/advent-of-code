use std::cmp::min;

pub fn run_a(_: i32, input: &[String]) -> String {
    let crabs = parse_crabs(&input[0]);

    let min_position = *crabs.iter()
        .min()
        .unwrap();

    let max_position = *crabs.iter()
        .max()
        .unwrap();

    let mut min_fuel_cost = i32::MAX;

    for pos in min_position..=max_position {
        min_fuel_cost = min_fuel_cost.min(crabs.iter()
            .map(|crab| (pos - crab).abs())
            .sum()
        );
    }

    format!("{}", min_fuel_cost)
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("")
}

fn parse_crabs(input: &str) -> Vec<i32> {
    input.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_sample() -> Vec<String> {
        vec![
            "16,1,2,0,4,2,7,1,2,14"
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "37";

        assert_eq!(expected, run_a(0, &input));
    }

    #[test]
    fn test_sample_b() {
        init();

        let input = get_sample();

        let expected = "";

        assert_eq!(expected, run_b(0, &input));
    }
}