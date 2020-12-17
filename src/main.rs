extern crate chrono;
extern crate core;
extern crate ego_tree;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate num_integer;
extern crate ordered_float;
extern crate permutohedron;
extern crate regex;

use std::time::Instant;

mod advent_helper;
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
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let days_to_run = vec![16];

    info!("Running days: {:?}", days_to_run);

    if days_to_run.contains(&1) {
        info!("Starting day01");
        let input = advent_helper::read_file_lines("resources/2020/day01.txt");

        info!("day01::run_a = {}", day01::run_a(&input));
        info!("day01::run_b = {}", day01::run_b(&input));
    }

    if days_to_run.contains(&2) {
        info!("Starting day02");
        let input = advent_helper::read_file_lines("resources/2020/day02.txt");

        info!("day02::run_a = {}", day02::run_a(&input));
        info!("day02::run_b = {}", day02::run_b(&input));
    }

    if days_to_run.contains(&3) {
        info!("Starting day03");
        let input = advent_helper::read_file_lines("resources/2020/day03.txt");

        info!("day03::run_a = {}", day03::run_a(&input));
        info!("day03::run_b = {}", day03::run_b(&input));
    }

    if days_to_run.contains(&4) {
        info!("Starting day04");
        let input = advent_helper::read_file_lines("resources/2020/day04.txt");

        info!("day04::run_a = {}", day04::run_a(&input));
        info!("day04::run_b = {}", day04::run_b(&input));
    }

    if days_to_run.contains(&5) {
        info!("Starting day05");
        let input = advent_helper::read_file_lines("resources/2020/day05.txt");

        info!("day05::run_a = {}", day05::run_a(&input));
        info!("day05::run_b = {}", day05::run_b(&input));
    }

    if days_to_run.contains(&6) {
        info!("Starting day06");
        let input = advent_helper::read_file_lines("resources/2020/day06.txt");

        info!("day06::run_a = {}", day06::run_a(&input));
        info!("day06::run_b = {}", day06::run_b(&input));
    }

    if days_to_run.contains(&7) {
        info!("Starting day07");
        let input = advent_helper::read_file_lines("resources/2020/day07.txt");

        info!("day07::run_a = {}", day07::run_a(&input));
        info!("day07::run_b = {}", day07::run_b(&input));
    }

    if days_to_run.contains(&8) {
        info!("Starting day08");
        let input = advent_helper::read_file_lines("resources/2020/day08.txt");

        info!("day08::run_a = {}", day08::run_a(&input));
        info!("day08::run_b = {}", day08::run_b(&input));
    }

    if days_to_run.contains(&9) {
        info!("Starting day09");
        let input = advent_helper::read_file_lines("resources/2020/day09.txt");

        info!("day09::run_a = {}", day09::run_a(&input));
        info!("day09::run_b = {}", day09::run_b(&input));
    }

    if days_to_run.contains(&10) {
        info!("Starting day10");
        let input = advent_helper::read_file_lines("resources/2020/day10.txt");

        info!("day10::run_a = {}", day10::run_a(&input));
        info!("day10::run_b = {}", day10::run_b(&input));
    }

    if days_to_run.contains(&11) {
        info!("Starting day11");
        let input = advent_helper::read_file_lines("resources/2020/day11.txt");

        info!("day11::run_a = {}", day11::run_a(&input));
        info!("day11::run_b = {}", day11::run_b(&input));
    }

    if days_to_run.contains(&12) {
        info!("Starting day12");
        let input = advent_helper::read_file_lines("resources/2020/day12.txt");

        info!("day12::run_a = {}", day12::run_a(&input));
        info!("day12::run_b = {}", day12::run_b(&input));
    }

    if days_to_run.contains(&13) {
        info!("Starting day13");
        let input = advent_helper::read_file_lines("resources/2020/day13.txt");

        info!("day13::run_a = {}", day13::run_a(&input));
        info!("day13::run_b = {}", day13::run_b(&input));
    }

    if days_to_run.contains(&14) {
        info!("Starting day14");
        let input = advent_helper::read_file_lines("resources/2020/day14.txt");

        info!("day14::run_a = {}", day14::run_a(&input));
        info!("day14::run_b = {}", day14::run_b(&input));
    }

    if days_to_run.contains(&15) {
        info!("Starting day15");
        let input = advent_helper::read_file_lines("resources/2020/day15.txt");

        info!("day15::run_a = {}", day15::run_a(&input));
        info!("day15::run_b = {}", day15::run_b(&input));
    }

    if days_to_run.contains(&16) {
        info!("Starting day16");
        let input = advent_helper::read_file_lines("resources/2020/day16.txt");

        info!("day16::run_a = {}", day16::run_a(&input));
        info!("day16::run_b = {}", day16::run_b(&input));
    }

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}
