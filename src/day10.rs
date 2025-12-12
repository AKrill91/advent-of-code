use std::convert::TryFrom;

#[derive(Debug)]
struct Machine {
    desired_indicators: Vec<bool>,
    current_indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<i64>,
}

//region Machine Parsing

impl TryFrom<&str> for Machine {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(' ')
            .collect::<Vec<&str>>();

        let (desired_indicators, current_indicators) = try_parse_indicators(parts[0])?;

        let buttons = try_parse_buttons(&parts[1..parts.len()-1])?;

        let joltage_requirements = try_parse_joltage(parts[parts.len() - 1])?;

        Ok(Machine {
            desired_indicators,
            current_indicators,
            buttons,
            joltage_requirements,
        })
    }
}

fn try_parse_indicators(input: &str) -> Result<(Vec<bool>, Vec<bool>), String> {
    let mut out = Vec::new();

    for c in input.chars() {
        match c {
            '[' | ']' => continue,
            '#' => out.push(true),
            '.' => out.push(false),
            _ => return Err(format!("Invalid indicator '{}'", c)),
        }
    }

    let len = out.len();

    Ok((out, vec![false; len]))
}

fn try_parse_buttons(input: &[&str]) -> Result<Vec<Vec<usize>>, String> {
    let mut out = Vec::new();

    for part in input {
        let part = part.trim_matches(|c| c == '(' || c == ')');

        let indices = part.split(',')
            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<usize>, String>>()?;

        out.push(indices);
    }

    Ok(out)
}

fn try_parse_joltage(input: &str) -> Result<Vec<i64>, String> {
    let part = input.trim_matches(|c| c == '{' || c == '}');

    let joltage = part.split(',')
        .map(|s| s.parse::<i64>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<i64>, String>>()?;

    Ok(joltage)
}

//endregion

impl Machine {
    fn minimum_presses_needed(&self) -> i64 {
        let mut next_round = vec![self.current_indicators.clone()];
        let mut press_count = 0;

        loop {
            press_count += 1;
            log::debug!("Round {}, {} states to explore", press_count, next_round.len());
            let this_round = next_round.clone();
            next_round.clear();

            for state in this_round {
                for button in &self.buttons {
                    let new_state = press_button(&state, button);

                    if new_state == self.desired_indicators {
                        return press_count;
                    }

                    next_round.push(new_state);
                }
            }

        }
    }
}

fn press_button(indicators: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut out = indicators.clone();

    for &index in button {
        out[index] = !out[index];
    }

    out
}

fn parse(input: &str) -> Vec<Machine> {
    input.trim()
        .lines()
        .map(|l| Machine::try_from(l).unwrap())
        .collect::<Vec<Machine>>()
}


pub async fn run_a(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|m| m.minimum_presses_needed())
        .sum()
}

pub async fn run_b(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());

        assert_eq!(parsed.len(), 3);
    }

    #[tokio::test]
    async fn part_a_example() {
        init();
        assert_eq!(7, run_a(example()).await);
    }

    #[tokio::test]
    async fn part_b_example() {
        init();
        assert_eq!(1, run_b(example()).await);
    }

    #[test]
    fn press_button() {
        assert_eq!(super::press_button(&vec![false, false, false], &vec![0]), vec![true, false, false]);
        assert_eq!(super::press_button(&vec![false, false, false], &vec![0, 2]), vec![true, false, true]);
        assert_eq!(super::press_button(&vec![true, false, true], &vec![0, 1]), vec![false, true, true]);
    }

    mod machine {
        use super::*;

        #[test]
        fn parse() {
            let machines = super::super::parse(example().trim().lines().nth(0).unwrap());
            let machine = &machines[0];

            assert_eq!(machine.current_indicators, vec![false, false, false, false]);
            assert_eq!(machine.desired_indicators, vec![false, true, true, false]);
            assert_eq!(machine.buttons, vec![vec![3], vec![1, 3], vec![2], vec![2, 3], vec![0, 2], vec![0, 1]]);
            assert_eq!(machine.joltage_requirements, vec![3, 5, 4, 7]);
        }

        #[test]
        fn try_parse_indicators() {
            let (indicators, current) = super::try_parse_indicators("[.#..#]").unwrap();

            assert_eq!(indicators, vec![false, true, false, false, true]);
            assert_eq!(current, vec![false, false, false, false, false]);
        }

        #[test]
        fn try_parse_buttons() {
            let buttons = super::try_parse_buttons(&vec!["(3)", "(1,3)", "(2)"]).unwrap();

            assert_eq!(buttons, vec![vec![3], vec![1, 3], vec![2]]);
        }

        #[test]
        fn try_parse_joltage() {
            let joltage = super::try_parse_joltage("{3,5,4,7}").unwrap();

            assert_eq!(joltage, vec![3, 5, 4, 7]);
        }

        #[test]
        fn minimum_presses_needed() {
            let machines = super::super::parse(example());

            assert_eq!(machines[0].minimum_presses_needed(), 2);
            assert_eq!(machines[1].minimum_presses_needed(), 3);
            assert_eq!(machines[2].minimum_presses_needed(), 2);
        }
    }
}