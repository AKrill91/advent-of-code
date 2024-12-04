mod day04;
mod utils;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let day04_input = std::fs::read_to_string("resources/2024/day04.txt").unwrap();

    let day04_a = day04::run_a(&day04_input);
    info!("Day 02 part a = {}", day04_a);
    let day04_b = day04::run_b(&day04_input);
    info!("Day 02 part b = {}", day04_b);

    info!("Overall - took {:?}", start.elapsed());
}