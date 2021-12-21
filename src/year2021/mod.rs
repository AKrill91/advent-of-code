use std::time::Instant;
use crate::{day_run_unknown, DayRun};

mod day01;
mod day02;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

pub fn run(year: i32, days_to_run: &[i32]) {
    days_to_run.iter()
        .for_each(|day| run_day(year, *day));
}

fn run_day(year: i32, day: i32) {
    let funcs: (DayRun, DayRun) = match day {
        1 => (day01::run_a, day01::run_b),
        2 => (day02::run_a, day02::run_b),
        4 => (day04::run_a, day04::run_b),
        5 => (day05::run_a, day05::run_b),
        6 => (day06::run_a, day06::run_b),
        7 => (day07::run_a, day07::run_b),
        8 => (day08::run_a, day08::run_b),
        9 => (day09::run_a, day09::run_b),
        10 => (day10::run_a, day10::run_b),
        _ => (day_run_unknown, day_run_unknown)
    };

    info!("{:04}:{:02} - Starting", year, day);
    let day_start = Instant::now();

    let resource_file = format!("resources/{:04}/day{:02}.txt", year, day);
    let input = crate::advent_helper::read_file_lines(&resource_file);

    for (letter, func) in &[("a", funcs.0), ("b", funcs.1)] {
        info!("{:04}:{:02}:{} - Starting", year, day, letter);
        let letter_start = Instant::now();
        let letter_result = func(day, &input);
        log::info!("{:04}:{:02}:{} - Got '{}'", year, day, letter, letter_result);
        info!("{:04}:{:02}:{} - Took {:?}", year, day, letter, letter_start.elapsed());
    }

    info!("{:04}:{:02} - Took {:?}", year, day, day_start.elapsed());
}