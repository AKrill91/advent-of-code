use std::collections::HashSet;
use env_logger::Builder;
use std::thread::sleep;
use std::time::Duration;
use std::{time, thread};

#[derive(Debug,Eq,PartialEq,Hash,Copy,Clone)]
pub enum OpCode {
    Add, //1
    Multiply, //2
    Input, //3
    Output, //4
    JumpIfTrue, //5
    JumpIfFalse, //6
    LessThan, //7
    Equals, //8
    Halt, //99
}

impl OpCode {
    fn from_i32(value: i32) -> OpCode {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            99 => OpCode::Halt,
            _ => panic!("Unknown value: {}", value)
        }
    }

    fn num_parameters(&self) -> usize {
        match self {
            OpCode::Add => 3,
            OpCode::Multiply => 3,
            OpCode::Input => 1,
            OpCode::Output => 1,
            OpCode::JumpIfTrue => 2,
            OpCode::JumpIfFalse => 2,
            OpCode::LessThan => 3,
            OpCode::Equals => 3,
            OpCode::Halt => 0
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

impl ParameterMode {
    fn from_i32(value: i32) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_1_mode: ParameterMode,
    param_2_mode: ParameterMode,
    param_3_mode: ParameterMode
}

pub struct IntCodeComputer {
    unsupported_opcodes: HashSet<OpCode>
}

impl IntCodeComputer {
    pub fn new(unsupported_codes: Vec<OpCode>) -> IntCodeComputer {
        let mut set = HashSet::new();

        for code in unsupported_codes {
            set.insert(code);
        }

        IntCodeComputer{
            unsupported_opcodes: set
        }
    }

    pub fn run(&self, instructions: &Vec<String>, inputs: Vec<i32>) -> Vec<i32> {
        let mut intcodes = parse_intcodes(&instructions);

        let mut input_position = 0;
        let mut instruction_position = 0;
        let mut outputs: Vec<i32> = vec![];

        while instruction_position < intcodes.len() {
            let info = intcodes[instruction_position];
            let opcode = OpCode::from_i32(info % 100);
            let mut increment = 1 + opcode.num_parameters();

            let param_1 = if opcode.num_parameters() > 0 {
                intcodes[instruction_position + 1]
            } else {
                0
            };

            let param_2 = if opcode.num_parameters() > 1 {
                intcodes[instruction_position + 2]
            } else {
                0
            };

            let param_3 = if opcode.num_parameters() > 2 {
                intcodes[instruction_position + 3]
            } else {
                0
            };

            let instruction = Instruction{
                opcode,
                param_1,
                param_2,
                param_3,
                param_1_mode: ParameterMode::from_i32((info / 100) % 10),
                param_2_mode: ParameterMode::from_i32((info / 1000) % 10),
                param_3_mode: ParameterMode::from_i32((info / 10000) % 10),
            };

            if self.unsupported_opcodes.contains(&opcode) {
                panic!("Unsupported opcode: {:?}", opcode);
            }

            debug!("{:?}", instruction);

            match instruction.opcode {
                OpCode::Add => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => {intcodes[instruction.param_1 as usize]},
                        ParameterMode::Immediate => {instruction.param_1},
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => {intcodes[instruction.param_2 as usize]},
                        ParameterMode::Immediate => {instruction.param_2},
                    };

                    intcodes[instruction.param_3 as usize] = left + right;
                },
                OpCode::Multiply => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => {intcodes[instruction.param_1 as usize]},
                        ParameterMode::Immediate => {instruction.param_1},
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => {intcodes[instruction.param_2 as usize]},
                        ParameterMode::Immediate => {instruction.param_2},
                    };

                    intcodes[instruction.param_3 as usize] = left * right;
                },
                OpCode::Input => {
                    intcodes[instruction.param_1 as usize] = inputs[input_position];
                    input_position += 1;
                },
                OpCode::Output => {
                    let out = match instruction.param_1_mode {
                        ParameterMode::Position => intcodes[instruction.param_1 as usize],
                        ParameterMode::Immediate => instruction.param_1
                    };
                    outputs.push(out);
                },
                OpCode::Halt => {
                    break;
                },
                OpCode::JumpIfTrue => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => {intcodes[instruction.param_1 as usize]},
                        ParameterMode::Immediate => {instruction.param_1},
                    };

                    if left > 0 {
                        let right = match instruction.param_2_mode {
                            ParameterMode::Position => { intcodes[instruction.param_2 as usize] },
                            ParameterMode::Immediate => { instruction.param_2 },
                        };

                        instruction_position = right as usize;
                        increment = 0;
                    }
                }
                OpCode::JumpIfFalse => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => {intcodes[instruction.param_1 as usize]},
                        ParameterMode::Immediate => {instruction.param_1},
                    };

                    if left == 0 {
                        let right = match instruction.param_2_mode {
                            ParameterMode::Position => { intcodes[instruction.param_2 as usize] },
                            ParameterMode::Immediate => { instruction.param_2 },
                        };

                        instruction_position = right as usize;
                        increment = 0;
                    }
                }
                OpCode::LessThan => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => {intcodes[instruction.param_1 as usize]},
                        ParameterMode::Immediate => {instruction.param_1},
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => {intcodes[instruction.param_2 as usize]},
                        ParameterMode::Immediate => {instruction.param_2},
                    };

                    intcodes[instruction.param_3 as usize] = if left < right {
                        1
                    } else {
                        0
                    };
                }
                OpCode::Equals => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => {intcodes[instruction.param_1 as usize]},
                        ParameterMode::Immediate => {instruction.param_1},
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => {intcodes[instruction.param_2 as usize]},
                        ParameterMode::Immediate => {instruction.param_2},
                    };

                    intcodes[instruction.param_3 as usize] = if left == right {
                        1
                    } else {
                        0
                    };
                }
            };

            instruction_position += increment;
        }

        outputs
    }
}

