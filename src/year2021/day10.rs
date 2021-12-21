pub fn run_a(_: i32, input: &[String]) -> String {
    let score: i32 = input.iter()
        .map(|line| read_chunks(line))
        .map(score)
        .sum();

    format!("{}", score)
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("")
}

fn score(c: Option<char>) -> i32 {
    match c {
        None => 0,
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        Some(c) => panic!("Unexpected character '{}'", c)
    }
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

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_score() {
        assert_eq!(0, score(None));
        assert_eq!(3, score(Some(')')));
        assert_eq!(57, score(Some(']')));
        assert_eq!(1197, score(Some('}')));
        assert_eq!(25137, score(Some('>')));
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

        let expected = "";

        assert_eq!(expected, run_b(0, &input));
    }
}