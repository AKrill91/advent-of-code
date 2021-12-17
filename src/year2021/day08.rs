use std::collections::HashMap;

const UNIQUE_SEGMENT_COUNTS: [usize;4] = [2,4,3,7];

pub fn run_a(_: i32, input: &[String]) -> String {
    let unique_digit_count: usize = input.iter()
        .map(|line| parse_line(line))
        .map(|(signals, outputs)| {
            outputs.iter()
                .filter(|o| {
                    UNIQUE_SEGMENT_COUNTS.contains(&o.len())
                })
                .count()
        })
        .sum();

    format!("{}", unique_digit_count)
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("")
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = line.split('|').collect();

    (
        parts[0].split(' ').filter(|s| !s.is_empty()).collect(),
        parts[1].split(' ').filter(|s| !s.is_empty()).collect(),
    )
}

fn get_segment_counts() -> HashMap<u8, u8> {
    vec![
        (0, 6),
        (1, 2),
        (2, 5),
        (3, 5),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 3),
        (8, 7),
        (9, 6),
    ]
        .into_iter()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse_line() {
        let line = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let (signals, outputs) = parse_line(line);

        assert_eq!(10, signals.len());
        assert_eq!(4, outputs.len());

        assert_eq!("ab", signals[9]);
        assert_eq!("fcadb", outputs[1]);
    }

    fn get_sample() -> Vec<String> {
        vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "26";

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