use std::collections::HashMap;

use regex::Regex;
use std::collections::HashSet;

#[derive(Hash,Eq,PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

struct Light {
    position: Point,
    velocity: Point,
}

impl Light {
    pub fn position_at_frame(&self, frame: i64) -> Point {
        Point {
            x: self.position.x + (self.velocity.x * frame),
            y: self.position.y + (self.velocity.y * frame),
        }
    }
}

#[derive(Copy, Clone)]
struct BoundingBox {
    x: i64,
    y: i64,
    width: i64,
    height: i64,
}

impl BoundingBox {
    pub fn area(&self) -> i64 {
        self.width * self.height
    }
}

pub fn run_a(input: &Vec<String>) -> String {
    let mut lights = parse_input(input);

    let mut smallest_box = bounding_box(&lights, 0);
    let mut smallest_box_frame = 0;

    let mut frame = 1;
    let frame_increase_limit = 10;
    let mut num_frames_increasing = 0;

    loop {
        let bounding_box = bounding_box(&lights, frame);

        if bounding_box.area() < smallest_box.area() {
            smallest_box = bounding_box;
            smallest_box_frame = frame;
            num_frames_increasing = 0;
        } else {
            num_frames_increasing += 1;
        }

        if num_frames_increasing > frame_increase_limit {
            break;
        }

        frame += 1;
    }

    println!("Smallest box occurred at frame {} with area {}", smallest_box_frame, smallest_box.area());

    let result = draw_lights(&lights, smallest_box_frame, smallest_box);

    println!("{}", result);

    result
}

fn parse_input(input: &Vec<String>) -> Vec<Light> {
    let mut output = Vec::new();

    let pattern = Regex::new("position=< *(-?\\d+), *(-?\\d+)> velocity=< *(-?\\d+), *(-?\\d+)>").unwrap();

    for line in input {
        if !pattern.is_match(line) {
            panic!("Line {} did not match", line);
        }

        let captures = pattern.captures(line).unwrap();

        let pos_x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let pos_y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vel_x = captures.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let vel_y = captures.get(4).unwrap().as_str().parse::<i64>().unwrap();

        let light = Light {
            position: Point { x: pos_x, y: pos_y },
            velocity: Point { x: vel_x, y: vel_y },
        };

        output.push(light);
    }

    output
}

fn bounding_box(lights: &Vec<Light>, frame: i64) -> BoundingBox {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;

    for light in lights {
        let pos = light.position_at_frame(frame);
        let pos_x = pos.x;
        let pos_y = pos.y;

        min_x = std::cmp::min(min_x, pos_x);
        max_x = std::cmp::max(max_x, pos_x);
        min_y = std::cmp::min(min_y, pos_y);
        max_y = std::cmp::max(max_y, pos_y);
    }

    BoundingBox {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
    }
}

fn draw_lights(lights: &Vec<Light>, frame: i64, bounding_box: BoundingBox) -> String {
    let mut output = String::new();
    let mut light_points: HashSet<Point> = HashSet::new();

    for light in lights {
        let point = light.position_at_frame(frame);
        light_points.insert(point);
    }

    for y_offset in 0..bounding_box.height + 1 {
        for x_offset in 0..bounding_box.width + 1 {
            let y = bounding_box.y + y_offset;
            let x = bounding_box.x +  x_offset;
            let point = Point { x, y };
            let c = if light_points.contains(&point) {
                '#'
            } else {
                '.'
            };
            output.push(c);
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_a() {
        let input = vec![
            String::from("position=< 9,  1> velocity=< 0,  2>"),
            String::from("position=< 7,  0> velocity=<-1,  0>"),
            String::from("position=< 3, -2> velocity=<-1,  1>"),
            String::from("position=< 6, 10> velocity=<-2, -1>"),
            String::from("position=< 2, -4> velocity=< 2,  2>"),
            String::from("position=<-6, 10> velocity=< 2, -2>"),
            String::from("position=< 1,  8> velocity=< 1, -1>"),
            String::from("position=< 1,  7> velocity=< 1,  0>"),
            String::from("position=<-3, 11> velocity=< 1, -2>"),
            String::from("position=< 7,  6> velocity=<-1, -1>"),
            String::from("position=<-2,  3> velocity=< 1,  0>"),
            String::from("position=<-4,  3> velocity=< 2,  0>"),
            String::from("position=<10, -3> velocity=<-1,  1>"),
            String::from("position=< 5, 11> velocity=< 1, -2>"),
            String::from("position=< 4,  7> velocity=< 0, -1>"),
            String::from("position=< 8, -2> velocity=< 0,  1>"),
            String::from("position=<15,  0> velocity=<-2,  0>"),
            String::from("position=< 1,  6> velocity=< 1,  0>"),
            String::from("position=< 8,  9> velocity=< 0, -1>"),
            String::from("position=< 3,  3> velocity=<-1,  1>"),
            String::from("position=< 0,  5> velocity=< 0, -1>"),
            String::from("position=<-2,  2> velocity=< 2,  0>"),
            String::from("position=< 5, -2> velocity=< 1,  2>"),
            String::from("position=< 1,  4> velocity=< 2,  1>"),
            String::from("position=<-2,  7> velocity=< 2, -2>"),
            String::from("position=< 3,  6> velocity=<-1, -1>"),
            String::from("position=< 5,  0> velocity=< 1,  0>"),
            String::from("position=<-6,  0> velocity=< 2,  0>"),
            String::from("position=< 5,  9> velocity=< 1, -2>"),
            String::from("position=<14,  7> velocity=<-2,  0>"),
            String::from("position=<-3,  6> velocity=< 2, -1>")
        ];

        let expected = String::from(
            "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
"
        );

        assert_eq!(expected, run_a(&input));
    }
}