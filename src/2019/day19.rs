use std::collections::{HashMap, HashSet};

use log::Level::Trace;

use intcode_computer::IntCodeComputer;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

struct TractorBeam {
    affected_points: HashSet<Point>,
    width: i64,
    height: i64,
}

impl TractorBeam {
    fn new_sized(instructions: &Vec<String>, size: Point) -> TractorBeam {
        let mut affected = HashSet::new();
        let computer = IntCodeComputer::new(vec![]);

        for y in 0..size.y {
            for x in 0..size.x {
                let point = Point { x, y };
                let result = computer.run(instructions, vec![x, y]);

                if result[0] == 1 {
                    affected.insert(point);
                }
            }
        }

        TractorBeam {
            affected_points: affected,
            width: size.x,
            height: size.y,
        }
    }

    fn for_size(instructions: &Vec<String>, size: Point) -> Point {
        let mut y = size.y - 1;
        let mut x = 0;

        let computer = IntCodeComputer::new(vec![]);

        //Basic pattern is find first point on x access for every y, then check the point opposite that
        loop {
            let result = computer.run(instructions, vec![x, y]);

            if result[0] == 1 {
                let test_x = x + size.x - 1;
                let test_y = y - size.y + 1;
                let result = computer.run(instructions, vec![test_x, test_y]);

                if result [0] == 1 {
                    y = test_y;
                    break;
                } else {
                    y += 1;
                }
            } else {
                x += 1;
            }
        }

        Point { x, y }
    }

    pub fn render(&self) -> String {
        let mut chars = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };
                chars.push(
                    match self.affected_points.contains(&point) {
                        true => '#',
                        false => '.'
                    }
                );
            }
            chars.push('\n');
        }

        chars.into_iter().collect()
    }
}

pub fn run_a(input: &Vec<String>) -> usize {
    let beam = TractorBeam::new_sized(input, Point { x: 50, y: 50 });

    //info!("\n{}", beam.render());

    beam.affected_points.len()
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let result = TractorBeam::for_size(input, Point { x: 100, y: 100});

    result.x * 10_000 + result.y
}