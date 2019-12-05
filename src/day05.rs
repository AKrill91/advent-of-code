#[derive(Debug)]
enum OpCode {
    Add, //1
    Multiply, //2
    Input, //3
    Output, //4
    Halt, //99
}

impl OpCode {
    fn from_i32(value: i32) -> OpCode {
        match value {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
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

pub fn run_a(input: &Vec<String>) -> Vec<i64>{
    let inputs = vec![1];
    let mut input_position = 0;
    let mut outputs = vec![];

    let mut intcodes = parse_intcodes(&input);

    let mut position = 0;

    while position < intcodes.len() {
        let adjust = 0;
        let info = intcodes[position];
        let opcode = OpCode::from_i32(info % 100);
        let increment = 1 + opcode.num_parameters();

        debug!("{}", info);

        let param_1 = if opcode.num_parameters() > 0 {
            intcodes[position + 1]
        } else {
            0
        };

        let param_2 = if opcode.num_parameters() > 1 {
            intcodes[position + 2]
        } else {
            0
        };

        let param_3 = if opcode.num_parameters() > 2 {
            intcodes[position + 3]
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
                outputs.push(out as i64);
            },
            OpCode::Halt => {
                break;
            },
        };

        debug!("{:?}", instruction);

        position += increment;
    }

    outputs
}

pub fn run_b(input: &Vec<String>) -> i64 {
    0
}



fn parse_intcodes(input: &Vec<String>) -> Vec<i32> {
    let mut intcodes: Vec<i32> = vec![];

    for line in input {
        let parts = line.split(",");
        for part in parts {
            intcodes.push(part.parse::<i32>().unwrap());
        }
    }

//    let mut position = 0;
//
//    while position < intcodes.len() {
//        let adjust = 0;
//        let info = intcodes[position];
//        let opcode = info % 100;
//
//        let param_1 = 0;
//        let param_2 = 0;
//        let param_3 = 0;
//
//        let instruction = Instruction{
//            opcode,
//            param_1,
//            param_2,
//            param_3,
//            param_1_mode: ParameterMode::from_i32((info / 100) % 10),
//            param_2_mode: ParameterMode::from_i32((info / 1000) % 10),
//            param_3_mode: ParameterMode::from_i32((info / 10000) % 10),
//        };
//    }

    intcodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("3,0,4,0,99")];

        assert_eq!(1, run_a(&input)[0]);
    }
}