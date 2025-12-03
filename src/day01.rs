use itertools::Itertools;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Write};
use std::ops::Neg;
use std::str::FromStr;

const DIAL_SIZE: i32 = 100;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Direction::Left => 'L',
            Direction::Right => 'R',
        })
    }
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(value),
        }
    }
}

impl Direction {
    fn multiplier(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    amount: i32,
    direction: Direction,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.direction.fmt(f)?;
        self.amount.fmt(f)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = Direction::try_from(s.chars().nth(0).ok_or_else(|| "Empty instruction")?)
            .map_err(|e| format!("Unable to convert {} into direction", e))?;

        let amount = s[1..]
            .parse::<i32>()
            .map_err(|e| format!("Unable to convert {} into amount", e))?;

        Ok(Instruction { amount, direction })
    }
}

impl Instruction {
    fn new(amount: i32, direction: Direction) -> Self {
        Instruction { amount, direction }
    }

    fn value(&self) -> i32 {
        self.amount * self.direction.multiplier()
    }
}

struct Dial {
    position: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { position: 50 }
    }
}

impl Dial {
    fn new(position: i32) -> Dial {
        Dial { position }
    }

    fn apply(&mut self, instruction: &Instruction) -> i32 {
        log::debug!("Position: {}, Instruction: {}", self.position, instruction);
        let amount = instruction.value();
        let full_rotations = (amount / DIAL_SIZE).abs();
        let mut zero_count = full_rotations;
        let remainder = amount % DIAL_SIZE;
        let new_pos = ((self.position + remainder) + DIAL_SIZE) % DIAL_SIZE;

        if new_pos == 0 {
            // Stopped at 0
            zero_count += 1;
        } else if (self.position == 0) {
            //Don't want to treat coming off of 0 as crossing 0
        } else if (self.position + remainder) < 0 {
            zero_count += 1;
        } else if (self.position + remainder) > DIAL_SIZE {
            zero_count += 1;
        }

        self.position = new_pos;

        log::debug!("Crossed zero {} times", zero_count);

        zero_count
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut out = vec![];
    for line in input.trim().lines() {
        out.push(Instruction::from_str(line).unwrap());
    }

    out
}

pub fn run_a(input: &str) -> i32 {
    let mut dial = Dial::default();
    let mut zero_count = 0;

    let instructions = parse_instructions(input);

    for instruction in instructions {
        dial.apply(&instruction);
        if dial.position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

pub fn run_b(input: &str) -> i32 {
    let mut dial = Dial::default();
    let mut zero_count = 0;

    let instructions = parse_instructions(input);

    for instruction in instructions {
        zero_count += dial.apply(&instruction);
    }

    zero_count
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    mod instruction {
        use super::super::Instruction;
        use crate::day01::Direction;
        use std::str::FromStr;

        #[test]
        fn try_from_left() {
            let instruction = Instruction::from_str("L5").unwrap();

            assert_eq!(
                instruction,
                Instruction {
                    amount: 5,
                    direction: Direction::Left,
                }
            );
        }

        #[test]
        fn try_from_right() {
            let instruction = Instruction::from_str("R10").unwrap();

            assert_eq!(
                instruction,
                Instruction {
                    amount: 10,
                    direction: Direction::Right,
                }
            );
        }

        #[test]
        fn value_left(){
            assert_eq!(-10, Instruction::from_str("L10").unwrap().value());
        }

        #[test]
        fn value_right(){
            assert_eq!(10, Instruction::from_str("R10").unwrap().value());
        }
    }

    mod dial {
        use crate::day01::{Dial, Direction, Instruction};

        #[test]
        fn apply_left() {
            let mut dial = Dial::new(5);

            let zero_count = dial.apply(&Instruction::new(10, Direction::Left));

            assert_eq!(dial.position, 95);
            assert_eq!(zero_count, 1);
        }

        #[test]
        fn overapply_left() {
            let mut dial = Dial::new(5);

            let zero_count = dial.apply(&Instruction::new(110, Direction::Left));

            assert_eq!(dial.position, 95);
            assert_eq!(zero_count, 2);
        }

        #[test]
        fn apply_right() {
            let mut dial = Dial::new(95);

            let zero_count = dial.apply(&Instruction::new(10, Direction::Right));

            assert_eq!(dial.position, 5);
            assert_eq!(zero_count, 1);
        }

        #[test]
        fn overapply_right() {
            let mut dial = Dial::new(95);

            let zero_count = dial.apply(&Instruction::new(110, Direction::Right));

            assert_eq!(dial.position, 5);
            assert_eq!(zero_count, 2);
        }

        #[test]
        fn apply_left_no_zero() {
            let mut dial = Dial::new(15);

            let zero_count = dial.apply(&Instruction::new(10, Direction::Left));

            assert_eq!(dial.position, 5);
            assert_eq!(zero_count, 0);
        }

        #[test]
        fn apply_right_no_zero() {
            let mut dial = Dial::new(85);

            let zero_count = dial.apply(&Instruction::new(10, Direction::Right));

            assert_eq!(dial.position, 95);
            assert_eq!(zero_count, 0);
        }
    }

    #[test]
    fn a_sample() {
        init();
        let input = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        ";

        assert_eq!(run_a(input), 3);
    }

    #[test]
    fn b_sample() {
        init();
        let input = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        ";

        assert_eq!(run_b(input), 6);
    }
}
