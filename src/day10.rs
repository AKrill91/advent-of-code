use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::collections::hash_map::RandomState;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let joltages = get_joltages(input);

    let diffs = find_diffs(&joltages);

    diffs.0 * diffs.2
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let joltages = get_joltages(input);
    let max = joltages[joltages.len() - 1];

    let joltage_set: HashSet<i64, RandomState> = HashSet::from_iter(joltages.iter().cloned());

    let mut counts: HashMap<i64, i64> = HashMap::new();

    counts.insert(0, 1);

    for joltage in joltages {
        let mut num_paths = 0;

        for diff in 1..4 {
            let j = joltage - diff;

            if joltage_set.contains(&j) {
                num_paths += counts.get(&j).unwrap();
            } else if j == 0 {
                num_paths += 1;
            }
        }

        debug!("Setting {} to {}", joltage, num_paths);
        counts.insert(joltage, num_paths);
    }

    *counts.get(&max).unwrap()
}


fn get_joltages<'a, I, J>(input: I) -> Vec<i64>
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut joltages = vec![];

    for line in input {
        let str = line.as_ref();

        joltages.push(str.parse::<i64>().unwrap());
    }

    joltages.sort();

    joltages
}

fn find_diffs(joltages: &Vec<i64>) -> (i64, i64, i64) {
    let mut diff_one_count = 0;
    let mut diff_two_count = 0;
    let mut diff_three_count = 0;

    let mut previous = 0;

    for i in 0..joltages.len() {
        let joltage = joltages[i];
        let diff = joltage - previous;
        previous = joltage;

        match diff {
            1 => diff_one_count += 1,
            2 => diff_two_count += 1,
            3 => diff_three_count +=1,
            _ => panic!()
        }
    }


    (diff_one_count, diff_two_count, diff_three_count + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample_0() -> Vec<&'static str> {
        vec![
            "16",
            "10",
            "15",
            "5",
            "1",
            "11",
            "7",
            "19",
            "6",
            "12",
            "4",
        ]
    }

    fn get_sample_1() -> Vec<&'static str> {
        vec![
            "28",
            "33",
            "18",
            "42",
            "31",
            "14",
            "46",
            "20",
            "48",
            "47",
            "24",
            "23",
            "49",
            "45",
            "19",
            "38",
            "39",
            "11",
            "1",
            "32",
            "25",
            "35",
            "8",
            "17",
            "7",
            "9",
            "4",
            "2",
            "34",
            "10",
            "3",
        ]
    }

    fn get_sample_2() -> Vec<&'static str> {
        vec![
            "16",
            "10",
            "15",
            "5",
            "1",
            "11",
            "7",
            "19",
            "6",
            "12",
            "4",
            "20",
            "21"
        ]
    }

    fn get_sample_3() -> Vec<&'static str> {
        vec![
            "1",
            "2",
            "3",
            "4"
        ]
    }

    fn get_sample_4() -> Vec<&'static str> {
        vec![
            "1",
            "2",
            "3",
            "4",
            "5"
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_0();

        assert_eq!(35, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_1();

        assert_eq!(220, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_0();

        assert_eq!(8, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_1();

        assert_eq!(19208, run_b(&input));
    }

    #[test]
    pub fn sample_input_2_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_2();

        assert_eq!(16, run_b(&input));
    }

    #[test]
    pub fn sample_input_3_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_3();

        assert_eq!(7, run_b(&input));
    }

    #[test]
    pub fn sample_input_4_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample_4();

        assert_eq!(13, run_b(&input));
    }
}