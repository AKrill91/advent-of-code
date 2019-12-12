use std::collections::{HashMap, BTreeMap};
use advent_helper::gcd;
use std::cmp::{max, min};
use ordered_float::OrderedFloat;

#[derive(Eq, PartialEq, Debug)]
struct Asteroid {
    x: i32,
    y: i32,
}

impl Asteroid {
    fn can_see(&self, other: &Asteroid, all: &Vec<Asteroid>) -> bool {

        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        let gcd = if x_diff == 0 {
            if y_diff == 0 {
                1
            } else {
                y_diff
            }
        } else if y_diff == 0 {
            x_diff
        } else {
            gcd(y_diff, x_diff)
        };

        let x_reduced = x_diff / gcd.abs();
        let y_reduced = y_diff / gcd.abs();

        let mut validate_x = self.x + x_reduced;
        let mut validate_y = self.y + y_reduced;
        let mut blocked = false;

        while !blocked && ((validate_x != other.x) || (validate_y != other.y)) {
            assert!(validate_y < 100 && validate_x < 100);
            let look_for = Asteroid {
                x: validate_x,
                y: validate_y
            };

            debug!("Looking for {:?}", look_for);

            if all.contains(&look_for) {
                blocked = true;
            }

            validate_x += x_reduced;
            validate_y += y_reduced;
        }

        !blocked
    }

    fn angle_to(&self, other: &Asteroid) -> f64 {
        let x_diff = (other.x - self.x) as f64;
        let y_diff = (other.y - self.y) as f64;

        let angle = y_diff.atan2(x_diff).to_degrees();

        if angle < 0.0 {
            angle + 360.0
        } else {
            angle
        }
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let asteroids = parse_asteroids(input);

    info!("{} asteroids in field", asteroids.len());

    let (max_asteroid, max_visible) = find_best_asteroid(&asteroids);

    info!("Asteroid {:?} can see {} other asteroids", max_asteroid, max_visible);

    max_visible
}

pub fn run_b(input: &Vec<String>) -> i32 {
    let asteroids = parse_asteroids(input);

    let (max_asteroid, _) = find_best_asteroid(&asteroids);

    let mut angle_map: BTreeMap<OrderedFloat<f64>, Vec<&Asteroid>> = BTreeMap::new();

    for asteroid in &asteroids {
        let angle = max_asteroid.angle_to(asteroid);

        let ordered: OrderedFloat<f64> = OrderedFloat::from(angle);

        let entry = angle_map.entry(ordered).or_insert(vec![]);

        entry.push(asteroid);
    }

    let initial_angle = 270.0;
    let mut angle = 270.0;

    let mut trip = 0;
    let mut num_exploded = 0;
    let mut num_200 = max_asteroid;
    let mut first_trip = true;

    while num_exploded < 200 {
        let mut trip_incremented = false;
        debug!("Starting loop {}: {} exploded", trip, num_exploded);
        for (asteroid_angle, angle_asteroids) in angle_map.iter() {
            let a_angle = asteroid_angle.into_inner();
            if a_angle < angle {
                continue;
            }

            angle = a_angle;

            if angle >= initial_angle && !trip_incremented && !first_trip {
                trip += 1;
                trip_incremented = true;
            }

            if angle_asteroids.len() <= trip {
                continue;
            }

            let target = angle_asteroids[trip];

            debug!("Blowing up {:?}", target);

            num_exploded += 1;

            if num_exploded == 200 {
                num_200 = target;
            }
        }
        first_trip = false;

        angle = 0.0;
    }

    num_200.x * 100 + num_200.y
}


fn parse_asteroids(input: &Vec<String>) -> Vec<Asteroid> {
    let mut output = vec![];

    let mut y = 0;

    for line in input {
        let mut x = 0;
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            match c {
                '#' => {
                    output.push(Asteroid { x, y });
                }
                _ => {}
            }

            x += 1;
        }

        y += 1;
    }

    output
}

