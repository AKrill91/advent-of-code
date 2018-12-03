extern crate env_logger;
#[macro_use]
extern crate log;

mod day01;
mod day02;
mod day03;
mod advent_helper;

fn main() {
    env_logger::init();

    if false {
        info!("Starting day01");
        day01::run_a();
        day01::run_b();
    }

    if false {
        info!("Starting day02");
        day02::run_a();
        day02::run_b();
    }

    if true {
        info!("Starting day03");
        day03::run_a(advent_helper::read_file_lines("resources/day03.txt").expect("Error reading file"));
        day03::run_b();
    }
}