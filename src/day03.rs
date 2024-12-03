use std::str::Chars;
use regex::Regex;

pub fn run_a(input: &str) -> i64 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let matches = regex.captures_iter(input);

    let mut result = 0;

    for m in matches {
        let left_str = m.get(1).map(|m|m.as_str()).unwrap_or("");
        let right_str = m.get(2).map(|m|m.as_str()).unwrap_or("");

        let left = left_str.parse::<i64>().unwrap();
        let right = right_str.parse::<i64>().unwrap();

        result += left * right;
    }

    result
}

pub fn run_b(input: &str) -> i64 {
    let mut enabled = true;
    let mut output = 0;
    let mut iter = input.chars();

    while let Some(start) = iter.next() {
        match start {
            'd' => {
                let result = parse_conditional(&mut iter);

                match result {
                    ConditionalParseResult::Do => enabled = true,
                    ConditionalParseResult::Dont => enabled = false,
                    ConditionalParseResult::Invalid => {}
                }
            },
            'm' if enabled => {
                let result = parse_mul(&mut iter);
                match result {
                    MulParseResult::Mul(left, right) => {
                        output += left * right;
                        log::debug!("{} * {}, {}", left, right, output);
                    },
                    MulParseResult::Invalid => {}
                }
            },
            _ => {},
        }
    }

    output
}

#[derive(Debug, Eq, PartialEq)]
enum ConditionalParseResult {
    Do,
    Dont,
    Invalid
}

fn parse_conditional(iter: &mut Chars) -> ConditionalParseResult {
    let next = iter.next();

    if next != Some('o') {
        return ConditionalParseResult::Invalid;
    }

    let next = iter.next();

    match next {
        Some('(') => {
            let next = iter.next();

            match next {
                Some(')') => ConditionalParseResult::Do,
                _ => ConditionalParseResult::Invalid,
            }
        },
        Some('n') => {
            let apostrophe = iter.next();

            if apostrophe != Some('\'') {
                return ConditionalParseResult::Invalid;
            }

            let t = iter.next();

            match t {
                Some('t') => ConditionalParseResult::Dont,
                _ => ConditionalParseResult::Invalid,
            }
        },
        _ => {
            ConditionalParseResult::Invalid
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MulParseResult {
    Mul(i64, i64),
    Invalid
}

fn parse_mul(mut iter: &mut Chars) -> MulParseResult {
    let u = iter.next();

    if u != Some('u') {
        return MulParseResult::Invalid;
    }

    let l = iter.next();

    if l != Some('l') {
        return MulParseResult::Invalid;
    }

    let parend = iter.next();

    if parend != Some('(') {
        return MulParseResult::Invalid;
    }

    match parse_number(&mut iter, ',') {
        NumParseResult::Num(l) => {
            match parse_number(&mut iter, ')') {
                NumParseResult::Num(r) => MulParseResult::Mul(l, r),
                NumParseResult::Invalid => MulParseResult::Invalid,
            }
        }
        NumParseResult::Invalid => MulParseResult::Invalid
    }

}

#[derive(Debug, Eq, PartialEq)]
enum NumParseResult {
    Num(i64),
    Invalid,
}

fn parse_number(iter: &mut Chars, terminator: char) -> NumParseResult {
    let mut buffer = String::new();

    match iter.next() {
        Some(c) if c.is_ascii_digit() => {
            buffer.push(c);
        },
        _ => return NumParseResult::Invalid
    }

    for _ in 0..2 {
        match iter.next() {
            Some(c) if c.is_ascii_digit() => {
                buffer.push(c);
            },
            Some(c) if c == terminator => {
                return NumParseResult::Num(buffer.parse::<i64>().unwrap());
            },
            _ => return NumParseResult::Invalid
        }
    }

    match iter.next() {
        Some(c) if c == terminator => {
            NumParseResult::Num(buffer.parse::<i64>().unwrap())
        },
        _ => NumParseResult::Invalid
    }
}



#[cfg(test)]
mod test {

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn input() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    #[test]
    fn part_a_example() {
        init();

        let result = super::run_a(input());

        assert_eq!(result, 161);
    }

    #[test]
    fn part_b_example() {
        let result = super::run_b("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }

    #[test]
    fn part_b_end_on_parend() {
        let result = super::run_b("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)");
        assert_eq!(result, 48);
    }

    #[test]
    fn edges() {
        assert_eq!(super::run_b("-:-]what()(+/mul(957,396)?mul(550,844)%+why())-? #}from()mul(488,628)%}"), 1149636);
    }

    #[test]
    fn parse_conditional() {
        assert_eq!(super::ConditionalParseResult::Do, super::parse_conditional(&mut "o()".chars()));
        assert_eq!(super::ConditionalParseResult::Dont, super::parse_conditional(&mut "on't()".chars()));
        assert_eq!(super::ConditionalParseResult::Invalid, super::parse_conditional(&mut "a()".chars()));
        assert_eq!(super::ConditionalParseResult::Invalid, super::parse_conditional(&mut "ont()".chars()));
    }

    #[test]
    fn parse_mul() {

    }

    #[test]
    fn parse_number() {
        assert_eq!(super::NumParseResult::Num(1), super::parse_number(&mut "1,".chars(), ','));
        assert_eq!(super::NumParseResult::Num(1), super::parse_number(&mut "1)".chars(), ')'));
        assert_eq!(super::NumParseResult::Invalid, super::parse_number(&mut "1,".chars(), ')'));
        assert_eq!(super::NumParseResult::Num(12), super::parse_number(&mut "12,".chars(), ','));
        assert_eq!(super::NumParseResult::Num(123), super::parse_number(&mut "123,".chars(), ','));
        assert_eq!(super::NumParseResult::Invalid, super::parse_number(&mut "1234,".chars(), ','));
    }
}