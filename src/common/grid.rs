use std::collections::HashMap;
use std::hash::Hash;
use crate::common::Point2;

pub struct Grid<T, U> {
    area: T,
    height: T,
    max: Point2<T>,
    min: Point2<T>,
    points: HashMap<Point2<T>, U>,
    width: T,
}

impl<T, U> Grid<T, U> {
    pub fn points_mut(&mut self) -> &mut HashMap<Point2<T>, U> {
        &mut self.points
    }
}


impl<T, U> Grid<T, U>
    where T: Hash + Eq + num_traits::PrimInt,
          U: PartialOrd {
    pub fn new(points: HashMap<Point2<T>, U>) -> Self {
        let mut min_x = T::max_value();
        let mut min_y = T::max_value();
        let mut max_x = T::min_value();
        let mut max_y = T::min_value();

        points.keys()
            .for_each(|point| {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);

                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            });

        let max = Point2 {
            x: max_x,
            y: max_y,
        };

        let min = Point2 {
            x: min_x,
            y: min_y,
        };

        let height = max_y - min_y + T::from(1).unwrap();
        let width = max_x - min_x + T::from(1).unwrap();
        let area = width * height;

        Grid {
            area,
            height,
            max,
            min,
            points,
            width,
        }
    }

    pub fn get(&self, point: &Point2<T>) -> Option<&U> {
        self.points.get(point)
    }

    pub fn neighbors(&self, point: &Point2<T>, include_diagonals: bool) -> Vec<Point2<T>> {
        let possible = point.neighbors(include_diagonals);

        possible.into_iter()
            .filter(|p| self.points.contains_key(p))
            .collect()
    }

    pub fn local_minimums(&self, include_diagonals: bool) -> Vec<Point2<T>> {
        self.points.iter()
            .filter(|(point, height)| {
                let neighbors = self.neighbors(point, include_diagonals);

                !neighbors.iter()
                    .flat_map(|p| self.get(p))
                    .any(|h| h <= height)
            })
            .map(|(point, _)| { point.clone() })
            .collect()
    }
}

impl<T, U> Grid<T, U>
    where T: Copy {
    pub fn area(&self) -> T {
        self.area
    }
    pub fn height(&self) -> T {
        self.height
    }
    pub fn max(&self) -> Point2<T> {
        self.max
    }
    pub fn min(&self) -> Point2<T> {
        self.min
    }
    pub fn width(&self) -> T {
        self.width
    }
}

impl Grid<Point2<i32>, i32> {
    pub fn parse_simple_input<T: AsRef<str>>(input: &[T]) -> Grid<i32, i32> {
        Grid::new(
            input.iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.as_ref()
                        .chars()
                        .enumerate()
                        .map(move |(x, c)| (Point2{x: x as i32, y: y as i32},c.to_digit(10).unwrap() as i32))
                }
                )
                .collect()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructor_simple() {
        let grid: Grid<i32, i32> = Grid::new(
            vec![((-1, -1), -1), ((2, 1), 2)]
                .into_iter()
                .map(|((x, y), z)| {
                    (Point2 { x, y }, z)
                })
                .collect()
        );

        assert_eq!(4, grid.width);
        assert_eq!(3, grid.height);
        assert_eq!(12, grid.area);
    }

    #[test]
    fn neighbors() {
        let mut points: HashMap<Point2<i32>, ()> = HashMap::new();
        for y in 0..=3 {
            for x in 0..=3 {
                points.insert(Point2 { x, y }, ());
            }
        }

        let grid = Grid::new(points);

        let corner_neighbors = grid.neighbors(&Point2 { x: 0, y: 0 }, false);

        assert_eq!(2, corner_neighbors.len());
        assert!(corner_neighbors.contains(&Point2 { x: 1, y: 0 }));
        assert!(corner_neighbors.contains(&Point2 { x: 0, y: 1 }));

        let edge_neighbors = grid.neighbors(&Point2 { x: 1, y: 0 }, false);
        assert_eq!(3, edge_neighbors.len());
        assert!(edge_neighbors.contains(&Point2 { x: 0, y: 0 }));
        assert!(edge_neighbors.contains(&Point2 { x: 1, y: 1 }));
        assert!(edge_neighbors.contains(&Point2 { x: 2, y: 0 }));
    }

    #[test]
    fn local_minimums() {
        let mut points: HashMap<Point2<i32>, i32> = vec![
            (Point2 { x: 0, y: 0 }, 1),
            (Point2 { x: 1, y: 0 }, 0),
            (Point2 { x: 2, y: 0 }, 1),
            (Point2 { x: 0, y: 1 }, 0),
            (Point2 { x: 1, y: 1 }, 1),
            (Point2 { x: 2, y: 1 }, 0),
            (Point2 { x: 0, y: 2 }, 1),
            (Point2 { x: 1, y: 2 }, 0),
            (Point2 { x: 2, y: 2 }, 1),
        ]
            .into_iter()
            .collect();

        let grid = Grid::new(points);

        let minimums = grid.local_minimums(false);

        assert_eq!(4, minimums.len());
        assert!(minimums.contains(&Point2 { x: 1, y: 0 }));
        assert!(minimums.contains(&Point2 { x: 0, y: 1 }));
        assert!(minimums.contains(&Point2 { x: 2, y: 1 }));
        assert!(minimums.contains(&Point2 { x: 1, y: 2 }));
    }

    #[test]
    fn local_minimums_equal_doesnt_count() {
        let mut points: HashMap<Point2<i32>, i32> = vec![
            (Point2 { x: 0, y: 0 }, 1),
            (Point2 { x: 1, y: 0 }, 2),
            (Point2 { x: 2, y: 0 }, 2),
            (Point2 { x: 0, y: 1 }, 2),
            (Point2 { x: 1, y: 1 }, 2),
            (Point2 { x: 2, y: 1 }, 2),
            (Point2 { x: 0, y: 2 }, 2),
            (Point2 { x: 1, y: 2 }, 2),
            (Point2 { x: 2, y: 2 }, 2),
        ]
            .into_iter()
            .collect();

        let grid = Grid::new(points);

        let minimums = grid.local_minimums(false);

        assert_eq!(1, minimums.len());
    }
}