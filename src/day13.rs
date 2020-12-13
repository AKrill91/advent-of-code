use std::collections::HashMap;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let schedule = BusSchedule::from(input);

    let (bus, time) = schedule.earliest_departure();

    bus * (time - schedule.earliest)
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let schedule = BusSchedule::from(input);

    schedule.earliest_alignment()
}


//Yay rosetta code, I have no idea what this is actually doing
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

struct BusSchedule {
    earliest: i64,
    buses: HashMap<usize, i64>,
}

impl<'a, I, J> From<I> for BusSchedule
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    fn from(input: I) -> Self {
        let mut iter = input.into_iter();
        let earliest = iter.next().unwrap().as_ref().parse::<i64>().unwrap();
        let mut buses = HashMap::new();
        let bus_str = iter.next().unwrap().as_ref();

        for (i, bus) in bus_str.split(",").enumerate() {
            if let Ok(n) = bus.parse::<i64>() {
                buses.insert(i, n);
            }
        }

        BusSchedule {
            earliest,
            buses,
        }
    }
}

impl BusSchedule {
    pub fn earliest_departure(&self) -> (i64, i64) {
        let mut out = (i64::max_value(), i64::max_value());
        let earliest = self.earliest as f64;

        for bus in self.buses.values() {
            let id = *bus as f64;

            let multiplier = (earliest / id).ceil() as i64;
            let time = *bus * multiplier;

            if time < out.1 {
                out = (*bus, time);
            }
        }

        out
    }

    pub fn earliest_alignment(&self) -> i64 {
        let mods: Vec<i64> = self.buses.values().cloned().collect();
        let res: Vec<i64> = self.buses
            .iter()
            .map(|(i, b)| *b - *i as i64)
            .collect();

        chinese_remainder(&res, &mods).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "939",
            "7,13,x,x,59,x,31,19",
        ]
    }

    #[test]
    pub fn bus_schedule_from_str() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        let schedule = BusSchedule::from(&sample);

        assert_eq!(939, schedule.earliest);
        assert_eq!(5, schedule.buses.len());
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(295, run_a(&sample));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(1068781, run_b(&sample));
    }

    #[test]
    pub fn sample_input_1_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = vec![
            "1",
            "17,x,13,19"
        ];

        assert_eq!(3417, run_b(&sample));
    }

    #[test]
    pub fn sample_input_2_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = vec![
            "1",
            "67,7,59,61"
        ];

        assert_eq!(754018, run_b(&sample));
    }

    #[test]
    pub fn sample_input_3_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = vec![
            "1",
            "67,x,7,59,61"
        ];

        assert_eq!(779210, run_b(&sample));
    }

    #[test]
    pub fn sample_input_4_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = vec![
            "1",
            "67,7,x,59,61"
        ];

        assert_eq!(1261476, run_b(&sample));
    }

    #[test]
    pub fn sample_input_5_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = vec![
            "1",
            "1789,37,47,1889"
        ];

        assert_eq!(1202161486, run_b(&sample));
    }

}