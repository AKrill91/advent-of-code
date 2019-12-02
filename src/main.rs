extern crate chrono;
extern crate core;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate regex;

mod day01;
mod advent_helper;

fn main() {
    env_logger::init();

    if false {
        info!("Starting day01");
        day01::run_a();
        day01::run_b();
    }
}