use intcode_computer::{IntCodeComputer, IntCodeProgram};

pub fn run_a(input: &Vec<String>) -> i64 {
    let computer = IntCodeComputer::new(vec![]);
    let mut program = computer.start(input);

    let command = vec![
        chars_to_i64("NOT C T\n"),
        chars_to_i64("AND D T\n"),
        chars_to_i64("OR T J\n"),
        chars_to_i64("NOT A T\n"),
        chars_to_i64("OR T J\n"),
        chars_to_i64("WALK\n")
    ];

    run(program, command)
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let computer = IntCodeComputer::new(vec![]);
    let mut program = computer.start(input);

    let command = vec![
        chars_to_i64("NOT H T\n"),
        chars_to_i64("OR C T\n"),
        chars_to_i64("AND A T\n"),
        chars_to_i64("AND B T\n"),
        chars_to_i64("NOT T T\n"),
        chars_to_i64("AND D T\n"),
        chars_to_i64("OR T J\n"),
        chars_to_i64("RUN\n")
    ];

    run(program, command)
}

fn run(mut program: IntCodeProgram, instructions: Vec<Vec<i64>>) -> i64 {
    program.run();

    for input in instructions {
        for i in input {
            program.input(i);
        }
    }

    let outputs = program.get_outputs();

    let mut chars = vec![];
    let mut output = 0;

    for out in outputs {
        if out > 255 {
            output = out;
        } else {
            chars.push((out as u8) as char);
        }
    }

    let rendered: String = chars.into_iter().collect();

    info!("\n{}\n", rendered);

    output
}

fn chars_to_i64(input: &str) -> Vec<i64> {
    let mut out = vec![];

    for c in input.chars() {
        out.push(c as i64);
    }

    out
}