mod day08;
mod utils;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();
    let day08_input = std::fs::read_to_string("resources/2024/day08.txt").unwrap();

    let a_start = Instant::now();
    let day08_a = day08::run_a(&day08_input);
    info!("Day 08 part a = {} , took {:?}", day08_a, a_start.elapsed());

    let b_start = Instant::now();
    let day08_b = day08::run_b(&day08_input);
    info!("Day 08 part b = {} , took {:?}", day08_b, b_start.elapsed());

    info!("Overall - took {:?}", start.elapsed());
}