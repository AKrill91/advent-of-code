pub fn run_a(input: &Vec<String>) -> i32 {
    let mut pots = parse_initial_state(&input[0]);
    let rules = parse_rules(&input);
    let offset = 0;

    let header = print_header(offset, pots.len());
    let pot_str = print_pots(&pots);

    println!("{}", header);
    println!("{}", pot_str);

    pot_sum(offset, &pots)
}

fn parse_initial_state(input: &str) -> Vec<bool> {
    let mut output = Vec::new();

    let bytes = input.as_bytes();

    for byte in bytes {
        let byte = *byte;

        if byte == 35 { // #
            output.push(true);
        } else if byte == 46 { // .
            output.push(false);
        }
    }

    output
}

fn parse_rules(input: &Vec<String>) {
    for line in &input[2..] {

    }
}

fn print_header(offset: i32, num_pots: usize) -> String {
    let mut output = String::new();

    //Write header
    for i in 0..num_pots {
        let position = offset + (i as i32);
        let abs_position = position.abs() as u32;

        let c = if i == 0 || position % 10 == 0 || i == num_pots - 1 {
            if position < 0 {
                '-'
            } else {
                std::char::from_digit(abs_position / 100, 10).unwrap()
            }
        } else {
            ' '
        };

        output.push(c);
    }

    output.push('\n');

    for i in 0..num_pots {
        let position = offset + (i as i32);
        let abs_position = position.abs() as u32;

        let c = if i == 0 || position % 10 == 0 || i == num_pots - 1 {
            std::char::from_digit((abs_position / 10) % 10, 10).unwrap()
        } else {
            ' '
        };

        output.push(c);
    }

    output.push('\n');

    for i in 0..num_pots {
        let position = offset + (i as i32);
        let abs_position = position.abs() as u32;

        let c = if i == 0 || position % 10 == 0 || i == num_pots - 1 {
            std::char::from_digit(abs_position % 10, 10).unwrap()
        } else {
            ' '
        };

        output.push(c);
    }

    output
}

fn print_pots(pots: &Vec<bool>) -> String {
    let mut output = String::new();

    for pot in pots {
        let c = if *pot {
            '#'
        } else {
            '.'
        };

        output.push(c);
    }

    output
}

fn pot_sum(offset: i32, pots: &Vec<bool>) -> i32 {
    let mut sum = 0;
    for i in 0..pots.len() {
        let position = offset + i as i32;

        if pots[i] {
            sum += position;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_a() {
        let input = vec![
            String::from("initial state: #..#.#..##......###...###"),
            String::from(""),
        String::from("##.#. => #"),
            String::from("#.#.. => #"),
            String::from("##... => ."),
            String::from("...## => #"),
            String::from("###.# => #"),
            String::from("#.##. => #"),
            String::from("#.### => #"),
            String::from("####. => #"),
            String::from(".#..# => #"),
            String::from("...#. => ."),
            String::from("#..#. => ."),
            String::from("#.#.# => ."),
            String::from(".##.# => ."),
            String::from("..#.. => ."),
            String::from(".#.## => #"),
            String::from("..##. => ."),
            String::from(".#.#. => #"),
            String::from("#..## => #"),
            String::from("..#.# => #"),
            String::from("#.... => ."),
            String::from("..### => ."),
            String::from("#...# => ."),
            String::from("##### => #"),
            String::from("###.. => #"),
            String::from("....# => ."),
            String::from("##.## => #"),
            String::from(".#### => ."),
            String::from("..... => ."),
            String::from("##..# => #"),
            String::from(".##.. => ."),
            String::from(".###. => ."),
            String::from(".#... => #"),
        ];

        assert_eq!(325, run_a(&input));
    }
}