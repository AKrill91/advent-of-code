mod day03;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let day03_input = std::fs::read_to_string("resources/2024/day03.txt").unwrap();

    let day03_a = day03::run_a(&day03_input);
    info!("Day 02 part a = {}", day03_a);
    let day03_b = day03::run_b(&day03_input);
    info!("Day 02 part b = {}", day03_b);

    info!("Overall - took {:?}", start.elapsed());
}