use std::collections::{BTreeSet, HashMap, HashSet};

use regex::{Captures, Regex};

use day19::MatchCriteria::{Complex, Constant, Simple};

lazy_static! {
    static ref CONST_PATTERN: Regex = Regex::new(r#"(\d+): "([ab])""#).unwrap();
    static ref SIMPLE_PATTERN: Regex = Regex::new(r#"(\d+): (\d+) ?(\d+)? ?(\d+)?"#).unwrap();
    static ref COMPLEX_PATTERN: Regex = Regex::new(r#"(\d+): (\d+) ?(\d+)? ?(\d+)? (\|) (\d+) ?(\d+)? ?(\d+)?"#).unwrap();
}

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let (rules, messages) = parse(input);
    let rule_zero = rules.get(&0).unwrap();

    messages.iter()
        .filter(|msg| rule_zero.matches(**msg, &rules))
        .count() as i64
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let (mut rules, messages) = parse(input);

    rules.insert(8, Rule::from("8: 42 | 42 8"));
    rules.insert(11, Rule::from("11: 42 31 | 42 11 31"));

    let rule_zero = rules.get(&0).unwrap();

    messages.iter()
        .filter(|msg| rule_zero.matches(**msg, &rules))
        .count() as i64
}

fn parse<'a, I, J>(input: I) -> (HashMap<i32, Rule>, Vec<&'a str>)
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut rules = HashMap::new();
    let mut messages = vec![];
    let mut rules_done = false;

    for line in input {
        let str = line.as_ref();

        if str.is_empty() {
            rules_done = true;
        } else if rules_done {
            messages.push(str);
        } else {
            let rule = Rule::from(str);
            rules.insert(rule.id, rule);
        }
    }

    (rules, messages)
}

#[derive(Eq, PartialEq, Debug)]
enum MatchCriteria {
    Constant(char),
    Simple(Vec<i32>),
    Complex(Vec<i32>, Vec<i32>),
}

