use permutohedron::Heap;
use intcode_computer::IntCodeComputer;

pub fn run_a(input: &Vec<String>) -> i64 {
    let num_engines = 5;
    let mut phase_settings = vec![0,1,2,3,4];

    let mut max_signal = std::i64::MIN;
    let mut best_permutation = vec![];

    let permutations = Heap::new(&mut phase_settings);

    for permutation in permutations {
        let mut engine_input = 0;

        for i in 0..num_engines {
            let setting = permutation[i];
            let mut computer = IntCodeComputer::new(vec![]);

            let signal = computer.run(input, vec![setting, engine_input]);

            engine_input = signal[0];
        }

        if engine_input > max_signal {
            max_signal = engine_input;
            best_permutation = permutation;
        }
    }

    info!("Best signal was {} with {:?}", max_signal, best_permutation);

    max_signal
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let num_engines = 5;
    let mut phase_settings = vec![5,6,7,8,9];

    let mut max_signal = std::i64::MIN;
    let mut best_permutation = vec![];

    let permutations = Heap::new(&mut phase_settings);

    for permutation in permutations {
        let mut programs = vec![];

        for i in 0..num_engines {
            let setting = permutation[i];
            let mut computer = IntCodeComputer::new(vec![]);
            let mut program = computer.start(input);
            program.run();
            program.input(setting);
            programs.push(program);
        }

        let mut engine_input = 0;
        let mut last_output = std::i64::MIN;
        let mut halted = false;

        while !halted {
            for i in 0..num_engines {
                debug!("Perm {:?} - Providing {} to engine {}", permutation,  engine_input, i);
                let mut program = &mut programs[i];

                program.input(engine_input);

                if program.halted() {
                    halted = true;
                }

                engine_input = program.latest_output().unwrap();
            }
        }

        let compare = programs[num_engines - 1].latest_output().unwrap();

        if compare > max_signal {
            max_signal = compare;
            best_permutation = permutation;
        }
    }

    info!("Best signal was {} with {:?}", max_signal, best_permutation);

    max_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")];

        assert_eq!(43210, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![String::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")];

        assert_eq!(54321, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")];

        assert_eq!(65210, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let input = vec![String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")];

        assert_eq!(139629729, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")];

        assert_eq!(18216, run_b(&input));
    }
}