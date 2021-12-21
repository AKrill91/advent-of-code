use std::str::Chars;

pub fn run_a(_: i32, input: &[String]) -> String {
    let score: i32 = input.iter()
        .map(|line| read_chunks(line))
        .map(error_score)
        .sum();

    format!("{}", score)
}

pub fn run_b(_: i32, input: &[String]) -> String {
    let scores: Vec<i64> = input.iter()
        .filter(|line| read_chunks(line).is_none())
        .map(|line| complete_chunks(line))
        .collect();

    info!("Found {} scores: {:?}", scores.len(), scores);

    let winner = winning_score(scores);

    format!("{}", winner)
}

fn error_score(c: Option<char>) -> i32 {
    match c {
        None => 0,
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        Some(c) => panic!("Unexpected character '{}'", c)
    }
}

fn completion_score(chars: Chars) -> i64 {
    let mut score = 0;

    for c in chars {
        score *= 5;

        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unexpected character '{}'", c)
        }
    }

    score
}

fn winning_score(mut scores: Vec<i64>) -> i64 {
    scores.sort();
    scores[scores.len() / 2]
}

fn read_chunks(chunks: &str) -> Option<char> {
    let mut stack = vec![];

    for c in chunks.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if '(' != stack.pop().unwrap() {
                    return Some(c);
                }
            },
            ']' => {
                if '[' != stack.pop().unwrap() {
                    return Some(c);
                }
            },
            '}' => {
                if '{' != stack.pop().unwrap() {
                    return Some(c);
                }
            },
            '>' => {
                if '<' != stack.pop().unwrap() {
                    return Some(c);
                }
            }
            _ => panic!("Unexpected character {}", c)
        }
    }
    None
}

fn complete_chunks(chunks: &str) -> i64 {
    let mut stack = vec![];

    for c in chunks.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            },
            ')' | ']' | '}' | '>' => {
                stack.pop();
            },
            _ => panic!("Unexpected character {}", c)
        }
    }

    let mut missing = "".to_string();

    while let Some(c) = stack.pop() {
        missing.push_str(get_end(c));
    }

    completion_score(missing.chars())
}

fn get_end(c: char) -> &'static str {
    match c {
        '(' => ")",
        '[' => "]",
        '{' => "}",
        '<' => ">",
        _ => panic!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_error_score() {
        assert_eq!(0, error_score(None));
        assert_eq!(3, error_score(Some(')')));
        assert_eq!(57, error_score(Some(']')));
        assert_eq!(1197, error_score(Some('}')));
        assert_eq!(25137, error_score(Some('>')));
    }

    #[test]
    fn test_completion_score() {
        assert_eq!(288957, completion_score("}}]])})]".chars()));
        assert_eq!(5566, completion_score(")}>]})".chars()));
        assert_eq!(1480781, completion_score("}}>}>))))".chars()));
        assert_eq!(995444, completion_score("]]}}]}]}>".chars()));
        assert_eq!(294, completion_score("])}>".chars()));
    }

    #[test]
    fn test_winning_score() {
        assert_eq!(288957, winning_score(vec![288957, 5566, 1480781, 995444, 294]))
    }

    #[test]
    fn read_chunks_simple() {
        assert_eq!(None, read_chunks("()"));
        assert_eq!(Some(']'), read_chunks("(]"));
    }

    fn get_sample() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "26397";

        assert_eq!(expected, run_a(0, &input));
    }

    #[test]
    fn test_sample_b() {
        init();

        let input = get_sample();

        let expected = "288957";

        assert_eq!(expected, run_b(0, &input));
    }
}