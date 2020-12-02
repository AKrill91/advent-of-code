extern crate chrono;
extern crate core;
extern crate ego_tree;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate num_integer;
extern crate ordered_float;
extern crate permutohedron;
extern crate regex;

use std::time::Instant;

mod day01;
mod day02;

mod advent_helper;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let days_to_run = vec![2];

    info!("Running days: {:?}", days_to_run);

    if days_to_run.contains(&1) {
        info!("Starting day01");
        let input = advent_helper::read_file_lines("resources/2020/day01.txt");

        info!("day01::run_a = {}", day01::run_a(&input));
        info!("day01::run_b = {}", day01::run_b(&input));
    }

    if days_to_run.contains(&2) {
        info!("Starting day02");
        let input = advent_helper::read_file_lines("resources/2020/day02.txt");

        info!("day02::run_a = {}", day02::run_a(&input));
        info!("day02::run_b = {}", day02::run_b(&input));
    }

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}
