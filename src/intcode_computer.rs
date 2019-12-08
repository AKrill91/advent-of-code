use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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
    Immediate,
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
    param_3_mode: ParameterMode,
}

pub struct IntCodeComputer {
    unsupported_opcodes: HashSet<OpCode>
}

pub struct IntCodeProgram {
    unsupported_opcodes: HashSet<OpCode>,
    intcodes: Vec<i32>,
    position: usize,
    outputs: Vec<i32>,
    halted: bool,
}

impl IntCodeComputer {
    pub fn new(unsupported_codes: Vec<OpCode>) -> IntCodeComputer {
        let mut set = HashSet::new();

        for code in unsupported_codes {
            set.insert(code);
        }

        IntCodeComputer {
            unsupported_opcodes: set
        }
    }

    pub fn run(&self, instructions: &Vec<String>, inputs: Vec<i32>) -> Vec<i32> {
        let mut program = IntCodeProgram {
            unsupported_opcodes: self.unsupported_opcodes.clone(),
            intcodes: IntCodeComputer::parse_intcodes(instructions),
            position: 0,
            outputs: vec![],
            halted: false,
        };

        program.run();

        for i in inputs {
            program.input(i);
        }

        program.outputs
    }

    pub fn start(&self, instructions: &Vec<String>) -> IntCodeProgram {
        IntCodeProgram {
            unsupported_opcodes: self.unsupported_opcodes.clone(),
            intcodes: IntCodeComputer::parse_intcodes(instructions),
            position: 0,
            outputs: vec![],
            halted: false,
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
}

impl IntCodeProgram {

    ///Runs until input is needed.
    pub fn run(&mut self) {
        while self.position < self.intcodes.len() {
            let instruction = self.parse_instruction();
            let mut increment = 1 + instruction.opcode.num_parameters();

            match instruction.opcode {
                OpCode::Add => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_1 as usize] }
                        ParameterMode::Immediate => { instruction.param_1 }
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_2 as usize] }
                        ParameterMode::Immediate => { instruction.param_2 }
                    };

                    self.intcodes[instruction.param_3 as usize] = left + right;
                }
                OpCode::Multiply => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_1 as usize] }
                        ParameterMode::Immediate => { instruction.param_1 }
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_2 as usize] }
                        ParameterMode::Immediate => { instruction.param_2 }
                    };

                    self.intcodes[instruction.param_3 as usize] = left * right;
                }
                OpCode::Input => {
                    break;
                }
                OpCode::Output => {
                    let out = match instruction.param_1_mode {
                        ParameterMode::Position => self.intcodes[instruction.param_1 as usize],
                        ParameterMode::Immediate => instruction.param_1
                    };

                    self.outputs.push(out);
                }
                OpCode::Halt => {
                    self.halted = true;
                    break;
                }
                OpCode::JumpIfTrue => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_1 as usize] }
                        ParameterMode::Immediate => { instruction.param_1 }
                    };

                    if left > 0 {
                        let right = match instruction.param_2_mode {
                            ParameterMode::Position => { self.intcodes[instruction.param_2 as usize] }
                            ParameterMode::Immediate => { instruction.param_2 }
                        };

                        self.position = right as usize;
                        increment = 0;
                    }
                }
                OpCode::JumpIfFalse => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_1 as usize] }
                        ParameterMode::Immediate => { instruction.param_1 }
                    };

                    if left == 0 {
                        let right = match instruction.param_2_mode {
                            ParameterMode::Position => { self.intcodes[instruction.param_2 as usize] }
                            ParameterMode::Immediate => { instruction.param_2 }
                        };

                        self.position = right as usize;
                        increment = 0;
                    }
                }
                OpCode::LessThan => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_1 as usize] }
                        ParameterMode::Immediate => { instruction.param_1 }
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_2 as usize] }
                        ParameterMode::Immediate => { instruction.param_2 }
                    };

                    self.intcodes[instruction.param_3 as usize] = if left < right {
                        1
                    } else {
                        0
                    };
                }
                OpCode::Equals => {
                    let left = match instruction.param_1_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_1 as usize] }
                        ParameterMode::Immediate => { instruction.param_1 }
                    };

                    let right = match instruction.param_2_mode {
                        ParameterMode::Position => { self.intcodes[instruction.param_2 as usize] }
                        ParameterMode::Immediate => { instruction.param_2 }
                    };

                    self.intcodes[instruction.param_3 as usize] = if left == right {
                        1
                    } else {
                        0
                    };
                }
            };

            self.position += increment;
        }
    }

    ///Runs with given input until more input is required
    pub fn input(&mut self, input: i32) {
        let instruction = self.parse_instruction();

        assert_eq!(OpCode::Input, instruction.opcode);

        self.intcodes[instruction.param_1 as usize] = input;
        self.position += 1 + instruction.opcode.num_parameters();

        self.run();
    }

    fn parse_instruction(&self) -> Instruction {
        let info = self.intcodes[self.position];
        let opcode = OpCode::from_i32(info % 100);

        let param_1 = if opcode.num_parameters() > 0 {
            self.intcodes[self.position + 1]
        } else {
            0
        };

        let param_2 = if opcode.num_parameters() > 1 {
            self.intcodes[self.position + 2]
        } else {
            0
        };

        let param_3 = if opcode.num_parameters() > 2 {
            self.intcodes[self.position + 3]
        } else {
            0
        };

        let instruction = Instruction {
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

        instruction
    }

    pub fn halted(&self) -> bool {
        return self.halted;
    }

    pub fn latest_output(&self) -> Option<i32> {
        let mut out = None;
        let len = self.outputs.len();

        if len > 0 {
            out = Some(self.outputs[len - 1]);
        }

        out
    }
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