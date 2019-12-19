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
extern crate petgraph;

use std::time::Instant;

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
mod day16;
mod day17;
mod day18;
mod day19;
mod day21;

mod advent_helper;
mod intcode_computer;

fn main() {
    env_logger::init();

    let start = Instant::now();

    let days_to_run = vec![18];

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

    if days_to_run.contains(&13) {
        info!("Starting day13");

        let input = advent_helper::read_file_lines("resources/2019/day13.txt");
        info!("Total of {} block tiles", day13::run_a(&input));
        info!("Final score: {}", day13::run_b(&input));
    }

    if days_to_run.contains(&14) {
        info!("Starting day14");

        let input = advent_helper::read_file_lines("resources/2019/day14.txt");
        info!("Total of {} ore required for a single fuel", day14::run_a(&input));
        info!("Total of {} fuel for one trillion ore", day14::run_b(&input));
    }

    if days_to_run.contains(&16) {
        info!("Starting day16");

        let input = advent_helper::read_file_lines("resources/2019/day16.txt");
        let output = day16::run_a(&input);
        info!("First 8 digits are {:?}", output);
        let output = day16::run_b(&input);
        info!("Real message is  {:?}", output);
    }

    if days_to_run.contains(&17) {
        info!("Starting day17");

        let input = advent_helper::read_file_lines("resources/2019/day17.txt");
        info!("Alignment parameter sum: {}", day17::run_a(&input));
        info!("{} dust collected", day17::run_b(&input));
    }

    if days_to_run.contains(&18) {
        info!("Starting day18");

        let input = advent_helper::read_file_lines("resources/2019/day18.txt");
        info!("Shortest path is {}", day18::run_a(&input));
    }

    if days_to_run.contains(&19) {
        info!("Starting day19");

        let input = advent_helper::read_file_lines("resources/2019/day19.txt");
        info!("Number of affected points: {}", day19::run_a(&input));
        info!("Closest point: {}", day19::run_b(&input));
    }

    if days_to_run.contains(&21) {
        info!("Starting day21");

        let input = advent_helper::read_file_lines("resources/2019/day21.txt");
        info!("Hull damage: {}", day21::run_a(&input));
    }

    let elapsed = start.elapsed();

    info!("{:?} millis elapsed", elapsed.as_millis());
}
