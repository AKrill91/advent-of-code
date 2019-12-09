use std::collections::{HashSet, HashMap};

const UNINITIALIZED_MEMORY: i64 = 0;

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
    AdjustBase, //9
    Halt, //99
}

impl OpCode {
    fn from_i64(value: i64) -> OpCode {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustBase,
            99 => OpCode::Halt,
            _ => panic!("Unknown value: {}", value)
        }
    }

    fn num_parameters(&self) -> i64 {
        match self {
            OpCode::Add => 3,
            OpCode::Multiply => 3,
            OpCode::Input => 1,
            OpCode::Output => 1,
            OpCode::JumpIfTrue => 2,
            OpCode::JumpIfFalse => 2,
            OpCode::LessThan => 3,
            OpCode::Equals => 3,
            OpCode::AdjustBase => 1,
            OpCode::Halt => 0
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_i64(value: i64) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown value: {}", value)
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    param_1: i64,
    param_2: i64,
    param_3: i64,
    param_1_mode: ParameterMode,
    param_2_mode: ParameterMode,
    param_3_mode: ParameterMode,
}

pub struct IntCodeComputer {
    unsupported_opcodes: HashSet<OpCode>
}

pub struct IntCodeProgram {
    unsupported_opcodes: HashSet<OpCode>,
    intcodes: HashMap<i64, i64>,
    position: i64,
    outputs: Vec<i64>,
    halted: bool,
    relative_base: i64
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

    pub fn run(&self, instructions: &Vec<String>, inputs: Vec<i64>) -> Vec<i64> {
        let mut program = IntCodeProgram {
            unsupported_opcodes: self.unsupported_opcodes.clone(),
            intcodes: IntCodeComputer::parse_intcodes(instructions),
            position: 0,
            outputs: vec![],
            halted: false,
            relative_base: 0
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
            relative_base: 0
        }
    }

    fn parse_intcodes(input: &Vec<String>) -> HashMap<i64, i64> {
        let mut intcodes: HashMap<i64, i64> = HashMap::new();
        let mut position = 0;

        for line in input {
            let parts = line.split(",");
            for part in parts {
                intcodes.insert(position,part.parse::<i64>().unwrap());
                position += 1;
            }
        }

        intcodes
    }
}

impl IntCodeProgram {

    ///Runs until input is needed.
    pub fn run(&mut self) {
        while self.position < self.intcodes.len() as i64 {
            let instruction = self.parse_instruction();
            let mut increment = 1 + instruction.opcode.num_parameters();

            match instruction.opcode {
                OpCode::Add => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);
                    let right = self.lookup_value(instruction.param_2, instruction.param_2_mode);
                    self.write_value(instruction.param_3, instruction.param_3_mode, left + right);
                }
                OpCode::Multiply => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);
                    let right = self.lookup_value(instruction.param_2, instruction.param_2_mode);
                    self.write_value(instruction.param_3, instruction.param_3_mode, left * right);
                }
                OpCode::Input => {
                    break;
                }
                OpCode::Output => {
                    let out = self.lookup_value(instruction.param_1, instruction.param_1_mode);

                    self.outputs.push(out);
                }
                OpCode::JumpIfTrue => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);

                    if left > 0 {
                        let right = self.lookup_value(instruction.param_2, instruction.param_2_mode);

                        self.position = right;
                        increment = 0;
                    }
                }
                OpCode::JumpIfFalse => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);

                    if left == 0 {
                        let right = self.lookup_value(instruction.param_2, instruction.param_2_mode);

                        self.position = right;
                        increment = 0;
                    }
                }
                OpCode::LessThan => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);
                    let right = self.lookup_value(instruction.param_2, instruction.param_2_mode);

                    self.write_value(
                        instruction.param_3,
                        instruction.param_3_mode,
                        if left < right {
                            1
                        } else {
                            0
                        }
                    );
                }
                OpCode::Equals => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);
                    let right = self.lookup_value(instruction.param_2, instruction.param_2_mode);

                    self.write_value(
                        instruction.param_3,
                        instruction.param_3_mode,
                        if left == right {
                            1
                        } else {
                            0
                        }
                    );
                }
                OpCode::AdjustBase => {
                    let left = self.lookup_value(instruction.param_1, instruction.param_1_mode);

                    self.relative_base += left;
                }
                OpCode::Halt => {
                    self.halted = true;
                    break;
                }
            };

            self.position += increment;
        }
    }

    ///Runs with given input until more input is required
    pub fn input(&mut self, input: i64) {
        let instruction = self.parse_instruction();

        assert_eq!(OpCode::Input, instruction.opcode);

        self.write_value(instruction.param_1, instruction.param_1_mode, input);

        self.position += 1 + instruction.opcode.num_parameters();

        self.run();
    }

    fn parse_instruction(&self) -> Instruction {
        let info = *self.intcodes.get(&self.position).unwrap();
        let opcode = OpCode::from_i64(info % 100);

        let position_1 = self.position + 1;
        let position_2 = self.position + 2;
        let position_3 = self.position + 3;

        let param_1 = if opcode.num_parameters() > 0 {
            *self.intcodes.get(&position_1).unwrap()
        } else {
            0
        };

        let param_2 = if opcode.num_parameters() > 1 {
            *self.intcodes.get(&position_2).unwrap()
        } else {
            0
        };

        let param_3 = if opcode.num_parameters() > 2 {
            *self.intcodes.get(&position_3).unwrap()
        } else {
            0
        };

        let instruction = Instruction {
            opcode,
            param_1,
            param_2,
            param_3,
            param_1_mode: ParameterMode::from_i64((info / 100) % 10),
            param_2_mode: ParameterMode::from_i64((info / 1000) % 10),
            param_3_mode: ParameterMode::from_i64((info / 10000) % 10),
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

    pub fn latest_output(&self) -> Option<i64> {
        let mut out = None;
        let len = self.outputs.len();

        if len > 0 {
            out = Some(self.outputs[len - 1]);
        }

        out
    }

    fn lookup_value(&self, parameter: i64, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => { *self.intcodes.get(&parameter).unwrap_or(&UNINITIALIZED_MEMORY) }
            ParameterMode::Immediate => { parameter }
            ParameterMode::Relative => { *self.intcodes.get(&(parameter + self.relative_base)).unwrap_or(&UNINITIALIZED_MEMORY)}
        }
    }

    fn write_value(&mut self, parameter: i64, mode: ParameterMode, value: i64) {
        match mode {
            ParameterMode::Position => {self.intcodes.insert(parameter, value);}
            ParameterMode::Immediate => {panic!("Attempt to write in immediate mode");}
            ParameterMode::Relative => {self.intcodes.insert((parameter + self.relative_base), value);}
        }
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

    #[test]
    pub fn sample_input_quine() {
        let instructions = vec![String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99")];

        let expected: Vec<i64> = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];

        let computer = IntCodeComputer::new(vec![]);

        let output = computer.run(&instructions, vec![]);

        assert_eq!(expected, output);
    }

    #[test]
    pub fn sample_input_huge_number() {
        let instructions = vec![String::from("1102,34915192,34915192,7,4,7,99,0")];

        let computer = IntCodeComputer::new(vec![]);

        let output = computer.run(&instructions, vec![]);
        let as_str = output[0].to_string();

        assert_eq!(16, as_str.len());
    }

    #[test]
    pub fn sample_input_large_number() {
        let instructions = vec![String::from("104,1125899906842624,99")];

        let computer = IntCodeComputer::new(vec![]);

        let expected: Vec<i64> = vec![1125899906842624];

        let output = computer.run(&instructions, vec![]);

        assert_eq!(expected, output);
    }
}