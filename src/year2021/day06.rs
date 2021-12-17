
pub fn run_a(_: i32, input: &[String]) -> String {
    let fishies = run_days(80, input);

    format!("{}", fishies.len())
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("")
}


fn run_days(num_days: usize, input: &[String]) -> Vec<LanternFish> {
    let mut fishies = LanternFish::parse_fish(&input[0]);

    for _ in 0..num_days {
        let num_to_add = fishies.iter_mut()
            .filter_map(|f: &mut LanternFish| {
                if f.tick() { Some(1) } else {None}
            })
            .count();

        for _ in 0..num_to_add {
            fishies.push(LanternFish::new());
        }
    }

    fishies
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanternFish {
    timer: u8,
}

impl From<&str> for LanternFish {
    fn from(s: &str) -> Self {
        LanternFish {
            timer: s.parse::<u8>().unwrap(),
        }
    }
}

impl LanternFish {
    fn new() -> Self {
        LanternFish {
            timer: 8
        }
    }

    fn tick(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }

    fn parse_fish(line: &str) -> Vec<Self> {
        line.split(',')
            .map(LanternFish::from)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::year2021::{run, run_day};
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse_fish() {
        let expected: Vec<u8> = vec![4,3,2,1];
        let actual: Vec<u8> = LanternFish::parse_fish("4,3,2,1")
            .into_iter()
            .map(|f| f.timer)
            .collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_fish_tick() {
        let mut fish = LanternFish{timer: 1};
        assert_eq!(false, fish.tick());
        assert_eq!(0, fish.timer);
        assert_eq!(true, fish.tick());
        assert_eq!(6, fish.timer);
    }

    fn get_sample() -> Vec<String> {
        vec![
            "3,4,3,1,2"
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "5934";

        assert_eq!(expected, run_a(0, &input));
    }

    #[test]
    fn test_sample_a_small() {
        init();
        let input = get_sample();
        let expected = "26";
        assert_eq!(expected, run_days(18, &input).len().to_string());
    }

    #[test]
    fn test_sample_b() {
        init();

        let input = get_sample();

        let expected = "";

        assert_eq!(expected, run_b(0, &input));
    }
}