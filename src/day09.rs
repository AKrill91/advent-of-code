use std::collections::{HashMap, HashSet};
use std::io::Write;
use crate::utils::point::Point;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Green,
    Greenish,
    Red,
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Green => 'X',
            Tile::Greenish => 'x',
            Tile::Red => '#',
        }
    }
}

fn parse(input: &str) -> Vec<Point<i64>> {
    input.trim()
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            Point::new(x, y)
        })
        .collect()
}

pub fn run_a(input: &str) -> i64 {
    let points = parse(input);

    let mut max_area = i64::MIN;

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let width = (p1.x - p2.x).abs() + 1;
            let height = (p1.y - p2.y).abs() + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn run_b(input: &str) -> i64 {
    let points = parse(input);
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    let mut tiles = HashMap::new();

    for i in 0..points.len() {
        let p = points[i];

        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);

        tiles.insert(p, Tile::Red);

        let previous_index = if i == 0 {
            points.len() - 1
        } else {
            i - 1
        };

        let prev = points[previous_index];

        if p.x == prev.x {
            let range = if p.y > prev.y {
                prev.y + 1..p.y
            } else {
                p.y + 1..prev.y
            };

            for y in range {
                tiles.insert(Point::new(p.x, y), Tile::Green);
            }
        } else if p.y == prev.y {
            let range = if p.x > prev.x {
                prev.x + 1..p.x
            } else {
                p.x + 1..prev.x
            };

            for x in range {
                tiles.insert(Point::new(x, p.y), Tile::Green);
            }
        }
    }

    let width = max_x - min_x;
    let height = max_y - min_y;

    log::debug!("min x: {min_x:?}, max x: {max_x:?}, min y: {min_y:?}, max y: {max_y:?}, {} x {}", width, height);

    // Find a point inside the shape
    let in_shape = (min_y..=max_y).into_iter()
        .flat_map(|y|
            (min_x..=max_x).into_iter()
                .map(move |x| Point::new(x, y))
        )
        .filter(|p| !tiles.contains_key(p))
        .find(|p| {
            (min_y..p.y).into_iter()
                .map(|y| {
                    Point::new(p.x, y)
                })
                .filter(|p| {
                    let tile = tiles.get(p);
                    match tile {
                        None => false,
                        Some(&Tile::Empty) => false,
                        Some(_) => true,
                    }
                })
                .count() % 2 == 1
        })
        .unwrap();

    log::debug!("Point {:?} is in the shape", in_shape);

    let mut to_process = HashSet::from([in_shape]);

    let mut step = 0;

    // Flood fill
    while let Some(next) = to_process.iter().next().cloned() {
        to_process.remove(&next);
        if tiles.contains_key(&next) {
            continue;
        }

        if step % 1000 == 0 {
            log::debug!("Flood fill step {}, to process {}", step, to_process.len());
        }

        tiles.insert(next, Tile::Greenish);

        to_process.extend(
            vec![
                Point::new(next.x + 1, next.y),
                Point::new(next.x - 1, next.y),
                Point::new(next.x, next.y + 1),
                Point::new(next.x, next.y - 1),
            ]
                .into_iter()
                .filter(|p| {
                    !tiles.contains_key(p)
                })
        );
        step += 1;
    }

    log::debug!("Finished flood fill");

    let mut max_area = i64::MIN;

    // Find the biggest area
    for i in 0..points.len() - 1 {
        'nested: for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let width = (p1.x - p2.x).abs() + 1;
            let height = (p1.y - p2.y).abs() + 1;
            let area = width * height;

            if area > max_area {
                let min_x = p1.x.min(p2.x);
                let max_x = p1.x.max(p2.x);
                let min_y = p1.y.min(p2.y);
                let max_y = p1.y.max(p2.y);

                log::debug!("Checking area from {p1:?} to {p2:?}, {area}");
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        log::debug!("Checking if {:?} is in shape", Point::new(x, y));
                        let point = Point::new(x, y);
                        let tile = tiles.get(&point).unwrap_or(&Tile::Empty);

                        if tile.eq(&Tile::Empty) {
                            continue 'nested;
                        }
                    }
                }

                log::debug!("Max area from {p1:?} to {p2:?}, {area}");

                max_area = area;
            }
        }
    }

    // Draw the shape
    if width < 100 {
        for y in min_y - 1..=max_y + 1 {
            let mut row = String::new();
            for x in min_x - 1..=max_x + 1 {
                let point = Point::new(x, y);
                if point.eq(&in_shape) {
                    row.push('O');
                    continue;
                }

                let tile = tiles.get(&point).unwrap_or(&Tile::Empty);

                row.push(tile.into());
            }

            log::debug!("{:?}", row);
        }
    }

    max_area
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
        assert_eq!(parsed.len(), 8);
        assert_eq!(parsed[0], Point::new(7, 1));
        assert_eq!(parsed[1], Point::new(11, 1));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(50, run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(24, run_b(example()));
    }
}