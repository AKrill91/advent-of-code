pub fn run_a(input: &Vec<String>) -> i64{
    let mut sum = 0;
    for line in input {
        let mass = line.parse::<i64>().unwrap();
        sum += find_fuel(mass, false);
    }

    sum
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in input {
        let mass = line.parse::<i64>().unwrap();
        sum += find_fuel(mass, true);
    }

    sum
}

fn find_fuel(mass: i64, include_for_fuel: bool) -> i64 {
    let mut total_fuel= (mass / 3) - 2;

    if include_for_fuel {
        debug!("{} fuel for module", total_fuel);
        let mut fuel_for_fuel = (total_fuel / 3) - 2;

        while fuel_for_fuel > 0 {
            debug!("{} required for that fuel", fuel_for_fuel);
            total_fuel += fuel_for_fuel;
            fuel_for_fuel = (fuel_for_fuel / 3) - 2;
        }
    }

    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("12")];

        assert_eq!(2, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![String::from("14")];

        assert_eq!(2, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![String::from("1969")];

        assert_eq!(654, run_a(&input));
    }

    #[test]
    pub fn sample_input_3_a() {
        let input = vec![String::from("100756")];

        assert_eq!(33583, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let input = vec![String::from("14")];

        assert_eq!(2, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let input = vec![String::from("1969")];

        assert_eq!(966, run_b(&input));
    }

    #[test]
    pub fn sample_input_2_b() {
        let input = vec![String::from("100756")];

        assert_eq!(50346, run_b(&input));
    }
}