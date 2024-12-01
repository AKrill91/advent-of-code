mod day01;

use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let day01_a = day01::run_a(std::fs::read_to_string("resources/2024/day01.txt").unwrap().as_str());

    info!("Day 01 part a = {}", day01_a);

    info!("Overall - took {:?}", start.elapsed());
}