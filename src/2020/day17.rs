use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::Hash;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut dimension: PocketDimension<Point3D> = PocketDimension::from(input);

    for _ in 0..6 {
        dimension = dimension.step();
    }

    dimension.num_active() as i64
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut dimension: PocketDimension<Point4D> = PocketDimension::from(input);

    for _ in 0..6 {
        dimension = dimension.step();
    }

    dimension.num_active() as i64
}

trait Point {
    fn new(x: i32, y: i32) -> Self;
    fn min<'a, I: IntoIterator<Item=&'a Self>>(iter: I) -> Self
        where Self: 'a;
    fn max<'a, I: IntoIterator<Item=&'a Self>>(iter: I) -> Self
        where Self: 'a;
    fn neighbors(&self) -> Vec<Self>
        where Self: Sized;
    fn bound_by(min: &Self, max: &Self, buffer: i32) -> Vec<Self>
        where Self: Sized;
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point for Point3D {
    fn new(x: i32, y: i32) -> Self {
        Point3D {
            x,
            y,
            z: 0,
        }
    }

    fn min<'a, I: IntoIterator<Item=&'a Self>>(iter: I) -> Self {
        let mut min_x = i32::max_value();
        let mut min_y = i32::max_value();
        let mut min_z = i32::max_value();

        for p in iter {
            min_x = min(min_x, p.x);
            min_y = min(min_y, p.y);
            min_z = min(min_z, p.z);
        }

        Point3D {
            x: min_x,
            y: min_y,
            z: min_z,
        }
    }

    fn max<'a, I: IntoIterator<Item=&'a Self>>(iter: I) -> Self {
        let mut max_x = i32::min_value();
        let mut max_y = i32::min_value();
        let mut max_z = i32::min_value();

        for p in iter {
            max_x = max(max_x, p.x);
            max_y = max(max_y, p.y);
            max_z = max(max_z, p.z);
        }

        Point3D {
            x: max_x,
            y: max_y,
            z: max_z,
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = vec![];
        let x = self.x;
        let y = self.y;
        let z = self.z;

        for nx in x - 1..=x + 1 {
            for ny in y - 1..=y + 1 {
                for nz in z - 1..=z + 1 {
                    if nx == x && ny == y && nz == z {
                        continue;
                    }
                    neighbors.push(Point3D::new(nx, ny, nz));
                }
            }
        }

        neighbors
    }

    fn bound_by(min: &Self, max: &Self, buffer: i32) -> Vec<Self> {
        let mut points = vec![];

        for x in (min.x - buffer)..=(max.x + buffer) {
            for y in (min.y - buffer)..=(max.y + buffer) {
                for z in (min.z - buffer)..=(max.z + buffer) {
                    points.push(Point3D::new(x, y, z));
                }
            }
        }

        points
    }
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point4D {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Point4D {
        Point4D { x, y, z, w }
    }
}

impl Point for Point4D {
    fn new(x: i32, y: i32) -> Self {
        Point4D {
            x,
            y,
            z: 0,
            w: 0
        }
    }

    fn min<'a, I: IntoIterator<Item=&'a Self>>(iter: I) -> Self {
        let mut min_x = i32::max_value();
        let mut min_y = i32::max_value();
        let mut min_z = i32::max_value();
        let mut min_w = i32::max_value();

        for p in iter {
            min_x = min(min_x, p.x);
            min_y = min(min_y, p.y);
            min_z = min(min_z, p.z);
            min_w = min(min_w, p.w);
        }

        Point4D {
            x: min_x,
            y: min_y,
            z: min_z,
            w: min_w
        }
    }

    fn max<'a, I: IntoIterator<Item=&'a Self>>(iter: I) -> Self {
        let mut max_x = i32::min_value();
        let mut max_y = i32::min_value();
        let mut max_z = i32::min_value();
        let mut max_w = i32::min_value();

        for p in iter {
            max_x = max(max_x, p.x);
            max_y = max(max_y, p.y);
            max_z = max(max_z, p.z);
            max_w = max(max_w, p.w);
        }

        Point4D {
            x: max_x,
            y: max_y,
            z: max_z,
            w: max_w
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = vec![];
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;

        for nx in x - 1..=x + 1 {
            for ny in y - 1..=y + 1 {
                for nz in z - 1..=z + 1 {
                    for nw in w-1..=w+1 {
                        if nx == x && ny == y && nz == z && nw == w{
                            continue;
                        }
                        neighbors.push(Point4D::new(nx, ny, nz, nw));
                    }
                }
            }
        }

        neighbors
    }

    fn bound_by(min: &Self, max: &Self, buffer: i32) -> Vec<Self> {
        let mut points = vec![];

        for x in (min.x - buffer)..=(max.x + buffer) {
            for y in (min.y - buffer)..=(max.y + buffer) {
                for z in (min.z - buffer)..=(max.z + buffer) {
                    for w in (min.w - buffer)..=(max.w + buffer) {
                        points.push(Point4D::new(x, y, z, w));
                    }
                }
            }
        }

        points
    }
}

#[derive(Debug)]
struct PocketDimension<P> {
    active_cubes: HashSet<P>,
    minimum: P,
    maximum: P,
}

impl<'a, I, J, P> From<I> for PocketDimension<P>
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized,
        P: Point + Eq + Hash + PartialEq
{
    fn from(input: I) -> Self {
        let mut active_cubes = HashSet::new();

        for (y, line) in input.into_iter().enumerate() {
            for (x, c) in line.as_ref().chars().into_iter().enumerate() {
                if c == '#' {
                    let p: P = P::new(x as i32, y as i32);
                    active_cubes.insert(p);
                }
            }
        }

        let minimum = P::min(&active_cubes);
        let maximum = P::max(&active_cubes);

        PocketDimension {
            active_cubes,
            minimum,
            maximum,
        }
    }
}

impl<P> PocketDimension<P>
    where P: Point + Eq + Hash + PartialEq
{
    pub fn num_active(&self) -> usize {
        self.active_cubes.len()
    }

    pub fn step(&self) -> PocketDimension<P> {
        let mut active_cubes = HashSet::new();
        let min = &self.minimum;
        let max = &self.maximum;

        let points = P::bound_by(min, max, 1);

        for p in points {
            let currently_active = self.active_cubes.contains(&p);
            let neighbors = p.neighbors();
            let active_neighbors = neighbors.iter()
                .filter(|n| self.active_cubes.contains(*n))
                .count();

            let new_active = if currently_active {
                active_neighbors == 2 || active_neighbors == 3
            } else {
                active_neighbors == 3
            };

            if new_active {
                active_cubes.insert(p);
            }
        }

        let minimum = P::min(&active_cubes);

        let maximum = P::max(&active_cubes);

        PocketDimension {
            active_cubes,
            minimum,
            maximum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            ".#.",
            "..#",
            "###",
        ]
    }

    #[test]
    pub fn dimension_from_sample() {
        let sample = get_sample();
        let dim: PocketDimension<Point3D> = PocketDimension::from(&sample);

        assert_eq!(5, dim.num_active());
    }

    #[test]
    pub fn dimension_step_sample() {
        let sample = get_sample();
        let dim: PocketDimension<Point3D> = PocketDimension::from(&sample);

        let new_dim = dim.step();
        assert_eq!(11, new_dim.num_active());
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(112, run_a(&sample));
    }

    #[test]
    #[ignore]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(848, run_b(&sample));
    }
}