use nalgebra::Vector3;
use regex::Regex;

#[derive(Debug)]
struct Moon {
    label: String,
    position: Vector3<i32>,
    velocity: Vector3<i32>,
}

impl Moon {
    pub fn new(label: String, x: i32, y: i32, z: i32) -> Moon {
        Moon {
            label,
            position: Vector3::new(x, y, z),
            velocity: Vector3::zeros(),
        }
    }

    pub fn apply_gravity(&mut self, right: Vector3<i32>) {
        let gravity_affect = Vector3::new(
            if self.position.x > right.x {
                -1
            } else if self.position.x == right.x {
                0
            } else {
                1
            },
            if self.position.y > right.y {
                -1
            } else if self.position.y == right.y {
                0
            } else {
                1
            },
            if self.position.z > right.z {
                -1
            } else if self.position.z == right.z {
                0
            } else {
                1
            },
        );

        self.velocity = self.velocity + gravity_affect;
    }

    pub fn step(&mut self) {
        self.position = self.position + self.velocity;
    }

    pub fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    pub fn potential_energy(&self) -> i32 {
        let mut sum = 0;
        for component in self.position.iter() {
            sum += component.abs();
        }
        sum
    }

    pub fn kinetic_energy(&self) -> i32 {
        let mut sum = 0;
        for component in self.velocity.iter() {
            sum += component.abs();
        }
        sum
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    run(&input, 1000)
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}

pub fn run(input: &Vec<String>, num_steps: u32) -> i32 {
    let mut moons = parse_input(input);

    for _ in 0..num_steps {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }

                let right = moons.get(j).unwrap().position;
                let left = moons.get_mut(i).unwrap();
                left.apply_gravity(right);
            }
        }

        for moon in moons.iter_mut() {
            moon.step();
        }
    }

    moons.into_iter()
        .map(|x| x.total_energy())
        .sum()
}

fn parse_input(input: &Vec<String>) -> Vec<Moon> {
    let moon_labels: Vec<&str> = vec!["Io", "Europa", "Ganymede", "Callisto"];
    let mut line_num = 0;
    let mut moons = vec![];

    let moon_pattern = Regex::new(r"<x=(-?\d{1,5}), y=(-?\d{1,5}), z=(-?\d{1,5})>").unwrap();

    for line in input {
        if !moon_pattern.is_match(line) {
            panic!("Line `{}` does not match expected pattern", line);
        }

        let captures = moon_pattern.captures(line).unwrap();

        moons.push(
            Moon::new(
                moon_labels[line_num].to_string(),
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            )
        );

        line_num += 1;
    }

    moons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("<x=-1, y=0, z=2>"),
            String::from("<x=2, y=-10, z=-7>"),
            String::from("<x=4, y=-8, z=8>"),
            String::from("<x=3, y=5, z=-1>"),
        ];

        assert_eq!(179, run(&input, 10));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![
            String::from("<x=-8, y=-10, z=0>"),
            String::from("<x=5, y=5, z=10>"),
            String::from("<x=2, y=-7, z=3>"),
            String::from("<x=9, y=-8, z=-3>"),
        ];

        assert_eq!(1940, run(&input, 100));
    }
}