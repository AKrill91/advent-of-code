pub fn run_a(input: &Vec<String>, adjust: bool) -> i64 {
    let mut intcodes= vec!();

    for line in input {
        let parts = line.split(",");
        for part in parts {
            intcodes.push(part.parse::<i64>().unwrap());
        }
    }

    if adjust {
        intcodes[1] = 12;
        intcodes[2] = 2;
    }

    let mut i = 0;
    loop {
        let operation = intcodes[i];

        if operation == 99 {
            break;
        } else if operation == 1 {
            let left = intcodes[i+1] as usize;
            let right = intcodes[i+2] as usize;
            let out = intcodes[i+3] as usize;
            intcodes[out] = intcodes[left] + intcodes[right];
            i = i + 3;
        } else if operation == 2 {
            let left = intcodes[i+1] as usize;
            let right = intcodes[i+2] as usize;
            let out = intcodes[i+3] as usize;
            intcodes[out] = intcodes[left] * intcodes[right];
            i = i + 3;
        } else {
            warn!("Unknown opcode {} encountered at index {}", operation, i);
            break;
        }
        i = i + 1;
    }

    intcodes[0]
}

pub fn run_b(input: &Vec<String>) {

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