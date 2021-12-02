use regex::Regex;

pub fn run_a(input: &Vec<String>) -> i64 {
    let mut valid = 0;

    for line in input {
        let (policy, password) = parse_line(&line);

        if policy.check_count(&password){
            valid += 1;
        }
    }

    valid
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let mut valid = 0;

    for line in input {
        let (policy, password) = parse_line(&line);

        if policy.check_position(&password){
            valid += 1;
        }
    }

    valid
}

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    pub fn check_count(&self, password: &str) -> bool {
        let mut letter_match = 0;

        for c in password.chars() {
            if c == self.letter {
                letter_match += 1;
            }
        }

        letter_match >= self.min && letter_match <= self.max
    }

    pub fn check_position(&self, password: &str) -> bool {
        let first = password.chars().nth(self.min - 1).unwrap() == self.letter;
        let second = password.chars().nth(self.max - 1).unwrap() == self.letter;

        first ^ second
    }
}

fn parse_line(line: &str) -> (Policy, String) {

    let pattern = Regex::new(r"(\d{1,5})-(\d{1,5}) ([a-z]): ([a-z]{1,30})").unwrap();

    if !pattern.is_match(line) {
        panic!();
    }

    let captures = pattern.captures(line).unwrap();

    let policy = Policy {
        min: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        max: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        letter: captures.get(3).unwrap().as_str().chars().next().unwrap()
    };

    let password = String::from(captures.get(4).unwrap().as_str());

    (policy, password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];

        assert_eq!(2, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];

        assert_eq!(1, run_b(&input));
    }
}