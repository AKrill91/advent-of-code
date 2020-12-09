use std::cmp::{max, min};

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    run_a_with_length(input, 25)
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    run_b_with_length(input, 25)
}

pub fn run_a_with_length<'a, I, J>(input: I, preamble_length: usize) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let numbers = read_numbers(input);

    find_weakness(&numbers, preamble_length)
}

pub fn run_b_with_length<'a, I, J>(input: I, preamble_length: usize) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let numbers = read_numbers(input);
    let weakness = find_weakness(&numbers, preamble_length);

    for set_start in 0..numbers.len() - 1 {
        let start_num = numbers[set_start];
        let mut sum = start_num;
        let mut set_max = start_num;
        let mut set_min = start_num;

        for set_end in set_start + 1..numbers.len() {
            let end_num = numbers[set_end];
            sum += end_num;
            set_max = max(set_max, end_num);
            set_min = min(set_min, end_num);

            if sum >= weakness {
                break;
            }
        }

        if sum == weakness {
            return set_min + set_max;
        }
    }

    -1
}

pub fn find_weakness(numbers: &Vec<i64>, preamble_length: usize) -> i64 {
    for i in preamble_length..numbers.len() {
        let n = numbers[i];
        let preamble_start = i - preamble_length;
        let preamble_end = i;

        let mut pair_found = false;

        for first in preamble_start..preamble_end - 1 {
            let p1 = numbers[first];
            let look_for = n - p1;

            for second in first + 1..preamble_end {
                let p2 = numbers[second];

                pair_found = pair_found || p2 == look_for;
            }
        }

        if !pair_found {
            return n;
        }
    }

    -1
}


fn read_numbers<'a, I, J>(input: I) -> Vec<i64>
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut numbers = vec![];

    for line in input {
        let s = line.as_ref();
        let num = s.parse::<i64>().unwrap();

        numbers.push(num);
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<&'static str> {
        vec![
            "35",
            "20",
            "15",
            "25",
            "47",
            "40",
            "62",
            "55",
            "65",
            "95",
            "102",
            "117",
            "150",
            "182",
            "127",
            "219",
            "299",
            "277",
            "309",
            "576",
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(127, run_a_with_length(&input, 5));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(62, run_b_with_length(&input, 5));
    }
}