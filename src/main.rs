extern crate chrono;
extern crate core;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate regex;

mod day01;
mod day02;
mod advent_helper;

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
}