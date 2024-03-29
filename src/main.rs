extern crate chrono;
extern crate core;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::collections::HashMap;
use std::time::Instant;

mod advent_helper;
mod year2021;
mod common;

pub type YearRun = fn(i32, &[i32]) -> ();
pub type DayRun = fn(i32, &[String]) -> String;

fn year_run_unknown(year: i32, _: &[i32]) {
    log::warn!("Unknown year {:04}", year);
}

pub fn day_run_unknown(day: i32, _: &[String]) -> String {
    format!("unknown day {:02}", day)
}

fn main() {
    env_logger::init();

    let start = Instant::now();

    let run_these: HashMap<i32, Vec<i32>> = vec![
        (2021, vec![10])
    ]
        .into_iter()
        .collect();

    run_these.iter()
        .for_each(|(year, days)| run_year(*year, days));

    info!("Overall - took {:?}", start.elapsed());
}

fn run_year(year: i32, days: &[i32]) {
    let func: YearRun = match year {
        2021 => year2021::run,
        _ => year_run_unknown
    };

    info!("{:04} - Starting", year);
    let year_start = Instant::now();

    func(year, days);

    info!("{:04} - Took {:?}", year, year_start.elapsed());
}