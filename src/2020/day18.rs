use std::fmt::{Debug, Formatter, Write};
use std::str::Chars;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    input.into_iter()
        .map(|line| tokenize(line.as_ref()))
        .map(|e| e.evaluate(false))
        .sum()
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    input.into_iter()
        .map(|line| tokenize(line.as_ref()))
        .map(|e| e.evaluate(true))
        .sum()
}

fn tokenize(input: &str) -> Expression {
    let mut iter = input.chars().into_iter();
    parse(&mut iter)
}

fn parse(iter: &mut Chars) -> Expression {
    let mut expressions = vec![];

    while let Some(c) = iter.next() {
        match c {
            '(' => expressions.push(parse(iter)),
            ')' => return Expression::List(expressions),
            '*' => expressions.push(Expression::Operation(Operation::Multiplication)),
            '+' => expressions.push(Expression::Operation(Operation::Addition)),
            ' ' => continue,
            _ => expressions.push(Expression::Value(c.to_digit(10).unwrap() as i64))
        }
    }

    Expression::List(expressions)
}

enum Expression {
    List(Vec<Expression>),
    Operation(Operation),
    Value(i64)
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::List(expressions) => {
                f.write_char('(')?;
                for e in expressions {
                    e.fmt(f)?;
                }
                f.write_char(')')
            }
            Expression::Operation(o) => o.fmt(f),
            Expression::Value(n) => n.fmt(f)
        }
    }
}

impl Expression {
    pub fn evaluate(&self, advanced: bool) -> i64 {
        match self {
            Expression::List(expressions) => {
                if advanced {
                    Expression::eval_list_advanced(expressions)
                } else {
                    Expression::eval_list(expressions)
                }
            },
            Expression::Operation(o) => panic!("Evaluate on peration {:?}", o),
            Expression::Value(n) => *n
        }
    }

    fn eval_list(expressions: &Vec<Expression>) -> i64 {
        let mut iter = expressions.iter().peekable();
        let mut lhs = iter.next().unwrap().evaluate(false);
        while iter.peek().is_some() {
            let operation = iter.next().unwrap();
            let rhs = iter.next().unwrap().evaluate(false);

            lhs = match operation {
                Expression::Operation(Operation::Addition) => lhs + rhs,
                Expression::Operation(Operation::Multiplication) => lhs * rhs,
                _ => panic!("expected operator, found {:?}", operation)
            }
        }
        lhs
    }

    fn eval_list_advanced(expressions: &Vec<Expression>) -> i64 {
        let mut second_phase = vec![];

        let mut iter = expressions.iter().peekable();
        let mut lhs = iter.next().unwrap().evaluate(true);
        while iter.peek().is_some() {
            let operation = iter.next().unwrap();
            let rhs = iter.next().unwrap().evaluate(true);

            lhs = match operation {
                Expression::Operation(Operation::Addition) => lhs + rhs,
                Expression::Operation(Operation::Multiplication) => {
                    second_phase.push(Expression::Value(lhs));
                    second_phase.push(Expression::Operation(Operation::Multiplication));
                    rhs
                }
                _ => panic!("expected operator, found {:?}", operation)
            }
        }
        second_phase.push(Expression::Value(lhs));

        let mut iter = second_phase.iter().peekable();
        let mut lhs = iter.next().unwrap().evaluate(true);
        while iter.peek().is_some() {
            let operation = iter.next().unwrap();
            let rhs = iter.next().unwrap().evaluate(true);

            lhs = match operation {
                Expression::Operation(Operation::Multiplication) => lhs * rhs,
                _ => panic!("expected operator, found {:?}", operation)
            }
        }

        lhs
    }
}

enum Operation {
    Addition,
    Multiplication
}

impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Addition => f.write_char('+'),
            Operation::Multiplication => f.write_char('*')
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "1 + 2 * 3 + 4 * 5 + 6",
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        ]
    }

    #[test]
    pub fn simple_addition() {
        assert_eq!(3, run_a(vec!["1 + 2"]));
        assert_eq!(7, run_a(vec!["4 + 3"]));
        assert_eq!(6, run_a(vec!["1 + 2 + 3"]));
    }

    #[test]
    pub fn simple_multiplication() {
        assert_eq!(2, run_a(vec!["1 * 2"]));
        assert_eq!(12, run_a(vec!["4 * 3"]));
        assert_eq!(24, run_a(vec!["1 * 2 * 3 * 4"]));
    }

    #[test]
    pub fn simple_parends() {
        assert_eq!(3, run_a(vec!["(1 + 2)"]));
        assert_eq!(6, run_a(vec!["(1 + 2 + 3)"]));
        assert_eq!(12, run_a(vec!["(4 * 3)"]));
        assert_eq!(10, run_a(vec!["1 + (2 + 3) + 4"]));
    }

    #[test]
    pub fn nested_parends() {
        assert_eq!(100, run_a(vec!["5 * (4 * (3 + 2))"]));
        assert_eq!(62, run_a(vec!["(((5 * 4) * 3) + 2)"]));
    }


    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(26_457, run_a(&sample));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(694_173, run_b(&sample));
    }
}