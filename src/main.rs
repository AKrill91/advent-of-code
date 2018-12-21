extern crate chrono;
extern crate core;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
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

    if false {
        info!("Starting day03");
        let input = advent_helper::read_file_lines("resources/day03.txt");

        day03::run_a(&input);
        day03::run_b(&input);
    }

    if false {
        info!("Starting day04");
        let input = advent_helper::read_file_lines("resources/day04.txt");

        day04::run_a(&input);
        day04::run_b(&input);
    }

    if false {
        info!("Starting day05");
        let input = advent_helper::read_file_bytes("resources/day05.txt");

        day05::run_a(&input);
        day05::run_b(&input);
    }

    if false {
        info!("Starting day06");
        let input = advent_helper::read_file_lines("resources/day06.txt");

        day06::run_a(&input);
        day06::run_b(&input, 10000);
    }

    if false {
        info!("Starting day07");
        let input = advent_helper::read_file_lines("resources/day07.txt");

        day07::run_a(&input);
        day07::run_b(&input, 5, 60);
    }

    if false {
        info!("Starting day08");
        let input = advent_helper::read_file_lines("resources/day08.txt");

        day08::run_a(&input);
        day08::run_b(&input);
    }

    if false {
        info!("Starting day09");
        let input = advent_helper::read_file_lines("resources/day09.txt");

        day09::run_a(&input);
        day09::run_b(&input, 100);
    }

    if false {
        info!("Starting day10");
        let input = advent_helper::read_file_lines("resources/day10.txt");

        day10::run_a(&input);
        day10::run_b(&input);
    }

    if false {
        info!("Starting day11");
        let input = 3999;

        day11::run_a(input, 300, 300);
        day11::run_b(input, 300, 300);
    }

    if true {
        info!("Starting day12");
        let input = advent_helper::read_file_lines("resources/day12.txt");

        day12::run_a(&input);
    }
}