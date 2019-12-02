extern crate chrono;
extern crate core;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate regex;

mod day01;
mod advent_helper;

fn main() {
    env_logger::init();

    if true {
        info!("Starting day01");
        let input = advent_helper::read_file_lines("resources/2019/day01.txt");

        info!("Require {} fuel, without fuel weight", day01::run_a(&input));
        info!("Require {} fuel, with fuel weight", day01::run_b(&input));

    }
}