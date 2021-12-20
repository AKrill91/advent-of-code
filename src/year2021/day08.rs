use std::collections::{HashMap, HashSet};

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
    let sum: u32 = input.iter()
        .map(|line| parse_line(line))
        .map(|(signals, outputs)| solve(signals, outputs))
        .sum();

    format!("{}", sum)
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = line.split('|').collect();

    (
        parts[0].split(' ').filter(|s| !s.is_empty()).collect(),
        parts[1].split(' ').filter(|s| !s.is_empty()).collect(),
    )
}

fn solve(signals: Vec<&str>, outputs: Vec<&str>) -> u32 {
    let mut possibilities = DisplaySegment::possibilities();

    signals.iter()
        .chain(outputs.iter())
        .filter(|s| UNIQUE_SEGMENT_COUNTS.contains(&s.len()))
        .for_each(|s| {

        });

    0
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum DisplaySegment {
    A,
    B,
    C,
    D,
    E,
    F,
    G
}

impl DisplaySegment {
    fn all() -> [Self;7] {
        [
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
        ]
    }

    fn possibilities() -> HashMap<Self, HashSet<char>> {
        Self::all()
            .iter()
            .map(|s| (*s, vec!['a','b','c','d','e','f','g'].into_iter().collect::<HashSet<char>>()))
            .collect()
    }
}

enum SevenSegmentDisplay {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE
}

impl SevenSegmentDisplay {
    fn segments_with_count(count: usize) -> Vec<SevenSegmentDisplay> {
        match count {
            2 => vec![Self::ONE],
            3 => vec![Self::SEVEN],
            4 => vec![Self::FOUR],
            5 => vec![Self::TWO, Self::THREE,Self::FIVE],
            6 => vec![Self::ZERO, Self::SIX, Self::NINE],
            7 => vec![Self::EIGHT],
            _ => panic!()
        }
    }

    fn segment_count(&self) -> usize {
        match self {
            SevenSegmentDisplay::ZERO => 6,
            SevenSegmentDisplay::ONE => 2,
            SevenSegmentDisplay::TWO => 5,
            SevenSegmentDisplay::THREE => 5,
            SevenSegmentDisplay::FOUR => 4,
            SevenSegmentDisplay::FIVE => 5,
            SevenSegmentDisplay::SIX => 6,
            SevenSegmentDisplay::SEVEN => 3,
            SevenSegmentDisplay::EIGHT => 7,
            SevenSegmentDisplay::NINE => 6
        }
    }

    fn segments(&self) -> HashSet<DisplaySegment> {
        match self {
            SevenSegmentDisplay::ZERO => vec![DisplaySegment::A,DisplaySegment::B,DisplaySegment::C,DisplaySegment::E,DisplaySegment::F,DisplaySegment::G],
            SevenSegmentDisplay::ONE => vec![DisplaySegment::C, DisplaySegment::F],
            SevenSegmentDisplay::TWO => vec![DisplaySegment::A,DisplaySegment::C,DisplaySegment::D,DisplaySegment::E,DisplaySegment::G],
            SevenSegmentDisplay::THREE => vec![DisplaySegment::A,DisplaySegment::C,DisplaySegment::D,DisplaySegment::F,DisplaySegment::G],
            SevenSegmentDisplay::FOUR => vec![DisplaySegment::B,DisplaySegment::C,DisplaySegment::D,DisplaySegment::F],
            SevenSegmentDisplay::FIVE => vec![DisplaySegment::A,DisplaySegment::B,DisplaySegment::D,DisplaySegment::F,DisplaySegment::G],
            SevenSegmentDisplay::SIX => vec![DisplaySegment::A,DisplaySegment::B,DisplaySegment::D,DisplaySegment::E,DisplaySegment::F,DisplaySegment::G],
            SevenSegmentDisplay::SEVEN => vec![DisplaySegment::A,DisplaySegment::C,DisplaySegment::F],
            SevenSegmentDisplay::EIGHT => vec![DisplaySegment::A,DisplaySegment::B,DisplaySegment::C,DisplaySegment::D,DisplaySegment::E,DisplaySegment::F,DisplaySegment::G],
            SevenSegmentDisplay::NINE => vec![DisplaySegment::A,DisplaySegment::B,DisplaySegment::C,DisplaySegment::D,DisplaySegment::F,DisplaySegment::G]
        }
            .into_iter()
            .collect()
    }
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

        let expected = "61229";

        assert_eq!(expected, run_b(0, &input));
    }
}