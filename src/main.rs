extern crate chrono;
extern crate core;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate regex;

use std::time::Instant;

mod advent_helper;
mod year2021;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let years_to_run = vec![2021];
    info!("Running years: {:?}", years_to_run);

    if years_to_run.contains(&2021) {
        let year_start = Instant::now();

        year2021::run();

        info!("Year 2021 took {:?}", year_start.elapsed());
    }

    info!("{:?} elapsed", start.elapsed());
}
