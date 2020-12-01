use std::collections::HashMap;

use day17::CameraPixel::*;
use intcode_computer::IntCodeComputer;

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn alignment_parameter(&self) -> i32 {
        self.x * self.y
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum CameraPixel {
    Scaffold,
    Open,
    RobotUp,
    RobotDown,
    RobotLeft,
    RobotRight,
    RobotTumble,
    LineFeed,
}

impl CameraPixel {
    fn from_i64(input: i64) -> CameraPixel {
        match input {
            10 => LineFeed,
            35 => Scaffold,
            46 => Open,
            60 => RobotLeft,
            62 => RobotRight,
            88 => RobotTumble,
            94 => RobotUp,
            118 => RobotDown,
            _ => panic!()
        }
    }

    fn to_char(&self) -> char {
        match self {
            Scaffold => '#',
            Open => '.',
            RobotUp => '^',
            RobotDown => 'v',
            RobotLeft => '<',
            RobotRight => '>',
            RobotTumble => 'X',
            LineFeed => '\n',
        }
    }
}

struct CameraOutput {
    pixels: HashMap<Point, CameraPixel>,
    width: i32,
    height: i32,
}

impl CameraOutput {
    pub fn parse(input: &Vec<i64>) -> CameraOutput {
        let mut pixels = HashMap::new();

        let mut x = 0;
        let mut y = 0;

        for i in input {
            let i = *i;
            let output = CameraPixel::from_i64(i);

            pixels.insert(Point { x, y }, output);

            if output == LineFeed {
                y += 1;
                x = 0;
            } else {
                x += 1;
            }
        }

        let width = pixels.keys()
            .map(|p| p.x)
            .max()
            .unwrap() + 1;

        let height = pixels.keys()
            .map(|p| p.y)
            .max()
            .unwrap();

        CameraOutput {
            pixels,
            width,
            height,
        }
    }

    pub fn render(&self) -> String {
        let mut chars = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };

                let pixel = self.pixels.get(&point).unwrap();
                chars.push(pixel.to_char());
            }
        }

        chars.into_iter().collect()
    }

    pub fn get_intersections(&self) -> Vec<Point> {
        let mut output = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };
                if self.is_intersection(&point) {
                    output.push(point);
                }
            }
        }

        output
    }

    pub fn is_intersection(&self, point: &Point) -> bool {
        self.is_scaffold(point) &&
            self.is_scaffold(&point.up()) &&
            self.is_scaffold(&point.down()) &&
            self.is_scaffold(&point.left()) &&
            self.is_scaffold(&point.right())
    }

    pub fn is_scaffold(&self, point: &Point) -> bool {
        self.pixels.get(point)
            .map_or(false, |pixel| *pixel == Scaffold)
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let computer = IntCodeComputer::new(vec![]);
    let mut program = computer.start(input);
    program.run();

    let output = CameraOutput::parse(&program.get_outputs());

    let intersections = output.get_intersections();

    info!("Found {} intersections", intersections.len());

    intersections.iter()
        .map(|p| p.alignment_parameter())
        .sum()
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let mut adjustments = HashMap::new();
    adjustments.insert(0, 2);
    let computer = IntCodeComputer::new(vec![]);
    let mut program = computer.start_with_adjustments(input, adjustments);
    program.run();

    let main_routine = chars_to_i64("A,A,B,C,A,C,A,B,C,B\n");
    let function_a = chars_to_i64("R,12,L,8,R,6\n");
    let function_b = chars_to_i64("R,12,L,6,R,6,R,8,R,6\n");
    let function_c = chars_to_i64("L,8,R,8,R,6,R,12\n");

    let inputs = vec![
        main_routine,
        function_a,
        function_b,
        function_c,
        chars_to_i64("n\n")
    ];

    for input in inputs {
        for i in input {
            program.input(i);
        }
    }

    program.latest_output().unwrap()
}

fn chars_to_i64(input: &str) -> Vec<i64> {
    let mut out = vec![];

    for c in input.chars() {
        out.push(c as i64);
    }

    out
}