use num_integer::Integer;

use regex::Regex;

#[derive(Debug, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn add(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
        self.z += velocity.z;
    }
}

#[derive(Debug, Hash)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Velocity {
    pub fn zero() -> Velocity {
        Velocity { x: 0, y: 0, z: 0 }
    }

    pub fn kinetic_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn add(&mut self, velocity: &Velocity) {
        self.x += velocity.x;
        self.y += velocity.y;
        self.z += velocity.z;
    }

    pub fn index(&self, index: usize) -> i32 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!()
        }
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    run(&input, 1000)
}

pub fn run_b(input: &Vec<String>) -> u64 {
    let initial_positions = parse_input(input);

    let cycles: Vec<u64> = (0..3).map(|i| cycle(&initial_positions, i)).collect();
    cycles.iter().fold(1, |acc,x| acc.lcm(x))
}

fn cycle(initial_positions: &[Position], index: usize) -> u64 {
    let mut positions = initial_positions.to_vec();
    let mut velocities = vec![
        Velocity::zero(),
        Velocity::zero(),
        Velocity::zero(),
        Velocity::zero()
    ];

    let mut step = 1;

    do_step(&mut positions, &mut velocities);

    while !(0..4).all(|i| velocities[i].index(index) == 0) {
        do_step(&mut positions, &mut velocities);
        step += 1;
    }

    step * 2
}

pub fn run(input: &Vec<String>, num_steps: u64) -> i32 {
    let mut positions = parse_input(input);
    let mut velocities = vec![
        Velocity::zero(),
        Velocity::zero(),
        Velocity::zero(),
        Velocity::zero()
    ];

    for _ in 0..num_steps {
        do_step(&mut positions, &mut velocities);
    }

    (0..4).map(|i| positions[i].potential_energy() * velocities[i].kinetic_energy())
        .sum()
}

fn sign(left: i32, right: i32) -> i32 {
    if left > right {
        -1
    } else if left < right {
        1
    } else {
        0
    }
}

fn calc_gravity(positions: &[Position]) -> Vec<Velocity> {
    let mut out = vec![];

    for pos in positions.iter() {
        let mut sum_gravity = [0; 3];

        for ppos in positions.iter() {
            sum_gravity[0] += sign(pos.x, ppos.x);
            sum_gravity[1] += sign(pos.y, ppos.y);
            sum_gravity[2] += sign(pos.z, ppos.z);
        }

        out.push(Velocity { x: sum_gravity[0], y: sum_gravity[1], z: sum_gravity[2] });
    }

    out
}

fn do_step(positions: &mut [Position], velocities: &mut [Velocity]) {
    let gravity = calc_gravity(positions);
    (0..4).for_each(|i| velocities[i].add(&gravity[i]));
    (0..4).for_each(|i| positions[i].add(&velocities[i]));
}

fn parse_input(input: &Vec<String>) -> Vec<Position> {
    let mut moons = vec![];

    let moon_pattern = Regex::new(r"<x=(-?\d{1,5}), y=(-?\d{1,5}), z=(-?\d{1,5})>").unwrap();

    assert_eq!(input.len(), 4);

    for line in input {
        if !moon_pattern.is_match(line) {
            panic!("Line `{}` does not match expected pattern", line);
        }

        let captures = moon_pattern.captures(line).unwrap();

        moons.push(
            Position {
                x: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                y: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                z: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            }
        );
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

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("<x=-1, y=0, z=2>"),
            String::from("<x=2, y=-10, z=-7>"),
            String::from("<x=4, y=-8, z=8>"),
            String::from("<x=3, y=5, z=-1>"),
        ];

        assert_eq!(2772, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let input = vec![
            String::from("<x=-8, y=-10, z=0>"),
            String::from("<x=5, y=5, z=10>"),
            String::from("<x=2, y=-7, z=3>"),
            String::from("<x=9, y=-8, z=-3>"),
        ];

        assert_eq!(4686774924, run_b(&input));
    }
}