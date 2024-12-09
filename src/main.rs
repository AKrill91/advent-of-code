mod day09;
mod utils;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();
    let day09_input = std::fs::read_to_string("resources/2024/day09.txt").unwrap();

    let a_start = Instant::now();
    let day09_a = day09::run_a(&day09_input);
    info!("Day 09 part a = {} , took {:?}", day09_a, a_start.elapsed());

    let b_start = Instant::now();
    let day09_b = day09::run_b(&day09_input);
    info!("Day 09 part b = {} , took {:?}", day09_b, b_start.elapsed());

    info!("Overall - took {:?}", start.elapsed());
}