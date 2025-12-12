mod utils;
pub mod day11;

use std::time::Instant;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let start = Instant::now();
    let input = std::fs::read_to_string("resources/2025/day11.txt").unwrap();

    let a_start = Instant::now();
    let a = day11::run_a(&input).await;
    info!("Part a = {} , took {:?}", a, a_start.elapsed());

    let b_start = Instant::now();
    let b = day11::run_b(&input).await;
    info!("Part b = {} , took {:?}", b, b_start.elapsed());

    info!("Overall - took {:?}", start.elapsed());
}