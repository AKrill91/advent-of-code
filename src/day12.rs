pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let instructions: Vec<FerryInstruction> = input.into_iter()
        .map(|i| FerryInstruction::from(i.as_ref()))
        .collect();

    let mut ferry = Ferry::new();

    for instruction in instructions {
        ferry = ferry.act(&instruction);
    }

    ferry.position.0.abs() + ferry.position.1.abs()
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let instructions: Vec<FerryInstruction> = input.into_iter()
        .map(|i| FerryInstruction::from(i.as_ref()))
        .collect();

    let mut ferry = Ferry::new();
    let mut waypoint = (10, 1);

    for instruction in instructions {
        ferry = ferry.act_on_waypoint(&instruction, &mut waypoint);
    }

    ferry.position.0.abs() + ferry.position.1.abs()
}

struct Ferry {
    heading: i64,
    position: (i64, i64),
}

impl Ferry {
    pub fn new() -> Self {
        Ferry {
            heading: 0,
            position: (0, 0)
        }
    }

    pub fn act(&self, instruction: &FerryInstruction) -> Ferry {
        let mut adjust = (0, 0);
        let mut heading = self.heading;
        match instruction.action {
            FerryAction::North => {
                adjust = (0, 1)
            }
            FerryAction::South => {
                adjust = (0, -1)
            }
            FerryAction::East => {
                adjust = (1, 0)
            }
            FerryAction::West => {
                adjust = (-1, 0)
            }
            FerryAction::Left => {
                heading += instruction.amount
            }
            FerryAction::Right => {
                heading -= instruction.amount
            }
            FerryAction::Forward => {
                info!("Moving forward with heading of {}", heading);
                adjust = match heading {
                    0 => (1,0),
                    90 => (0, 1),
                    180 => (-1, 0),
                    270 => (0, -1),
                    _ => panic!()
                }
            }
        }

        adjust = (adjust.0 * instruction.amount, adjust.1 * instruction.amount);
        heading = if heading < 0 {
            heading + 360
        } else if heading >= 360 {
            heading % 360
        } else { heading };

        Ferry {
            heading,
            position: (self.position.0 + adjust.0, self.position.1 + adjust.1)
        }
    }

    pub fn act_on_waypoint(&self, instruction: &FerryInstruction, waypoint: &mut (i64, i64)) -> Ferry {
        let mut position = self.position;

        match instruction.action {
            FerryAction::North => {
                waypoint.1 += instruction.amount;
            }
            FerryAction::South => {
                waypoint.1 -= instruction.amount;
            }
            FerryAction::East => {
                waypoint.0 += instruction.amount;
            }
            FerryAction::West => {
                waypoint.0 -= instruction.amount;
            }
            FerryAction::Left => {
                rotate_waypoint(waypoint, instruction.amount);
            }
            FerryAction::Right => {
                rotate_waypoint(waypoint, instruction.amount * -1);
            }
            FerryAction::Forward => {
                position = (self.position.0 + waypoint.0 * instruction.amount, self.position.1 + waypoint.1 * instruction.amount)
            }
        }

        Ferry {
            heading: self.heading,
            position
        }
    }


}

fn rotate_waypoint(waypoint: &mut (i64, i64), amount: i64) {
    let amount = (amount + 360) % 360;
    match amount {
        0 => {},
        90 => {
            let tmp = waypoint.0;
            waypoint.0 = waypoint.1 * -1;
            waypoint.1 = tmp;
        },
        180 => {
            waypoint.0 *= -1;
            waypoint.1 *= -1;
        },
        270 => {
            let tmp = waypoint.0;
            waypoint.0 = waypoint.1;
            waypoint.1 = tmp * -1;
        },
        _ => panic!()
    }
}

enum FerryAction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl From<char> for FerryAction {
    fn from(c: char) -> Self {
        match c {
            'N' => FerryAction::North,
            'S' => FerryAction::South,
            'E' => FerryAction::East,
            'W' => FerryAction::West,
            'L' => FerryAction::Left,
            'R' => FerryAction::Right,
            'F' => FerryAction::Forward,
            _ => panic!()
        }
    }
}

struct FerryInstruction {
    action: FerryAction,
    amount: i64,
}

impl<'a> From<&'a str> for FerryInstruction {
    fn from(input: &'a str) -> Self {
        let mut chars = input.chars();
        let action = FerryAction::from(chars.next().unwrap());

        let digits: String = chars.collect();

        FerryInstruction {
            action,
            amount: digits.parse::<i64>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11",
        ]
    }

    #[test]
    pub fn rotate_waypoint_cw_0() {
        let mut waypoint = (2, 3);
        rotate_waypoint(&mut waypoint, 0);
        assert_eq!((2, 3), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_90_0() {
        let mut waypoint = (2, 3);
        rotate_waypoint(&mut waypoint, 90);
        assert_eq!((-3, 2), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_90_1() {
        let mut waypoint = (-3, 2);
        rotate_waypoint(&mut waypoint, 90);
        assert_eq!((-2, -3), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_90_2() {
        let mut waypoint = (-2, -3);
        rotate_waypoint(&mut waypoint, 90);
        assert_eq!((3, -2), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_90_3() {
        let mut waypoint = (3, -2);
        rotate_waypoint(&mut waypoint, 90);
        assert_eq!((2, 3), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_180() {
        let mut waypoint = (2, 3);
        rotate_waypoint(&mut waypoint, 180);
        assert_eq!((-2, -3), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_270_0() {
        let mut waypoint = (2, 3);
        rotate_waypoint(&mut waypoint, 270);
        assert_eq!((3, -2), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_270_1() {
        let mut waypoint = (3, -2);
        rotate_waypoint(&mut waypoint, 270);
        assert_eq!((-2, -3), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_270_2() {
        let mut waypoint = (-2, -3);
        rotate_waypoint(&mut waypoint, 270);
        assert_eq!((-3, 2), waypoint);
    }

    #[test]
    pub fn rotate_waypoint_cw_270_3() {
        let mut waypoint = (-3, 2);
        rotate_waypoint(&mut waypoint, 270);
        assert_eq!((2, 3), waypoint);
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(25, run_a(&sample));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(286, run_b(&sample));
    }
}