mod utils;
pub mod day05;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();
    let input = std::fs::read_to_string("resources/2025/day05.txt").unwrap();

    let a_start = Instant::now();
    let a = day05::run_a(&input);
    info!("Part a = {} , took {:?}", a, a_start.elapsed());

    let b_start = Instant::now();
    let b = day05::run_b(&input);
    info!("Part b = {} , took {:?}", b, b_start.elapsed());

    info!("Overall - took {:?}", start.elapsed());
}