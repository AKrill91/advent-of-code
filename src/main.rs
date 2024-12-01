use std::time::Instant;
use log::info;

fn main() {
    env_logger::init();

    let start = Instant::now();

    info!("Overall - took {:?}", start.elapsed());
}