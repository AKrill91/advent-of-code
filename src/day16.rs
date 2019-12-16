pub fn run_a(input: &Vec<String>) -> [u32; 8] {
    run(input, 100)
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}

pub fn run(input: &Vec<String>, num_phases: usize) -> [u32; 8] {
    let base_pattern: [i32; 4] = [0, 1, 0, -1];
    let mut signal = parse_string(input[0].as_str());
    let length = signal.len();
    info!("Signal is {} digits long", length);

    for phase in 0..num_phases {
        debug!("Running phase {}", phase);
        let mut new_signal = Vec::with_capacity(length);
        let mut pattern_pos = 1;

        for element in 1..length+1 {
            let mut sum = 0;

            for index in 0..length {
                let digit = signal[index];
                let pattern = base_pattern[get_pattern_index(element, index)];
                let amount = digit as i32 * pattern;
                sum += amount;
            }
            new_signal.push(sum.abs() as u32 % 10);
        }

        signal = new_signal;
    }

    assert!(length >= 8);

    [
        signal[0],
        signal[1],
        signal[2],
        signal[3],
        signal[4],
        signal[5],
        signal[6],
        signal[7]
    ]
}

pub fn get_pattern_index(element: usize, index: usize) -> usize {
    ((index + 1) % (4 * element)) / element
}

fn parse_string(input: &str) -> Vec<u32> {
    let mut out = Vec::with_capacity(input.len());

    for c in input.chars() {
        assert!(c.is_digit(10));

        out.push(c.to_digit(10).unwrap());
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn get_pattern_index_phase_1() {
        assert_eq!(1, get_pattern_index(1, 0));
        assert_eq!(2, get_pattern_index(1, 1));
        assert_eq!(3, get_pattern_index(1, 2));
        assert_eq!(0, get_pattern_index(1, 3));
        assert_eq!(1, get_pattern_index(1, 4));
    }

    #[test]
    pub fn get_pattern_index_phase_2() {
        assert_eq!(0, get_pattern_index(2, 0));
        assert_eq!(1, get_pattern_index(2, 1));
        assert_eq!(1, get_pattern_index(2, 2));
        assert_eq!(2, get_pattern_index(2, 3));
        assert_eq!(2, get_pattern_index(2, 4));
        assert_eq!(3, get_pattern_index(2, 5));
        assert_eq!(3, get_pattern_index(2, 6));
        assert_eq!(0, get_pattern_index(2, 7));
    }

    #[test]
    pub fn get_pattern_index_phase_3() {
        assert_eq!(0, get_pattern_index(3, 0));
        assert_eq!(0, get_pattern_index(3, 1));
        assert_eq!(1, get_pattern_index(3, 2));
        assert_eq!(1, get_pattern_index(3, 3));
        assert_eq!(1, get_pattern_index(3, 4));
        assert_eq!(2, get_pattern_index(3, 5));
        assert_eq!(2, get_pattern_index(3, 6));
        assert_eq!(2, get_pattern_index(3, 7));
    }

    #[test]
    pub fn sample_input_small() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![String::from("12345678")];
        let expected: [u32; 8] = [0, 1, 0, 2, 9, 4, 9, 8];

        assert_eq!(expected, run(&input, 4));
    }

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("80871224585914546619083218645595")];
        let expected: [u32; 8] = [2, 4, 1, 7, 6, 1, 7, 6];

        assert_eq!(expected, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![String::from("19617804207202209144916044189917")];
        let expected: [u32; 8] = [7, 3, 7, 4, 5, 4, 1, 8];

        assert_eq!(expected, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![String::from("69317163492948606335995924319873")];
        let expected: [u32; 8] = [5, 2, 4, 3, 2, 1, 3, 3];

        assert_eq!(expected, run_a(&input));
    }
}