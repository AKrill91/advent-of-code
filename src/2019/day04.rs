use std::collections::HashSet;

pub fn run_a(input: &Vec<String>) -> i64{
    run(input, has_double)
}

pub fn run_b(input: &Vec<String>) -> i64 {
    run(input, has_unique_double)
}

fn run(input: &Vec<String>, check_function: fn(i32) -> bool) -> i64 {
    let (min, max) = parse_range(&input[0]);

    let mut counter = find_first_valid(min);
    let mut num_checked = 0;
    let mut num_valid = 0;

    while counter <= max {
        debug!("Checking {}", counter);
        num_checked += 1;

        if check_function(counter) {
            num_valid += 1;
        }

        let mut increment = 1;
        let ones_place = counter % 10;

        if ones_place == 9 {
            let tens_place = (counter / 10) % 10;

            if tens_place == 9 {
                let hundreds_place = (counter / 100) % 10;

                if hundreds_place == 9 {
                    let thousands_place = (counter / 1_000) % 10;

                    if thousands_place == 9 {
                        let ten_thousands_place =  (counter / 10_000) % 10;

                        if ten_thousands_place == 9 {
                            let hundred_thousands_place = (counter / 100_000) % 10;
                            increment += 11_111 * (hundred_thousands_place + 1);
                        } else  {
                            increment += 1_111 * (ten_thousands_place + 1);
                        }
                    } else {
                        increment += 111 * (thousands_place + 1);
                    }
                } else {
                    increment += 11 * (hundreds_place + 1);
                }
            } else {
                increment += tens_place + 1;
            }
        }

        counter += increment;
    }

    debug!("Checked {} passwords, {} are valid", num_checked, num_valid);

    num_valid
}

fn find_first_valid(input: i32) -> i32 {
    let hun_tho = input / 100_000;
    let mut ten_tho = (input / 10_000) % 10;
    let mut tho = (input / 1_000) % 10;
    let mut hun = (input / 100) % 10;
    let mut ten = (input / 10) % 10;
    let mut one = input % 10;

    if ten_tho < hun_tho {
        ten_tho = hun_tho;
        tho = hun_tho;
        hun = hun_tho;
        ten = hun_tho;
        one = hun_tho;
    } else if tho < ten_tho {
        tho = ten_tho;
        hun = ten_tho;
        ten = ten_tho;
        one = ten_tho;
    } else if hun < tho  {
        hun = tho;
        ten = tho;
        one = tho;
    } else if ten < hun {
        ten = hun;
        one = hun;
    } else if one < ten {
        one = ten;
    }

    hun_tho * 100_000 + ten_tho * 10_000 + tho * 1_000 + hun * 100 + ten * 10 + one
}

fn has_double(input: i32) -> bool {
    for i in (2..7).rev() {
        if ((input % 10_i32.pow(i)) / 10_i32.pow(i-2)) % 11 == 0 {
            return true;
        }
    }

    false
}

pub fn has_unique_double(input: i32) -> bool {
    let as_str = input.to_string();
    let mut last_digit = '~';
    let mut run_length = 0;
    let mut run_lengths = HashSet::new();

    for digit in as_str.chars() {
        if digit == last_digit {
            run_length += 1;
        } else {
            if run_length > 0 {
                run_lengths.insert(run_length);
            }

            run_length = 1;
            last_digit = digit;
        }
    }

    run_lengths.insert(run_length);

    run_lengths.contains(&2)
}

fn parse_range(input: &str) -> (i32, i32) {
    let parts = input.split("-").collect::<Vec<&str>>();
    let min = parts[0].parse::<i32>().unwrap();
    let max = parts[1].parse::<i32>().unwrap();

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_unique_double() {
        let input = 112233;

        assert_eq!(true, has_unique_double(input));
    }

    #[test]
    pub fn sample_input_1_unique_double() {
        let input = 123444;

        assert_eq!(false, has_unique_double(input));
    }

    #[test]
    pub fn sample_input_2_unique_double() {
        let input = 111122;

        assert_eq!(true, has_unique_double(input));
    }
}