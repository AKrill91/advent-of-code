use std::collections::HashSet;

pub fn run_a(input: &Vec<String>) -> i64 {
    let target = 2020;
    let entries = get_entries(input);

    find_parts(&entries, target, 2).unwrap()
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let target = 2020;
    let entries = get_entries(input);

    find_parts(&entries, target, 3).unwrap()
}

fn find_parts(entries: &HashSet<i64>, target: i64, count: usize) -> Option<i64> {
    for entry in entries.iter() {
        let remainder = target - entry;

        if count == 2 {
            if entries.contains(&remainder) {
                debug!("Entries are {} and {}", entry, remainder);
                return Some(entry * remainder);
            }
        } else {
            if let Some(i) = find_parts(&entries, remainder, count - 1) {
                debug!("Entries are {} and {}", entry, i);

                return Some(entry * i);
            }
        }
    }

    None
}

fn get_entries(input: &Vec<String>) -> HashSet<i64> {
    let mut entries: HashSet<i64> = Default::default();

    for line in input {
        let entry = line.parse::<i64>().unwrap();

        entries.insert(entry);
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("1721"),
            String::from("979"),
            String::from("366"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        ];

        assert_eq!(514_579, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("1720"),
            String::from("979"),
            String::from("366"),
            String::from("300"),
            String::from("675"),
            String::from("1456"),
        ];

        assert_eq!(516_000, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("1721"),
            String::from("979"),
            String::from("366"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        ];

        assert_eq!(241_861_950, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("1721"),
            String::from("980"),
            String::from("365"),
            String::from("299"),
            String::from("675"),
            String::from("1456"),
        ];

        assert_eq!(241_447_500, run_b(&input));
    }
}