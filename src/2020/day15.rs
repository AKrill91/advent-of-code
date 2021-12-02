use std::collections::{HashMap, HashSet};

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    run(input, 2020)
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    run(input, 30_000_000)
}

pub fn run<'a, I, J>(input: I, goal: usize) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut numbers: Vec<i64> = vec![];
    let mut turn_map: HashMap<i64, usize> = HashMap::new();

    let input = input.into_iter().next().unwrap();

    for (turn, num) in input.as_ref().split(",").enumerate() {
        let n = num.parse::<i64>().unwrap();
        numbers.push(n);
    }

    for i in 0..(numbers.len() - 1) {
        let n = numbers[i];
        turn_map.insert(n, i);
    }

    for i in numbers.len()..goal {
        let last_turn = i - 1;
        let last_num = numbers[last_turn];

        let new_num = if let Some(n) = turn_map.get(&last_num) {
            last_turn - *n
        } else {
            0
        };

        turn_map.insert(last_num, last_turn);
        numbers.push(new_num as i64);
    }

    numbers[goal - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "0,3,6"
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(436, run_a(&sample));
    }

    #[test]
    pub fn more_sample_input_a() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(1, run_a(vec!["1,3,2"]));
        assert_eq!(10, run_a(vec!["2,1,3"]));
        assert_eq!(27, run_a(vec!["1,2,3"]));
        assert_eq!(78, run_a(vec!["2,3,1"]));
        assert_eq!(438, run_a(vec!["3,2,1"]));
        assert_eq!(1836, run_a(vec!["3,1,2"]));
    }

    #[test]
    #[ignore]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(175594, run_b(&sample));
    }

    #[test]
    #[ignore]
    pub fn more_sample_input_b() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert_eq!(2578, run_b(vec!["1,3,2"]));
        assert_eq!(3544142, run_b(vec!["2,1,3"]));
        assert_eq!(261214, run_b(vec!["1,2,3"]));
        assert_eq!(6895259, run_b(vec!["2,3,1"]));
        assert_eq!(18, run_b(vec!["3,2,1"]));
        assert_eq!(362, run_b(vec!["3,1,2"]));
    }
}