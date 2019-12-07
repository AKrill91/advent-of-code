use intcode_computer::{IntCodeComputer, OpCode};

pub fn run_a(input: &Vec<String>) -> Vec<i32>{
    let unsupported = vec![OpCode::JumpIfTrue, OpCode::JumpIfFalse, OpCode::LessThan, OpCode::Equals];
    let mut computer = IntCodeComputer::new(unsupported);

    computer.run(&input, vec![1])
}

pub fn run_b(input: &Vec<String>) -> Vec<i32> {
    let mut computer = IntCodeComputer::new(vec![]);

    computer.run(&input, vec![5])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("3,0,4,0,99")];

        assert_eq!(1, run_a(&input)[0]);
    }

    pub fn sample_input_0_b() {
        let input = vec![String::from("3,9,8,9,10,9,4,9,99,-1,8")];
    }
}