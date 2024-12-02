mod day01;
mod day02;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let day02_input = std::fs::read_to_string("resources/2024/day02.txt").unwrap();

    let day02_a = day02::run_a(&day02_input);
    info!("Day 02 part a = {}", day02_a);
    // let day02_b = day01::run_b(&day02_input);
    // info!("Day 02 part b = {}", day02_b);

    info!("Overall - took {:?}", start.elapsed());
}