use log::debug;
use regex::Regex;

pub fn run_a(input: &str) -> i64 {
    let input = input.trim();
    let mut left = vec![];
    let mut right = vec![];

    let whitespace = Regex::new("\\s+").unwrap();

    input.lines()
        .for_each(|line| {
            let mut parts = whitespace.split(line);
            let l = parts.next().unwrap().parse::<i64>().unwrap();
            let r = parts.next().unwrap().parse::<i64>().unwrap();
            left.push(l);
            right.push(r);
        });

    left.sort();
    right.sort();

    let mut total_distance = 0;

    for i in 0..left.len() {
        total_distance += (right[i] - left[i]).abs();
    }

    total_distance
}

#[cfg(test)]
mod test {
    use crate::day01::run_a;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn part_a_example() {
        init();

        let input = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

        let distance = run_a(input);

        assert_eq!(distance, 11);
    }
}