fn find_best_asteroid(asteroids: &Vec<Asteroid>) -> (&Asteroid, i32) {
    let mut max_asteroid = &asteroids[0];
    let mut max_visible = 0;

    for check in 0..asteroids.len() {
        let mut num_visible = 0;
        let checking = &asteroids[check];
        debug!("Checking asteroid {:?}", checking);

        for compare in 0..asteroids.len() {
            if check == compare {
                continue
            }


            let comparing = &asteroids[compare];

            if checking.can_see(comparing, asteroids) {
                debug!("Can see {:?}", comparing);
                num_visible += 1;
            }
        }

        if num_visible > max_visible {
            max_visible = num_visible;
            max_asteroid = checking;
        }
    }

    (max_asteroid, max_visible)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from(".#..#"),
            String::from("....."),
            String::from("#####"),
            String::from("....#"),
            String::from("...##")
        ];

        assert_eq!(8, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![
            String::from("......#.#."),
            String::from("#..#.#...."),
            String::from("..#######."),
            String::from(".#.#.###.."),
            String::from(".#..#....."),
            String::from("..#....#.#"),
            String::from("#..#....#."),
            String::from(".##.#..###"),
            String::from("##...#..#."),
            String::from(".#....####"),
        ];

        assert_eq!(33, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![
            String::from("#.#...#.#."),
            String::from(".###....#."),
            String::from(".#....#..."),
            String::from("##.#.#.#.#"),
            String::from("....#.#.#."),
            String::from(".##..###.#"),
            String::from("..#...##.."),
            String::from("..##....##"),
            String::from("......#..."),
            String::from(".####.###."),
        ];

        assert_eq!(35, run_a(&input));
    }

    #[test]
    pub fn sample_input_3_a() {
        let input = vec![
            String::from(".#..#..###"),
            String::from("####.###.#"),
            String::from("....###.#."),
            String::from("..###.##.#"),
            String::from("##.##.#.#."),
            String::from("....###..#"),
            String::from("..#.#..#.#"),
            String::from("#..#.#.###"),
            String::from(".##...##.#"),
            String::from(".....#.#.."),
        ];

        assert_eq!(41, run_a(&input));
    }

    #[test]
    pub fn sample_input_4_a() {
        let input = vec![
            String::from(".#..##.###...#######"),
            String::from("##.############..##."),
            String::from(".#.######.########.#"),
            String::from(".###.#######.####.#."),
            String::from("#####.##.#.##.###.##"),
            String::from("..#####..#.#########"),
            String::from("####################"),
            String::from("#.####....###.#.#.##"),
            String::from("##.#################"),
            String::from("#####.##.###..####.."),
            String::from("..######..##.#######"),
            String::from("####.##.####...##..#"),
            String::from(".#####..#.######.###"),
            String::from("##...#.##########..."),
            String::from("#.##########.#######"),
            String::from(".####.#.###.###.#.##"),
            String::from("....##.##.###..#####"),
            String::from(".#.#.###########.###"),
            String::from("#.#.#.#####.####.###"),
            String::from("###.##.####.##.#..##"),
        ];

        assert_eq!(210, run_a(&input));
    }

    #[test]
    pub fn sample_input_4_b() {
        let input = vec![
            String::from(".#..##.###...#######"),
            String::from("##.############..##."),
            String::from(".#.######.########.#"),
            String::from(".###.#######.####.#."),
            String::from("#####.##.#.##.###.##"),
            String::from("..#####..#.#########"),
            String::from("####################"),
            String::from("#.####....###.#.#.##"),
            String::from("##.#################"),
            String::from("#####.##.###..####.."),
            String::from("..######..##.#######"),
            String::from("####.##.####...##..#"),
            String::from(".#####..#.######.###"),
            String::from("##...#.##########..."),
            String::from("#.##########.#######"),
            String::from(".####.#.###.###.#.##"),
            String::from("....##.##.###..#####"),
            String::from(".#.#.###########.###"),
            String::from("#.#.#.#####.####.###"),
            String::from("###.##.####.##.#..##"),
        ];

        assert_eq!(802, run_b(&input));
    }
}