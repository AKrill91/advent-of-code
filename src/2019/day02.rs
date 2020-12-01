pub fn run_a(input: &Vec<String>, adjust: bool) -> usize {
    let intcodes= parse_intcodes(&input);
    let noun;
    let verb;

    if adjust {
        noun = 12;
        verb = 2;
    } else {
        noun = intcodes[1];
        verb = intcodes[2];
    }

    run(&intcodes, noun, verb)
}

pub fn run_b(input: &Vec<String>, target: usize) -> (usize, usize) {
    let intcodes = parse_intcodes(&input);

    let mut output = (0, 0);

    for noun in 0..intcodes.len() {
        for verb in 0..intcodes.len() {
            let result = run(&intcodes, noun, verb);

            if result == target {
                output = (noun, verb);
                break
            }
        }
    }

    output
}

fn run(input: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut copy = input.clone();

    copy[1] = noun;
    copy[2] = verb;

    let mut i = 0;
    loop {
        let operation = copy[i];

        if operation == 99 {
            break;
        } else if operation == 1 {
            let left = copy[i+1];
            let right = copy[i+2];
            let out = copy[i+3];
            copy[out] = copy[left] + copy[right];
            i = i + 3;
        } else if operation == 2 {
            let left = copy[i+1];
            let right = copy[i+2];
            let out = copy[i+3];
            copy[out] = copy[left] * copy[right];
            i = i + 3;
        } else {
            warn!("Unknown opcode {} encountered at index {}", operation, i);
            break;
        }
        i = i + 1;
    }

    copy[0]
}

fn parse_intcodes(input: &Vec<String>) -> Vec<usize> {
    let mut intcodes= vec!();

    for line in input {
        let parts = line.split(",");
        for part in parts {
            intcodes.push(part.parse::<usize>().unwrap());
        }
    }

    intcodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("1,0,0,0,99")];
        assert_eq!(2, run_a(&input, false));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![String::from("2,3,0,3,99")];
        assert_eq!(2, run_a(&input, false));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![String::from("2,4,4,5,99,0")];
        assert_eq!(2, run_a(&input, false));
    }

    #[test]
    pub fn sample_input_3_a() {
        let input = vec![String::from("1,1,1,4,99,5,6,0,99")];
        assert_eq!(30, run_a(&input, false));
    }
}