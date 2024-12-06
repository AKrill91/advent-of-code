use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use crate::utils::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

#[derive(Clone)]
struct Guard {
    position: Point<i32>,
    direction: Direction
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
}

impl Tile {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => Tile::Guard,
            _ => unimplemented!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Obstacle => '#',
            Tile::Guard => 'g'
        };

        std::fmt::Display::fmt(&c, f)
    }
}

fn parse(input: &str) -> (Grid<Tile>, Guard) {
    let input = input.trim();
    let mut grid = Grid::new(
        input.lines()
            .map(|line|{
                line.chars().map(Tile::parse)
                    .collect()
            })
            .collect()
    );

    let guard_tile = grid.find(Tile::Guard)[0];
    let guard_tile = Point {
        x: guard_tile.x as i32,
        y: guard_tile.y as i32
    };

    let guard = Guard {
        position: guard_tile,
        direction: Direction::North,
    };

    grid.replace::<Tile>(Tile::Guard, Tile::Empty);

    (grid, guard)
}

pub fn run_a(input: &str) -> i64 {
    let (grid, mut guard) = parse(input);
    let mut visited_tiles = HashSet::new();

    while grid.is_in_grid(&guard.position) {
        visited_tiles.insert(guard.position);
        let desired_position = guard.position + Point::from(guard.direction);
        let tile = grid.get(&desired_position);

        if matches!(tile, Some(Tile::Obstacle)) {
            guard.direction = guard.direction.clockwise_90();
        }

        guard.position += Point::from(guard.direction);

    }

    visited_tiles.len() as i64
}

pub fn run_b(input: &str) -> i64 {
    let (grid, guard_start) = parse(input);
    let mut guard = guard_start.clone();

    let mut visited_tiles = HashSet::new();

    while grid.is_in_grid(&guard.position) {
        visited_tiles.insert(guard.position);
        let desired_position = guard.position + Point::from(guard.direction);
        let tile = grid.get(&desired_position);

        if matches!(tile, Some(Tile::Obstacle)) {
            guard.direction = guard.direction.clockwise_90();
        }

        guard.position += Point::from(guard.direction);
    }

    log::debug!("Visited {} tiles", visited_tiles.len());

    let exhaustive_tiles_that_cause_loops = find_loops(grid.clone(), guard_start.clone(), grid.find(Tile::Empty).into_iter().map(|p| Point { x: p.x as i32, y: p.y as i32}));

    let tiles_that_cause_loops = find_loops(grid.clone(), guard_start.clone(), visited_tiles.into_iter());

    log::info!("Exhaustive: {}, non-exhaustive: {}", exhaustive_tiles_that_cause_loops.len(), tiles_that_cause_loops.len());

    tiles_that_cause_loops.len() as i64

}

fn find_loops<T: Iterator<Item = Point<i32>>>(grid: Grid<Tile>, guard: Guard, tiles: T) -> HashSet<Point<i32>> {
    let mut cause_loops = HashSet::new();

    for tile in tiles {
        let mut grid_clone = grid.clone();
        grid_clone.set(Point {
            x: tile.x as usize,
            y: tile.y as usize,
        }, Tile::Obstacle);

        if loops(grid_clone, guard.clone()) {
            cause_loops.insert(tile);
        }

    }

    cause_loops
}

fn loops(grid: Grid<Tile>, mut guard: Guard) -> bool {
    let mut collisions: HashSet<(Point<i32>, Direction)> = HashSet::new();

    while grid.is_in_grid(&guard.position) {
        let desired_position = guard.position + Point::from(guard.direction);
        let tile = grid.get(&desired_position);

        if matches!(tile, Some(Tile::Obstacle)) {
            let key = (desired_position, guard.direction);
            if collisions.contains(&key) {
                return true;
            } else {
                collisions.insert(key);
            }

            guard.direction = guard.direction.clockwise_90();

            let desired_position = guard.position + Point::from(guard.direction);
            let tile = grid.get(&desired_position);

            if matches!(tile, Some(Tile::Obstacle)) {
                let key = (desired_position, guard.direction);
                if collisions.contains(&key) {
                    return true;
                } else {
                    collisions.insert(key);
                }

                guard.direction = guard.direction.clockwise_90();
            }
        }

        guard.position += Point::from(guard.direction);
    }

    false
}

#[cfg(test)]
mod test {
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
    }


    #[test]
    fn part_a_example() {
        init();
        assert_eq!(41, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        assert_eq!(6, super::run_b(example()));
    }
}