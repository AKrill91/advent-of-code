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
mod intcode_computer;

use std::time::Instant;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let days_to_run = vec![1,2,3,4,5,6,7,8,9,10,11,12];

    info!("Running days: {:?}", days_to_run);

    if days_to_run.contains(&1) {
        info!("Starting day01");
        let input = advent_helper::read_file_lines("resources/2019/day01.txt");

        info!("Require {} fuel, without fuel weight", day01::run_a(&input));
        info!("Require {} fuel, with fuel weight", day01::run_b(&input));

    }

    if days_to_run.contains(&2) {
        info!("Starting day02");
        let input = advent_helper::read_file_lines("resources/2019/day02.txt");

        info!("{} was at position 0", day02::run_a(&input, true));
        let target = 19690720;
        let inputs = day02::run_b(&input, target);
        let result = 100 * inputs.0 + inputs.1;
        info!("{}, {} ({}) result in {}", inputs.0, inputs.1, result, target);
    }

    if days_to_run.contains(&3) {
        info!("Starting day03");
        let input = advent_helper::read_file_lines("resources/2019/day03.txt");

        let distance = day03::run_a(&input);
        info!("Closest intersection (manhattan) is {} away", distance);
        let distance = day03::run_b(&input);
        info!("Closest intersection (along wire) is {} away", distance);
    }

    if days_to_run.contains(&4) {
        info!("Starting day04");
        let input = advent_helper::read_file_lines("resources/2019/day04.txt");

        let num_valid = day04::run_a(&input);
        info!("{} passwords are valid", num_valid);
        let num_valid = day04::run_b(&input);
        info!("{} passwords are valid with more strict requirements", num_valid);
    }

    if days_to_run.contains(&5) {
        info!("Starting day05");
        let input = advent_helper::read_file_lines("resources/2019/day05.txt");

        info!("Outputs: {:?}", day05::run_a(&input));
        info!("Outputs: {:?}", day05::run_b(&input));
    }

    if days_to_run.contains(&6) {
        info!("Starting day06");
        let input = advent_helper::read_file_lines("resources/2019/day06.txt");

        info!("Total number of orbits: {}", day06::run_a(&input));
        info!("Number of orbital assists needed: {}", day06::run_b(&input));
    }

    if days_to_run.contains(&7) {
        info!("Starting day07");

        let input = advent_helper::read_file_lines("resources/2019/day07.txt");

        info!("Highest signal: {}", day07::run_a(&input));
        info!("Highest signal with feedback loop: {}", day07::run_b(&input));
    }

    if days_to_run.contains(&8) {
        info!("Starting day08");

        let input = advent_helper::read_file_lines("resources/2019/day08.txt");

        info!("Checksum: {}", day08::run_a(&input, 25, 6));
        info!("Result: \n{}", day08::run_b(&input, 25, 6));
    }

    if days_to_run.contains(&9) {
        info!("Starting day09");

        let input = advent_helper::read_file_lines("resources/2019/day09.txt");
        info!("BOOST keycode: {}", day09::run_a(&input));
        info!("Distress signal: {:?}", day09::run_b(&input));
    }

    if days_to_run.contains(&10) {
        info!("Starting day10");

        let input = advent_helper::read_file_lines("resources/2019/day10.txt");
        info!("Max asteroids: {}", day10::run_a(&input));
        info!("200th asteroid: {}", day10::run_b(&input));
    }

    if days_to_run.contains(&11) {
        info!("Starting day11");

        let input = advent_helper::read_file_lines("resources/2019/day11.txt");
        info!("Panels painted: {}", day11::run_a(&input));
        info!("Registration Id: \n{}", day11::run_b(&input));
    }

    if days_to_run.contains(&12) {
        info!("Starting day12");

        let input = advent_helper::read_file_lines("resources/2019/day12.txt");
        info!("Total energy: {}", day12::run_a(&input));
        info!("Cycle repeats every {} steps", day12::run_b(&input));
    }

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}