fn parse_intcodes(input: &Vec<String>) -> Vec<i32> {
    let mut intcodes: Vec<i32> = vec![];

    for line in input {
        let parts = line.split(",");
        for part in parts {
            intcodes.push(part.parse::<i32>().unwrap());
        }
    }

    intcodes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn sample_input_equals_position() {
        init();
        let instructions = vec![String::from("3,9,8,9,10,9,4,9,99,-1,8")];

        let computer = IntCodeComputer::new(vec![]);

        assert_eq!(1, computer.run(&instructions, vec![8])[0]);
    }

    #[test]
    pub fn sample_input_less_position() {
        let instructions = vec![String::from("3,9,7,9,10,9,4,9,99,-1,8")];

        let computer = IntCodeComputer::new(vec![]);

        assert_eq!(1, computer.run(&instructions, vec![7])[0]);
    }

    #[test]
    pub fn sample_input_equals_immediate() {
        let instructions = vec![String::from("3,3,1108,-1,8,3,4,3,99")];

        let computer = IntCodeComputer::new(vec![]);

        assert_eq!(1, computer.run(&instructions, vec![8])[0]);
    }

    #[test]
    pub fn sample_input_less_immediate() {
        let instructions = vec![String::from("3,3,1107,-1,8,3,4,3,99")];

        let computer = IntCodeComputer::new(vec![]);

        assert_eq!(1, computer.run(&instructions, vec![7])[0]);
    }

    #[test]
    pub fn sample_input_jump_position() {
        let instructions = vec![String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9")];

        let computer = IntCodeComputer::new(vec![]);

        assert_eq!(0, computer.run(&instructions, vec![0])[0]);
    }

    #[test]
    pub fn sample_input_jump_immediate() {
        let instructions = vec![String::from("3,3,1105,-1,9,1101,0,0,12,4,12,99,1")];

        let computer = IntCodeComputer::new(vec![]);

        assert_eq!(1, computer.run(&instructions, vec![42])[0]);
    }

}