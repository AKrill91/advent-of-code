mod day06;
mod utils;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();
    let day06_input = std::fs::read_to_string("resources/2024/day06.txt").unwrap();

    let a_start = Instant::now();
    let day06_a = day06::run_a(&day06_input);
    info!("Day 06 part a = {}, took {:?}", day06_a, a_start.elapsed());

    let b_start = Instant::now();
    let day06_b = day06::run_b(&day06_input);
    info!("Day 06 part b = {}, took {:?}", day06_b, b_start.elapsed());

    info!("Overall - took {:?}", start.elapsed());
}