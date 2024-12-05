mod day05;
mod utils;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let day05_input = std::fs::read_to_string("resources/2024/day05.txt").unwrap();

    let day05_a = day05::run_a(&day05_input);
    info!("Day 02 part a = {}", day05_a);
    let day05_b = day05::run_b(&day05_input);
    info!("Day 02 part b = {}", day05_b);

    info!("Overall - took {:?}", start.elapsed());
}