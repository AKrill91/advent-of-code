use std::collections::HashMap;

use intcode_computer::IntCodeComputer;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

pub fn run_a(input: &Vec<String>) -> usize {
    let points = run(input, Point { x: 50, y: 50 });

    points.values()
        .filter(|b| **b)
        .count()
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}

fn run(input: &Vec<String>, size: Point) -> HashMap<Point, bool> {
    let mut map = HashMap::new();
    let computer = IntCodeComputer::new(vec![]);

    for y in 0..size.y {
        for x in 0..size.x {
            let point = Point { x, y };
            let result = computer.run(input, vec![x, y]);

            map.insert(
                point,
                result[0] == 1,
            );
        }
    }

    map
}