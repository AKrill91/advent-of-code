mod day01;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let day01_input = std::fs::read_to_string("resources/2024/day01.txt").unwrap();

    let day01_a = day01::run_a(&day01_input);
    info!("Day 01 part a = {}", day01_a);
    let day01_b = day01::run_b(&day01_input);
    info!("Day 01 part b = {}", day01_b);

    info!("Overall - took {:?}", start.elapsed());
}