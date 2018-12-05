pub fn run_a(bytes: &[u8]) -> String {
    let mut output: Vec<u8> = Vec::new();

    for byte in bytes {
        let next = *byte;
        if output.len() == 0 {
            output.push(next);
            continue;
        }

        let last_32 = *output.last().unwrap() as i32;
        let next_32 = next as i32;
        let diff = (last_32 - next_32).abs();
        if diff == 32 {
            output.pop();
        } else {
            output.push(next);
        }
    }

    let result = String::from_utf8(output).unwrap();

    info!("Result is {}", result);
    info!("Result has a length of {}", result.len());

    result
}

pub fn run_b(bytes: &[u8]) -> String {
    let mut shortest_len = std::usize::MAX;
    let mut shortest_result: String = String::new();

    for to_remove in 65..90 {
        let mut output: Vec<u8> = Vec::new();

        for byte in bytes {
            let next = *byte;
            if next == to_remove || next == to_remove + 32 {
                continue;
            }
            if output.len() == 0 {
                output.push(next);
                continue;
            }

            let last_32 = *output.last().unwrap() as i32;
            let next_32 = next as i32;
            let diff = (last_32 - next_32).abs();
            if diff == 32 {
                output.pop();
            } else {
                output.push(next);
            }
        }

        let len = output.len();

        info!("Removing {} resulted in a length of {}", to_remove, len);

        if len < shortest_len {
            shortest_result = String::from_utf8(output).unwrap();
            shortest_len = len;
        }
    }

    info!("Shortest result is {}", shortest_result);
    info!("Shortest result has a length of {}", shortest_len);

    shortest_result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_input_a_nothing() {
        let sample = "aA".as_bytes();

        assert_eq!(String::from(""), run_a(&sample));
    }

    #[test]
    fn sample_input_a_double_nothing() {
        let sample = "abBA".as_bytes();

        assert_eq!(String::from(""), run_a(&sample));
    }

    #[test]
    fn sample_input_a_no_change() {
        let sample = "abAB".as_bytes();

        assert_eq!(String::from("abAB"), run_a(&sample));
    }

    #[test]
    fn sample_input_a_still_no_change() {
        let sample = "aabAAB".as_bytes();

        assert_eq!(String::from("aabAAB"), run_a(&sample));
    }

    #[test]
    fn sample_input_a_large() {
        let sample = "dabAcCaCBAcCcaDA".as_bytes();

        assert_eq!(String::from("dabCBAcaDA"), run_a(&sample));
    }

    #[test]
    fn sample_input_b() {
        let sample = "dabAcCaCBAcCcaDA".as_bytes();
        let result = run_b(&sample);

        assert_eq!(4, result.len());
    }
}