use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Write};
use std::iter::FromIterator;
use crate::utils::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Beam,
    Empty,
    Splitter,
    Start,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Beam => f.write_char('|'),
            Tile::Empty => f.write_char('.'),
            Tile::Splitter => f.write_char('^'),
            Tile::Start => f.write_char('S'),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::Beam),
            '.' => Ok(Tile::Empty),
            '^' => Ok(Tile::Splitter),
            'S' => Ok(Tile::Start),
            _ => Err(())
        }
    }
}

fn split_count(grid: &Grid<Tile>) -> usize {
    grid.find(Tile::Splitter)
        .iter()
        .filter(|tile| {
            let above = grid.get(Point::new(tile.x as i32, (tile.y - 1) as i32));

            above.eq(&Some(&Tile::Beam))
        })
        .count()
}

fn parse(input: &str) -> Grid<Tile> {
    let tiles: Vec<Vec<Tile>> = input.trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Tile::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    Grid::new(tiles)
}
pub fn run_a(input: &str) -> i64 {
    let mut grid = parse(input);
    let width = grid.width();
    let height = grid.height();

    let mut to_analyze: HashSet<Point<usize>> = HashSet::from_iter(grid.find(Tile::Start).into_iter());

    let mut step = 0;

    loop {
        log::debug!("Step {}", step);
        let mut changed = false;

        let mut analyze = HashSet::new();
        std::mem::swap(&mut to_analyze, &mut analyze);

        for point in analyze {
            if point.y == height - 1 {
                continue;
            }

            let below = Point::new(point.x, point.y + 1);
            let below_i32 = Point::new(below.x as i32, below.y as i32);
            let tile = grid.get(below_i32);

            match tile {
                Some(&Tile::Splitter) => {
                    if point.x > 0 {
                        let left = Point::new(below.x - 1, below.y);
                        grid.set(left, Tile::Beam);
                        to_analyze.insert(left);
                        changed = true;
                    }
                    if point.x < width - 1 {
                        let right = Point::new(below.x + 1, below.y);
                        grid.set(right, Tile::Beam);
                        to_analyze.insert(right);
                        changed = true;
                    }
                },
                Some(&Tile::Empty) => {
                    grid.set(below, Tile::Beam);
                    to_analyze.insert(below);
                    changed = true;
                },
                _ => {},
            }
        }

        if !changed {
            break
        }
        step += 1;
    }

    log::debug!("\n{}", grid);

    split_count(&grid) as i64
}

pub fn run_b(input: &str) -> i64 {
    let grid = parse(input);
    let mut cache: HashMap<Point<usize>, i64> = HashMap::new();

    let start = grid.find(Tile::Start).iter().nth(0).unwrap().clone();

    number_of_timelines(&grid, &mut cache, start)
}

fn number_of_timelines(grid: &Grid<Tile>, cache: &mut HashMap<Point<usize>, i64>, point: Point<usize>) -> i64 {
    if let Some(count) = cache.get(&point) {
        log::debug!("Returning {} for point {:?} from cache", count, point);
        return *count;
    }

    let point_i32 = Point::new(point.x as i32, point.y as i32);

    let width = grid.width();
    let height = grid.height();

    let mut timelines = 0;

    let tile = grid.get(&point_i32).unwrap();

    if tile.eq(&Tile::Splitter) {
        if point.x > 0 {
            let left = Point::new(point.x - 1, point.y);
            timelines += number_of_timelines(&grid, cache, left);
        }
        if point.x < width - 1 {
            let right = Point::new(point.x + 1, point.y);
            timelines += number_of_timelines(&grid, cache, right);
        }
    } else {
        if point.y < height - 1 {
            let below = Point::new(point.x, point.y + 1);
            timelines = number_of_timelines(&grid, cache, below);
        } else {
            timelines = 1;
        }
    }

    cache.insert(point, timelines);

    timelines
}

#[cfg(test)]
mod test {
    use crate::day07::Tile;
    use crate::utils::point::Point;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
    }

// ...S.....
// .........
// ...^.....
// .........
// ..^.^....
// .........
// .^.^.^..^
// .........

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());

        assert_eq!(parsed.width(), 15);
        assert_eq!(parsed.height(), 16);
        log::debug!("{}", parsed);
        assert_eq!(parsed.get(Point::new(7, 0)), Some(&Tile::Start));
        assert_eq!(parsed.get(Point::new(7, 1)), Some(&Tile::Empty));
        assert_eq!(parsed.get(Point::new(7, 2)), Some(&Tile::Splitter));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(21, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(40, super::run_b(example()));
    }
}