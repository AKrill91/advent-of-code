use std::collections::{HashSet, HashMap};

pub fn run_a(input: &Vec<String>) -> i64 {
    let program = Program::from(input);

    let mut execution = Execution::new(program);

    let mut executed_instructions = HashSet::new();
    let mut counter = execution.instruction_counter;

    while !executed_instructions.contains(&counter) {
        executed_instructions.insert(counter);
        execution.step();

        counter = execution.instruction_counter
    }

    execution.accumulator as i64
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let mut swap_after = 0;

    while swap_after < input.len() {
        let mut program = Program::from(input);
        swap_after = program.swap_nop_and_jmp(swap_after);
        let mut execution = Execution::new(program);

        execution.run();

        if !execution.looping {
            return execution.accumulator as i64
        }
    }

    -1
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        match input {
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            "nop" => Operation::Nop,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let mut parts = input.split(" ");
        let operation = Operation::from(parts.next().unwrap());
        let argument = parts.next().unwrap().parse::<i32>().unwrap();

        Instruction {
            operation,
            argument,
        }
    }
}

struct Program {
    instructions: Vec<Instruction>
}

impl<'a, I, J> From<I> for Program
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    fn from(lines: I) -> Self {
        let mut instructions = vec![];

        for line in lines {
            instructions.push(Instruction::from(line.as_ref()));
        }

        Program {
            instructions
        }
    }
}

impl Program {
    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn swap_nop_and_jmp(&mut self, start: usize) -> usize {
        for i in start..self.instructions.len() {
            let instruction = &mut self.instructions[i];

            if instruction.operation == Operation::Jmp {
                debug!("Swapping instruction at {}", i);
                instruction.operation = Operation::Nop;
                return i + 1;
            } else if instruction.operation == Operation::Nop {
                debug!("Swapping instruction at {}", i);
                instruction.operation = Operation::Jmp;
                return i + 1;
            }
        }

        self.instructions.len()
    }
}

struct Execution {
    program: Program,
    instruction_counter: i32,
    accumulator: i32,
    instruction_counts: HashMap<i32, i32>,
    looping: bool
}

impl Execution {
    pub fn new(program: Program) -> Self {
        Execution {
            program,
            instruction_counter: 0,
            accumulator: 0,
            instruction_counts: HashMap::new(),
            looping: false
        }
    }

    pub fn run(&mut self) {
        let mut counter = self.instruction_counter;

        while counter < self.program.len() as i32 && !self.looping {
            self.step();
            counter = self.instruction_counter;
        }
    }

    pub fn step(&mut self) {
        let counter = self.instruction_counter;
        let instruction = self.program.instructions[counter as usize];

        self.execute(instruction);
        let mut entry = self.instruction_counts.entry(counter).or_insert(0);
        *entry += 1;

        if self.instruction_counter == counter {
            debug!("Detecting single-instruction loop at {}", counter);
            self.looping = true;
        } else if *entry >= 1_000 {
            debug!("Assuming loop due to high count at {}", counter);
            self.looping = true;
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.operation {
            Operation::Acc => {
                self.accumulator += instruction.argument;
                self.instruction_counter += 1;
            }
            Operation::Jmp => {
                self.instruction_counter += instruction.argument;
            }
            Operation::Nop => {
                self.instruction_counter += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn instruction_from_str() {
        let acc = Instruction::from("acc 1");
        assert_eq!(Operation::Acc, acc.operation);
        assert_eq!(1, acc.argument);

        let jmp = Instruction::from("jmp -1");
        assert_eq!(Operation::Jmp, jmp.operation);
        assert_eq!(-1, jmp.argument);

        let nop = Instruction::from("nop 0");
        assert_eq!(Operation::Nop, nop.operation);
        assert_eq!(0, nop.argument);
    }

    #[test]
    pub fn program_from_str() {
        let lines = vec![
            "acc 1",
            "nop 0"
        ];

        let program = Program::from(&lines);

        assert_eq!(2, program.instructions.len());
    }

    fn get_sample() -> Vec<String> {
        vec![
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(5, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(8, run_b(&input));
    }
}