mod day07;
mod utils;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();
    let day07_input = std::fs::read_to_string("resources/2024/day07.txt").unwrap();

    let a_start = Instant::now();
    let day07_a = day07::run_a(&day07_input);
    info!("Day 07 part a = {} , took {:?}", day07_a, a_start.elapsed());

    let b_start = Instant::now();
    let day07_b = day07::run_b(&day07_input);
    info!("Day 07 part b = {} , took {:?}", day07_b, b_start.elapsed());

    info!("Overall - took {:?}", start.elapsed());
}