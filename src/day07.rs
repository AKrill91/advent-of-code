use itertools::Itertools;

struct Equation {
    value: i64,
    parts: Vec<i64>,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat
}

const OPERATIONS_WITHOUT_CONCAT: [Operation; 2] = [Operation::Add, Operation::Multiply];
const OPERATIONS_WITH_CONCAT: [Operation; 3] = [Operation::Multiply, Operation::Add, Operation::Concat];

impl Operation {
    pub fn all_combinations(length: usize, include_concatenation: bool) -> Vec<Vec<Operation>> {
        if length == 1 {
            if include_concatenation {
                return vec![vec![Operation::Add], vec![Operation::Multiply], vec![Operation::Concat]];
            } else {
                return vec![vec![Operation::Add], vec![Operation::Multiply]];
            }
        }

        let operations_to_use: &[Operation] = if include_concatenation { &OPERATIONS_WITH_CONCAT } else { &OPERATIONS_WITHOUT_CONCAT };

        let start: Vec<Vec<Operation>> = operations_to_use
            .iter()
            .cartesian_product(operations_to_use)
            .map(|(a, b)| vec![*a, *b])
            .collect();

        (2..length).fold(
            start,
            |acc, _| {
                acc.into_iter()
                    .cartesian_product(operations_to_use)
                    .map(|(mut a, b)| {
                        a.push(*b);
                        a
                })
                    .collect()
            }
        )
    }
}

impl Equation {
    pub fn new(value: i64, parts: Vec<i64>) -> Equation {
        Equation {
            value,
            parts
        }
    }

    fn could_be_true(&self, include_concatenation: bool) -> bool {
        let num_operators = self.parts.len() - 1;

        let operator_combinations = Operation::all_combinations(num_operators, include_concatenation);

        for operators in operator_combinations {
            let mut part_iter = self.parts.iter();
            let mut operator_iter = operators.iter();

            let mut sum = *part_iter.next().unwrap();

            while let Some(&n) = part_iter.next() {
                let operator = operator_iter.next().unwrap();

                match operator {
                    Operation::Add => {
                        sum += n;
                    }
                    Operation::Multiply => {
                        sum *= n;
                    }
                    Operation::Concat => {
                        let mut s = sum.to_string();
                        s.push_str(&n.to_string());
                        sum = s.parse().unwrap();
                    }
                }
            }

            if sum == self.value {
                return true;
            }
        }

        false
    }
}

fn parse(input: &str) -> Vec<Equation> {
    input.trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let value = parts.next().unwrap().parse::<i64>().unwrap();
            let parts = parts.next().unwrap().trim().split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            Equation { value, parts }

        })
        .collect()
}

pub fn run_a(input: &str) -> i64 {
    parse(input)
        .iter().filter(|eq| eq.could_be_true(false))
        .map(|eq| eq.value)
        .sum()
}

pub fn run_b(input: &str) -> i64 {
    parse(input)
        .iter().filter(|eq| eq.could_be_true(true))
        .map(|eq| eq.value)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day07::Equation;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
    }


    #[test]
    fn part_a_example() {
        init();
        assert_eq!(3749, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(11387, super::run_b(example()));
    }

    #[test]
    fn equation_could_be_true() {
        init();
        assert!(Equation::new(190, vec![10, 19]).could_be_true(false));
        assert!(Equation::new(3267, vec![81, 40, 27]).could_be_true(false));
        assert!(Equation::new(292, vec![11, 6, 16, 20]).could_be_true(false));

        assert!(!Equation::new(83, vec![17, 5]).could_be_true(false));
        assert!(!Equation::new(156, vec![15, 6]).could_be_true(false));
        assert!(!Equation::new(7290, vec![6, 8, 6, 15]).could_be_true(false));
    }

    #[test]
    fn equation_could_be_true_with_concat() {
        init();
        assert!(Equation::new(190, vec![10, 19]).could_be_true(true));
        assert!(Equation::new(3267, vec![81, 40, 27]).could_be_true(true));
        assert!(Equation::new(292, vec![11, 6, 16, 20]).could_be_true(true));
        assert!(Equation::new(156, vec![15, 6]).could_be_true(true));
        assert!(Equation::new(7290, vec![6, 8, 6, 15]).could_be_true(true));

        assert!(!Equation::new(83, vec![17, 5]).could_be_true(true));

    }
}