use log::debug;
use regex::Regex;

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
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

    (left, right)
}

pub fn run_a(input: &str) -> i64 {
    let (left, right) = parse(input);

    let mut total_distance = 0;

    for i in 0..left.len() {
        total_distance += (right[i] - left[i]).abs();
    }

    total_distance
}

pub fn run_b(input: &str) -> i64 {
    let (left, right) = parse(input);

    let mut similarity_score = 0;

    for l in left {
        let num_occurences = right.iter().filter(|x| **x == l).count();
        similarity_score += l * num_occurences as i64;
    }

    similarity_score
}

#[cfg(test)]
mod test {

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn input() -> &'static str {
        r"
3   4
4   3
2   5
1   3
3   9
3   3
"
    }

    #[test]
    fn part_a_example() {
        init();

        let distance = super::run_a(input());

        assert_eq!(distance, 11);
    }

    #[test]
    fn part_b_example() {
        let score = super::run_b(input());
        assert_eq!(score, 31)
    }
}