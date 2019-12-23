use intcode_computer::IntCodeComputer;

pub fn run_a(input: &Vec<String>) -> i64 {
    let computer = IntCodeComputer::new(vec![]);
    let mut program = computer.start(input);

    program.run();

    let command = vec![
        chars_to_i64("NOT C T\n"),
        chars_to_i64("AND D T\n"),
        chars_to_i64("OR T J\n"),
        chars_to_i64("NOT A T\n"),
        chars_to_i64("OR T J\n"),
        chars_to_i64("WALK\n")
    ];

    for input in command {
        for i in input {
            program.input(i);
        }
    }

    let outputs = program.get_outputs();

    info!("Outputs: {:?}", outputs);

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

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}

fn chars_to_i64(input: &str) -> Vec<i64> {
    let mut out = vec![];

    for c in input.chars() {
        out.push(c as i64);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}