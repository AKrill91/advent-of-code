use std::collections::VecDeque;

use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Technique {
    NewStack,
    IncrementDeal,
    Cut,
}

#[derive(Debug)]
struct Move {
    technique: Technique,
    count: i64,
}

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    pub fn new(deck_size: usize) -> Deck {
        let range = (0..deck_size);

        Deck {
            cards: range.collect(),
        }
    }

    pub fn apply(&mut self, m: Move) {
        match m.technique {
            Technique::NewStack => {
                self.cards.reverse();
            }
            Technique::IncrementDeal => {
                let increment = m.count as usize;
                let length = self.cards.len();
                let mut table = vec![0 as usize; length];
                let mut position = 0;

                for val in self.cards.iter() {
                    let val = *val;
                    table[position] = val;

                    position += increment;

                    if position >= length {
                        position -= length;
                    }
                }

                self.cards = table;
            }
            Technique::Cut => {
                let mut copy = Vec::with_capacity(self.cards.capacity());
                let length = self.cards.len();
                let mut position = (m.count + length as i64) as usize;

                if position >= length {
                    position -= length;
                }

                while copy.len() != length {
                    copy.push(*self.cards.get(position).unwrap());
                    position += 1;

                    if position >= length {
                        position -= length;
                    }
                }

                self.cards = copy;
            }
        }
    }
}

pub fn run_a(input: &Vec<String>) -> usize {
    let mut deck = Deck::new(10_007);
    run(&mut deck, parse_techniques(input));

    deck.cards.iter().position(|i|*i == 2019).unwrap()
}

pub fn run_b(input: &Vec<String>) -> i64 {
    0
}

fn run(deck: &mut Deck, moves: Vec<Move>) {
    for m in moves {
        deck.apply(m);
    }
}

fn parse_techniques(techniques: &Vec<String>) -> Vec<Move> {
    let cut_pattern = Regex::new(r"cut (-?\d{1,8})").unwrap();
    let new_stack_pattern = Regex::new(r"deal into new stack").unwrap();
    let increment_pattern = Regex::new(r"deal with increment (\d{1,8})").unwrap();

    let mut moves = vec![];

    for technique in techniques {
        let m: Move = if new_stack_pattern.is_match(technique) {
            Move {
                technique: Technique::NewStack,
                count: 0,
            }
        } else if cut_pattern.is_match(technique) {
            Move {
                technique: Technique::Cut,
                count: cut_pattern.captures(technique).unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            }
        } else if increment_pattern.is_match(technique) {
            Move {
                technique: Technique::IncrementDeal,
                count: increment_pattern.captures(technique).unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            }
        } else {
            panic!("Unknown technique: {}", technique);
        };

        moves.push(m);
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_new_stack() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut deck = Deck::new(10);

        deck.apply(
            Move {
                technique: Technique::NewStack,
                count: 0,
            }
        );

        let expected: Vec<usize> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_cut_positive() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut deck = Deck::new(10);

        deck.apply(
            Move {
                technique: Technique::Cut,
                count: 3,
            }
        );

        let expected: Vec<usize> = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_cut_negative() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut deck = Deck::new(10);

        deck.apply(
            Move {
                technique: Technique::Cut,
                count: -4,
            }
        );

        let expected: Vec<usize> = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_increment() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut deck = Deck::new(10);

        deck.apply(
            Move {
                technique: Technique::IncrementDeal,
                count: 3,
            }
        );

        let expected: Vec<usize> = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_0() {
        let instructions = vec![
            String::from("deal with increment 7"),
            String::from("deal into new stack"),
            String::from("deal into new stack"),
        ];

        let mut deck = Deck::new(10);
        let moves = parse_techniques(&instructions);
        let expected: Vec<usize> = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];

        run(&mut deck, moves);

        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_1() {
        let instructions = vec![
            String::from("cut 6"),
            String::from("deal with increment 7"),
            String::from("deal into new stack"),
        ];

        let mut deck = Deck::new(10);
        let moves = parse_techniques(&instructions);
        let expected: Vec<usize> = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

        run(&mut deck, moves);

        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_2() {
        let instructions = vec![
            String::from("deal with increment 7"),
            String::from("deal with increment 9"),
            String::from("cut -2"),
        ];

        let mut deck = Deck::new(10);
        let moves = parse_techniques(&instructions);
        let expected: Vec<usize> = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9];

        run(&mut deck, moves);

        assert_eq!(deck.cards, expected);
    }

    #[test]
    pub fn sample_3() {
        let instructions = vec![
            String::from("deal into new stack"),
            String::from("cut -2"),
            String::from("deal with increment 7"),
            String::from("cut 8"),
            String::from("cut -4"),
            String::from("deal with increment 7"),
            String::from("cut 3"),
            String::from("deal with increment 9"),
            String::from("deal with increment 3"),
            String::from("cut -1"),
        ];

        let mut deck = Deck::new(10);
        let moves = parse_techniques(&instructions);
        let expected: Vec<usize> = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];

        run(&mut deck, moves);

        assert_eq!(deck.cards, expected);
    }
}