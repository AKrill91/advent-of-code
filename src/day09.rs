use intcode_computer::IntCodeComputer;

pub fn run_a(input: &Vec<String>) -> i64 {
    let computer = IntCodeComputer::new(vec![]);
    let output = computer.run(&input, vec![1]);

    output[0]
}

pub fn run_b(input: &Vec<String>) -> Vec<i64> {
    let computer = IntCodeComputer::new(vec![]);
    let output = computer.run(&input, vec![2]);

    output
}

#[cfg(test)]
mod tests {
    use super::*;
}