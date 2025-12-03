use std::str::FromStr;

fn is_valid_id(id: i64) -> bool {
    let as_str = id.to_string();

    if as_str.len() % 2 != 0 {
        true
    } else {
        let half = as_str.len() / 2;
        let first_half = &as_str[0..half];
        let second_half = &as_str[half..];

        first_half.ne(second_half)
    }
}

fn is_more_valid_id(id: i64) -> bool {
    let as_str = id.to_string();
    log::trace!("Checking {}", as_str);
    let len = as_str.len();
    let half_size = len / 2;

    'size: for slice_length in (1..=half_size) {
        log::trace!("Checking slice of length {}", slice_length);
        if len % slice_length != 0 {
            continue;
        }

        let amount = len / slice_length;

        let pattern = &as_str[0..slice_length];

        for i in 1..amount {
            let start = i * slice_length;
            let end = (i * slice_length) + slice_length;
            log::trace!("Checking slice {}, start: {}, end: {}", i, start, end);
            let other = &as_str[start..end];

            if pattern != other {
                log::trace!("{} != {}", pattern, other);
                continue 'size;
            }
        }

        return false
    }

    true
}

#[derive(Debug, Eq, PartialEq)]
struct ProductRange {
    first: i64,
    last: i64,
}

impl FromStr for ProductRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let first = parts.next().unwrap().parse::<i64>().unwrap();
        let last = parts.next().unwrap().parse::<i64>().unwrap();
        Ok(ProductRange { first, last })
    }
}

impl ProductRange {
    fn new(first: i64, last: i64) -> ProductRange {
        ProductRange { first, last }
    }

    fn basic_invalid_ids(&self) -> Vec<i64> {
        (self.first..=self.last)
            .filter(|i| !is_valid_id(*i))
            .collect()
    }

    fn all_invalid_ids(&self) -> Vec<i64> {
        (self.first..=self.last)
            .filter(|i| !is_more_valid_id(*i))
            .collect()
    }
}

fn parse(input: &str) -> Vec<ProductRange> {
    input.trim()
        .split(',')
        .map(|line| line.parse::<ProductRange>().unwrap())
        .collect()
}
pub fn run_a(input: &str) -> i64 {
    let ranges = parse(input);

    ranges.iter()
        .map(|r| r.basic_invalid_ids().iter().sum::<i64>())
        .sum()
}

pub fn run_b(input: &str) -> i64 {
    let ranges = parse(input);

    ranges.iter()
        .map(|r| r.all_invalid_ids().iter().sum::<i64>())
        .sum()
}

#[cfg(test)]
mod test {
    use super::ProductRange;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());

        assert_eq!(11, parsed.len());
        assert_eq!(ProductRange::new(11, 22), parsed[0]);
        assert_eq!(ProductRange::new(2121212118, 2121212124), parsed[10]);

    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(1227775554, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(4174379265, super::run_b(example()));
    }

    mod is_valid_id {
        use super::super::is_valid_id;

        #[test]
        fn single_repeat() {
            assert!(!is_valid_id(11));
        }

        #[test]
        fn double_repeat() {
            assert!(!is_valid_id(1010));
        }

        #[test]
        fn triple_repeat() {
            assert!(!is_valid_id(123123));
        }
    }

    mod is_more_valid_id {
        use super::super::is_more_valid_id;

        #[test]
        fn single_digit_repeat_twice() {
            super::init();
            assert!(!is_more_valid_id(11));
        }

        #[test]
        fn single_digit_repeat_thrice() {
            super::init();
            assert!(!is_more_valid_id(111));
        }

        #[test]
        fn two_digit_repeat_twice() {
            super::init();
            assert!(!is_more_valid_id(1010));
        }

        #[test]
        fn triple_digit_repeat_twice() {
            super::init();
            assert!(!is_more_valid_id(123123));
        }


    }

    mod product_range {
        use super::*;
        use super::super::*;

        #[test]
        fn example_one() {
            init();

            let range = ProductRange::new(11, 22);

            let invalid_ids = range.basic_invalid_ids();

            assert_eq!(2, invalid_ids.len());
            assert_eq!(invalid_ids[0], 11);
            assert_eq!(invalid_ids[1], 22);
        }

        #[test]
        fn example_two() {
            init();

            let range = ProductRange::new(95, 115);

            let invalid_ids = range.basic_invalid_ids();

            assert_eq!(1, invalid_ids.len());
            assert_eq!(invalid_ids[0], 99);
        }

        #[test]
        fn example_three() {
            init();

            let range = ProductRange::new(998, 1012);

            let invalid_ids = range.basic_invalid_ids();

            assert_eq!(1, invalid_ids.len());
            assert_eq!(invalid_ids[0], 1010);
        }

        #[test]
        fn example_four() {
            init();

            let range = ProductRange::new(1188511880, 1188511890);

            let invalid_ids = range.basic_invalid_ids();

            assert_eq!(1, invalid_ids.len());
            assert_eq!(invalid_ids[0], 1188511885);
        }
    }
}