impl MatchCriteria {
    pub fn get_dependent_ids(&self) -> Vec<i32> {
        let mut ids = vec![];

        match self {
            Constant(_) => {}
            Simple(s) => {
                for i in s {
                    ids.push(*i);
                }
            }
            Complex(l, r) => {
                for i in l {
                    ids.push(*i);
                }
                for i in r {
                    ids.push(*i);
                }
            }
        }

        ids
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Rule {
    id: i32,
    criteria: MatchCriteria,
}

impl From<&str> for Rule {
    fn from(input: &str) -> Self {
        let id;
        let criteria;

        if let Some(captures) = CONST_PATTERN.captures(input) {
            id = captures.get(1).unwrap().as_str().parse().unwrap();
            criteria = Constant(captures.get(2).unwrap().as_str().chars().next().unwrap());
        } else if let Some(captures) = COMPLEX_PATTERN.captures(input) {
            id = captures.get(1).unwrap().as_str().parse().unwrap();

            let mut left_matches = vec![];
            let mut right_matches = vec![];

            let mut index = 2;
            let mut left = true;

            while index < captures.len() {
                if let Some(capture) = captures.get(index) {
                    let s = capture.as_str();

                    if s == "|" {
                        left = false;
                    } else {
                        let val = s.parse().unwrap();

                        if left {
                            left_matches.push(val);
                        } else {
                            right_matches.push(val);
                        }
                    }
                }

                index += 1;
            }

            criteria = Complex(left_matches, right_matches);
        } else if let Some(captures) = SIMPLE_PATTERN.captures(input) {
            id = captures.get(1).unwrap().as_str().parse().unwrap();

            let mut matches = vec![];

            for i in 2..captures.len() {
                if let Some(capture) = captures.get(i) {
                    matches.push(capture.as_str().parse().unwrap());
                }
            }

            criteria = Simple(matches);
        } else {
            panic!("Input '{}' did not match any regex", input);
        }

        Rule {
            id,
            criteria,
        }
    }
}

impl Rule {
    pub fn matches(&self, input: &str, rules: &HashMap<i32, Rule>) -> bool {
        let mut index = 0;
        let result = self.test(input, rules, index);

        info!("{}, {} --- {:?}", input, input.len(), result);

        result.0 && result.1 == input.len()
    }

    fn test(&self, input: &str, rules: &HashMap<i32, Rule>, index: usize) -> (bool, usize) {
        let mut index = index;

        match &self.criteria {
            Constant(c) => {
                if let Some(compare) = input.chars().nth(index) {
                    (*c == compare, index + 1)
                } else {
                    (false, index)
                }
            }
            Simple(ids) => {
                Rule::test_vec(input, rules, index, ids)
            }
            Complex(ids, alt_ids) => {
                let mut init_index = index;
                let primary_check = Rule::test_vec(input, rules, index, ids);

                if primary_check.0 {
                    primary_check
                } else {
                    index = init_index;
                    Rule::test_vec(input, rules, index, alt_ids)
                }
            }
        }
    }

    fn test_vec(input: &str, rules: &HashMap<i32, Rule>, index: usize, ids: &Vec<i32>) -> (bool, usize) {
        let mut index = index;

        for id in ids {
            let rule = rules.get(id).unwrap();
            let result = rule.test(input, rules, index);

            if !result.0 {
                return (false, index);
            } else {
                index = result.1;
            }
        }

        (true, index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn rule_from_str_constant() {
        let rule = Rule::from("0: \"a\"");

        assert_eq!(0, rule.id);
        assert_eq!(Constant('a'), rule.criteria);
    }

    #[test]
    pub fn rule_from_str_simple() {
        assert_eq!(Simple(vec![1]), Rule::from("0: 1").criteria);
        assert_eq!(Simple(vec![1, 2]), Rule::from("0: 1 2").criteria);
        assert_eq!(Simple(vec![1, 2, 3]), Rule::from("0: 1 2 3").criteria);
    }

    #[test]
    pub fn rule_from_str_complex() {
        assert_eq!(Complex(vec![1], vec![4]), Rule::from("0: 1 | 4").criteria);
        assert_eq!(Complex(vec![1, 2], vec![4, 5]), Rule::from("0: 1 2 | 4 5").criteria);
        assert_eq!(Complex(vec![1, 2, 3], vec![4, 5, 6]), Rule::from("0: 1 2 3 | 4 5 6").criteria);
    }

    #[test]
    pub fn rule_match_const() {
        assert!(Rule::from("0: \"a\"").matches("a", &HashMap::new()));
        assert!(!Rule::from("0: \"a\"").matches("b", &HashMap::new()));
    }

    #[test]
    pub fn rule_match_simple() {
        let mut rules = HashMap::new();
        let rule_zero = Rule {
            id: 0,
            criteria: Simple(vec![1]),
        };
        rules.insert(0, rule_zero);
        rules.insert(1, Rule {
            id: 1,
            criteria: Constant('a'),
        });

        let rule_zero = rules.get(&0).unwrap();

        assert!(rule_zero.matches("a", &rules));
        assert!(!rule_zero.matches("b", &rules));
    }

    #[test]
    pub fn rule_match_complex() {
        let mut rules = HashMap::new();
        let rule_zero = Rule {
            id: 0,
            criteria: Complex(vec![1], vec![2]),
        };
        rules.insert(0, rule_zero);
        rules.insert(1, Rule {
            id: 1,
            criteria: Constant('a'),
        });
        rules.insert(2, Rule {
            id: 2,
            criteria: Constant('b'),
        });

        let rule_zero = rules.get(&0).unwrap();

        assert!(rule_zero.matches("a", &rules));
        assert!(rule_zero.matches("b", &rules));
        assert!(!rule_zero.matches("c", &rules));
    }

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ]
    }

    pub fn get_sample_1() -> Vec<&'static str> {
        vec![
            "0: 1 2",
            "1: \"a\"",
            "2: 1 3 | 3 1",
            "3: \"b\"",
            "",
            "aab",
            "aba",
            "ab",
            "aabb"
        ]
    }

    pub fn get_sample_2() -> Vec<&'static str> {
        vec![
"42: 9 14 | 10 1",
"9: 14 27 | 1 26",
"10: 23 14 | 28 1",
"1: \"a\"",
"11: 42 31",
"5: 1 14 | 15 1",
"19: 14 1 | 14 14",
"12: 24 14 | 19 1",
"16: 15 1 | 14 14",
"31: 14 17 | 1 13",
"6: 14 14 | 1 14",
"2: 1 24 | 14 4",
"0: 8 11",
"13: 14 3 | 1 12",
"15: 1 | 14",
"17: 14 2 | 1 7",
"23: 25 1 | 22 14",
"28: 16 1",
"4: 1 1",
"20: 14 14 | 1 15",
"3: 5 14 | 16 1",
"27: 1 6 | 14 18",
"14: \"b\"",
"21: 14 1 | 1 14",
"25: 1 1 | 1 14",
"22: 14 14",
"8: 42",
"26: 14 22 | 1 20",
"18: 15 15",
"7: 14 5 | 1 21",
"24: 14 1",
"",
"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
"bbabbbbaabaabba",
"babbbbaabbbbbabbbbbbaabaaabaaa",
"aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
"bbbbbbbaaaabbbbaaabbabaaa",
"bbbababbbbaaaaaaaabbababaaababaabab",
"ababaaaaaabaaab",
"ababaaaaabbbaba",
"baabbaaaabbaaaababbaababb",
"abbbbabbbbaaaababbbbbbaaaababb",
"aaaaabbaabaaaaababaa",
"aaaabbaaaabbaaa",
"aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
"babaaabbbaaabaababbaabababaaab",
"aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(2, run_a(&sample));
    }

    #[test]
    pub fn sample_input_1_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample_1();

        assert_eq!(2, run_a(&sample));
    }

    #[test]
    pub fn sample_input_2_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample_2();

        assert_eq!(3, run_a(&sample));
    }

    #[test]
    #[ignore]
    pub fn sample_input_2_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample_2();

        assert_eq!(12, run_b(&sample));
    }
}