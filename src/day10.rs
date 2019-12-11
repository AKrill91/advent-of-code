use std::collections::HashMap;
use advent_helper::gcd;
use std::cmp::max;

#[derive(Eq, PartialEq, Debug)]
struct Asteroid {
    x: i32,
    y: i32,
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let asteroids = parse_asteroids(input);

    info!("{} asteroids in field", asteroids.len());

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

            let x_diff = comparing.x - checking.x;
            let y_diff = comparing.y - checking.y;

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

            let mut validate_x = checking.x + x_reduced;
            let mut validate_y = checking.y + y_reduced;
            let mut blocked = false;

            while !blocked && ((validate_x != comparing.x) || (validate_y != comparing.y)) {
                assert!(validate_y < 100 && validate_x < 100);
                let look_for = Asteroid {
                    x: validate_x,
                    y: validate_y
                };

                debug!("Looking for {:?}", look_for);

                if asteroids.contains(&look_for) {
                    blocked = true;
                }

                validate_x += x_reduced;
                validate_y += y_reduced;
            }

            if !blocked {
                debug!("Can see {:?}", comparing);
                num_visible += 1;
            }
        }

        if num_visible > max_visible {
            max_visible = num_visible;
            max_asteroid = checking;
        }
    }

    info!("Asteroid {:?} can see {} other asteroids", max_asteroid, max_visible);

    max_visible
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
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
}