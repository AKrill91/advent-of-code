use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq)]
struct Problem {
    operation: Operation,
    values: Vec<i64>,
}

impl Problem {
    fn new<I: IntoIterator<Item = i64>>(operation: Operation, values: I) -> Problem {
        let iter = values.into_iter();

        Problem {
            operation,
            values: iter.collect(),
        }
    }

    fn result(&self) -> i64 {
        match self.operation {
            Operation::Addition => self.values.iter().sum(),
            Operation::Multiplication => self.values.iter().product(),
        }
    }
}

impl TryFrom<(&[&str], bool)> for Problem {
    type Error = ();

    fn try_from((lines, columnar): (&[&str], bool)) -> Result<Self, Self::Error> {
        let operation_line = &lines[lines.len() - 1];
        let value_lines = &lines[..lines.len() - 1];
        let operation = Operation::try_from(operation_line.chars().nth(0).ok_or(())?)?;

        log::debug!("Operation: {:?}", operation);

        log::debug!("Values {:?}", value_lines);

        let value_strings: Vec<String> = if columnar {
            transpose(value_lines)
        } else {
            value_lines.iter().map(|&s|s.to_owned()).collect()
        };

        log::debug!("Values: {:?}", value_strings);

        let values = value_strings.into_iter().map(|s| s.trim().parse::<i64>().unwrap()).collect();

        log::debug!("Values: {:?}", values);

        Ok(Problem {
            operation,
            values
        })
    }
}

fn transpose(lines: &[&str]) -> Vec<String> {
    // We can't just use the length of the first line because of trailing spaces on right-aligned
    // last problems being removed on save
    let len = lines.iter().map(|s| s.len()).max().unwrap();
    let mut out = vec![];

    for x in 0..len {
        let mut column = vec![];
        for y in 0..lines.len() {
            let line = lines[y];
            let c: u8 = if x >= line.len() {
                b' '
            } else {
                line.as_bytes()[x]
            };

            column.push(c);
        }

        out.push(String::from_utf8(column).unwrap());
    }

    out
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}

impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::Addition),
            '*' => Ok(Operation::Multiplication),
            _ => Err(()),
        }
    }
}

fn parse(input: &str, columnar: bool) -> Vec<Problem> {
    let lines = input.lines().collect::<Vec<_>>();

    let operations = lines[lines.len() - 1];

    let mut operation_indexes = vec![];

    //Operations are always left-aligned, so use those to determine groupings
    for (index, c) in operations.chars().enumerate() {
        if let Ok(_) = Operation::try_from(c) {
            operation_indexes.push(index);
        }
    }

    operation_indexes.iter().enumerate().map(|(i, start)| {
        log::debug!("Working on column {}", i);

        let slices: Vec<&str> = lines.iter()
            .map(|line| {
            if i == operation_indexes.len() - 1 {
                &line[*start..]
            } else {
                let end = operation_indexes[i + 1];
                &line[*start..end-1]
            }
        })
            .collect();

        Problem::try_from((slices.as_slice(), columnar)).unwrap()
    })
        .collect()
}
pub fn run_a(input: &str) -> i64 {
    parse(input, false)
        .iter()
        .map(Problem::result)
        .sum()
}

pub fn run_b(input: &str) -> i64 {
    parse(input, true)
        .iter()
        .map(Problem::result)
        .sum()
}

#[cfg(test)]
mod test {
    use super::Operation::{Addition, Multiplication};
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"
    }

    #[test]
    fn parse_row() {
        init();
        let parsed = super::parse(example(), false);

        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0], Problem::new(Multiplication, [123, 45, 6]));
        assert_eq!(parsed[1], Problem::new(Addition, [328, 64, 98]));
        assert_eq!(parsed[2], Problem::new(Multiplication, [51, 387, 215]));
        assert_eq!(parsed[3], Problem::new(Addition, [64, 23, 314]));
    }

    #[test]
    fn parse_column() {
        init();
        let parsed = super::parse(example(), true);

        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0], Problem::new(Multiplication, [1, 24, 356]));
        assert_eq!(parsed[1], Problem::new(Addition, [369, 248, 8]));
        assert_eq!(parsed[2], Problem::new(Multiplication, [32, 581, 175]));
        assert_eq!(parsed[3], Problem::new(Addition, [623, 431, 4]));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(4277556, run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(3263827, run_b(example()));
    }

    #[test]
    fn transpose() {
        let lines = [
            "123",
            " 45",
            "  6"
        ];

        let transposed = super::transpose(&lines);

        assert_eq!(transposed.len(), 3);
        assert_eq!(transposed[0], String::from("1  "));
        assert_eq!(transposed[1], String::from("24 "));
        assert_eq!(transposed[2], String::from("356"));
    }

    mod problem {
        use super::super::Operation::*;
        use super::super::*;

        #[test]
        fn try_from_row() {
            let lines = [
                "123",
                " 45",
                "  6",
                "*  "
            ];

            let problem = Problem::try_from((&lines[..], false)).unwrap();

            assert_eq!(problem.operation, Multiplication);
            assert_eq!(problem.values, vec![123, 45, 6]);
        }

        #[test]
        fn try_from_column() {
            let lines = [
                "123",
                " 45",
                "  6",
                "*  "
            ];

            let problem = Problem::try_from((&lines[..], true)).unwrap();

            assert_eq!(problem.operation, Multiplication);
            assert_eq!(problem.values, vec![1, 24, 356]);
        }

        #[test]
        fn result_addition() {
            super::init();
            let problem = Problem::new(Multiplication, [123, 45, 6]);
            assert_eq!(33210, problem.result());
        }

        #[test]
        fn result_multiplication() {
            super::init();
            let problem = Problem::new(Addition, [328, 64, 98]);
            assert_eq!(490, problem.result());
        }
    }
}