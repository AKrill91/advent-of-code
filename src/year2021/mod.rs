use std::time::Instant;
use crate::{day_run_unknown, DayRun};

mod day01;
mod day02;
mod day04;

pub fn run(year: i32, days_to_run: &Vec<i32>) {
    days_to_run.iter()
        .for_each(|day| run_day(year, *day));
}

fn run_day(year: i32, day: i32) {
    let funcs: (DayRun, DayRun) = match day {
        1 => (day01::run_a, day01::run_b),
        2 => (day02::run_a, day02::run_b),
        4 => (day04::run_a, day04::run_b),
        _ => (day_run_unknown, day_run_unknown)
    };

    info!("{:04}:{:02} - Starting", year, day);
    let day_start = Instant::now();

    let resource_file = format!("resources/{:04}/day{:02}.txt", year, day);
    let input = crate::advent_helper::read_file_lines(&resource_file);

    for (letter, func) in vec![("a", funcs.0), ("b", funcs.1)] {
        info!("{:04}:{:02}:{} - Starting", year, day, letter);
        let letter_start = Instant::now();
        let letter_result = func(day, &input);
        log::info!("{:04}:{:02}:{} - Got '{}'", year, day, letter, letter_result);
        info!("{:04}:{:02}:{} - Took {:?}", year, day, letter, letter_start.elapsed());
    }

    info!("{:04}:{:02} - Took {:?}", year, day, day_start.elapsed());
}