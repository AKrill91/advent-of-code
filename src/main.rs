extern crate chrono;
extern crate core;
extern crate ego_tree;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate permutohedron;
extern crate regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

mod advent_helper;
mod intcode_computer;

fn main() {
    env_logger::init();

    if true {
        info!("Starting day01");
        let input = advent_helper::read_file_lines("resources/2019/day01.txt");

        info!("Require {} fuel, without fuel weight", day01::run_a(&input));
        info!("Require {} fuel, with fuel weight", day01::run_b(&input));

    }

    if true {
        info!("Starting day02");
        let input = advent_helper::read_file_lines("resources/2019/day02.txt");

        info!("{} was at position 0", day02::run_a(&input, true));
        let target = 19690720;
        let inputs = day02::run_b(&input, target);
        let result = 100 * inputs.0 + inputs.1;
        info!("{}, {} ({}) result in {}", inputs.0, inputs.1, result, target);
    }

    if true {
        info!("Starting day03");
        let input = advent_helper::read_file_lines("resources/2019/day03.txt");

        let distance = day03::run_a(&input);
        info!("Closest intersection (manhattan) is {} away", distance);
        let distance = day03::run_b(&input);
        info!("Closest intersection (along wire) is {} away", distance);
    }

    if true {
        info!("Starting day04");
        let input = advent_helper::read_file_lines("resources/2019/day04.txt");

        let num_valid = day04::run_a(&input);
        info!("{} passwords are valid", num_valid);
        let num_valid = day04::run_b(&input);
        info!("{} passwords are valid with more strict requirements", num_valid);
    }

    if true {
        info!("Starting day05");
        let input = advent_helper::read_file_lines("resources/2019/day05.txt");

        info!("Outputs: {:?}", day05::run_a(&input));
        info!("Outputs: {:?}", day05::run_b(&input));
    }

    if true {
        info!("Starting day06");
        let input = advent_helper::read_file_lines("resources/2019/day06.txt");

        info!("Total number of orbits: {}", day06::run_a(&input));
        info!("Number of orbital assists needed: {}", day06::run_b(&input));
    }

    if true {
        info!("Starting day07");

        let input = advent_helper::read_file_lines("resources/2019/day07.txt");

        info!("Highest signal: {}", day07::run_a(&input));
        info!("Highest signal with feedback loop: {}", day07::run_b(&input));

    }
}