use std::collections::{HashMap, HashSet};
use crate::utils::point::Point;

struct Antenna {
    frequency: char,
    position: Point<i32>
}

fn parse(input: &str) -> (Point<i32>, HashMap<char, Vec<Antenna>>) {
    let input = input.trim();
    let mut width = None;
    let height = input.lines().count() as i32;

    let antennas = input.lines().enumerate()
        .flat_map(|(y, line)| {
            if width.is_none() {
                width = Some(line.len() as i32);
            }
            line.chars()
                .enumerate()
                .filter(|(_, c)| { *c  != '.'})
                .map(move |(x, c)| Antenna { frequency: c, position: Point::new(x as i32, y as i32) })
    })
        .fold(HashMap::new(), |mut acc: HashMap<char, Vec<Antenna>>, item| {
            acc.entry(item.frequency)
                .or_default()
                .push(item);
            acc
        });

    (Point::new(width.unwrap(), height), antennas)
}

fn all_antinodes(map_size: Point<i32>, antennas: &[Antenna], simple: bool) -> HashSet<Point<i32>> {
    let mut out = HashSet::new();

    for i in 0..antennas.len() {
        let first = &antennas[i];

        for j in i+1..antennas.len() {
            let second = &antennas[j];

            if simple {
                let antinodes = simple_antinodes(first, second);

                out.insert(antinodes[0]);
                out.insert(antinodes[1]);
            } else {
                let mut antinodes = harmonic_antinodes(map_size, first, second);
                antinodes.into_iter()
                    .for_each(|p| {
                        out.insert(p);
                    });
            }
        }
    }

    out
}

fn simple_antinodes(first: &Antenna, second: &Antenna) -> [Point<i32>; 2] {
    let diff = first.position - second.position;

    [
        first.position + diff,
        second.position - diff
    ]
}

fn harmonic_antinodes(map_size: Point<i32>, first: &Antenna, second: &Antenna) -> Vec<Point<i32>> {
    let mut out = vec![first.position, second.position];

    let diff = first.position - second.position;

    let mut test = first.position + diff;

    while is_in_map(&map_size, &test) {
        out.push(test);
        test += diff;
    }

    test = second.position - diff;

    while is_in_map(&map_size, &test) {
        out.push(test);
        test -= diff;
    }

    out
}

fn is_in_map(map_size: &Point<i32>, p: &Point<i32>) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < map_size.x && p.y < map_size.y
}

pub fn run_a(input: &str) -> i64 {
    let (map_size, antennas) = parse(input);

    let antinodes = antennas.iter()
        .flat_map(|(_, antennas)| {
            all_antinodes(map_size, &antennas, true)
        })
        .collect::<HashSet<Point<i32>>>();

    antinodes.iter()
        .filter(|p| is_in_map(&map_size, p))
        .count() as i64
}

pub fn run_b(input: &str) -> i64 {
    let (map_size, antennas) = parse(input);

    let antinodes = antennas.iter()
        .flat_map(|(_, antennas)| {
            all_antinodes(map_size, &antennas, false)
        })
        .collect::<HashSet<Point<i32>>>();

    antinodes.len() as i64
}

#[cfg(test)]
mod test {
    use crate::day08::Antenna;
    use crate::utils::point::Point;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
        assert_eq!(12, parsed.0.x);
        assert_eq!(12, parsed.0.y);
        assert_eq!(2, parsed.1.len());
        assert_eq!(4, parsed.1.get(&'0').unwrap().len());
        assert_eq!(3, parsed.1.get(&'A').unwrap().len());
    }

    #[test]
    fn simple_antinodes() {
        init();
        let first = Antenna { frequency: 'a', position: Point::new(4, 3) };
        let second = Antenna { frequency: 'a', position: Point::new(5, 5) };

        let antinodes = super::simple_antinodes(&first, &second);

        assert_eq!(Point::new(3, 1), antinodes[0]);
        assert_eq!(Point::new(6, 7), antinodes[1]);
    }

    #[test]
    fn harmonic_antinodes() {
        init();
        let antennas = vec![
            Antenna { frequency: 'T', position: Point::new(0, 0) },
            Antenna { frequency: 'T', position: Point::new(1, 2) },
            Antenna { frequency: 'T', position: Point::new(3, 1) },
        ];

        let antinodes = super::all_antinodes(Point::new(10, 10), &antennas, false);

        log::debug!("{:?}", antinodes);

        assert_eq!(9, antinodes.len());
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(14, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        assert_eq!(34, super::run_b(example()));
    }
}