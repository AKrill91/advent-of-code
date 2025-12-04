use crate::utils::point::Point;
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct Grid<T> {
    rows: Vec<Vec<T>>,
}

#[derive(Debug)]
pub struct AdjacentCells<'a, T> {
    pub above: Option<&'a T>,
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
    pub below: Option<&'a T>,
}

impl<'a, T> AdjacentCells<'a, T> {

    pub fn points(&self) -> [Option<&'a T>; 4] {
        [
            self.above,
            self.left,
            self.below,
            self.right,
        ]
    }
    pub fn count(&self) -> usize {
        self.points()
            .iter().filter(|o| o.is_some()).count()
    }
}

#[derive(Debug)]
pub struct AdjacentCellsDiagonal<'a, T> {
    pub above_left: Option<&'a T>,
    pub above: Option<&'a T>,
    pub above_right: Option<&'a T>,
    pub left: Option<&'a T>,
    pub right: Option<&'a T>,
    pub below_left: Option<&'a T>,
    pub below: Option<&'a T>,
    pub below_right: Option<&'a T>,
}

impl <'a, T> AdjacentCellsDiagonal<'a, T> {

    pub fn points(&self) -> [Option<&'a T>; 8] {
        [
            self.above_left,
            self.above,
            self.above_right,
            self.left,
            self.right,
            self.below_left,
            self.below,
            self.below_right,
        ]
    }
    pub fn count(&self) -> usize {
        self.points()
        .iter()
        .filter(|o| o.is_some())
        .count()
    }
}

impl<T> Grid<T>
where T: Debug, {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows.first().map(|row| row.len()).unwrap_or(0)
    }

    pub fn get<U: Copy + Into<i32>, V: Borrow<Point<U>>>(&self, position: V) -> Option<&T> {
        let pos = position.borrow();
        let y = pos.y.into();
        let x = pos.x.into();

        if x < 0 || y < 0 {
            None
        } else {
            self.rows.get(y as usize)
                .and_then(|row| row.get(x as usize))
        }
    }

    pub fn is_in_grid(&self, position: &Point<i32>) -> bool {
        position.x >= 0
            && position.y >= 0
            && position.y < self.rows.len() as i32
            && position.x < self.rows[position.y as usize].len() as i32
    }

    pub fn set(&mut self, position: Point<usize>, item: T) {
        self.rows[position.y][position.x] = item;
    }

    pub fn adjacent<U: Copy + Into<i32>, V: Borrow<Point<U>>>(&self, position: V) -> AdjacentCells<'_, T> {
        let pos = position.borrow();
        let x = pos.x.into();
        let y = pos.y.into();

        AdjacentCells {
            above: self.get(Point::new(x, y - 1)),
            left: self.get(Point::new(x - 1, y )),
            right: self.get(Point::new(x + 1, y)),
            below: self.get(Point::new(x, y + 1)),
        }
    }

    pub fn adjacent_diagonal<U: Copy + Into<i32>, V: Borrow<Point<U>>>(&self, position: V) -> AdjacentCellsDiagonal<'_, T> {
        let pos = position.borrow();
        let x = pos.x.into();
        let y = pos.y.into();

        AdjacentCellsDiagonal {
            above_left: self.get(Point::new(x - 1, y - 1)),
            above: self.get(Point::new(x, y - 1)),
            above_right: self.get(Point::new(x + 1, y - 1)),
            left: self.get(Point::new(x - 1, y )),
            right: self.get(Point::new(x + 1, y)),
            below_left: self.get(Point::new(x - 1, y + 1)),
            below: self.get(Point::new(x, y + 1)),
            below_right: self.get(Point::new(x + 1, y + 1))
        }
    }
}

impl<T> Grid<T>
where
    T: Eq,
{
    pub fn all_points(&self) -> Vec<Point<usize>> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| Point::new(x, y)))
            .collect()
    }

    pub fn find<U: Borrow<T>>(&self, search: U) -> Vec<Point<usize>> {
        let search = search.borrow();
        let mut out = vec![];

        for (y, row) in self.rows.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if item.eq(search) {
                    out.push(Point::new(x, y));
                }
            }
        }

        out
    }
}

impl<T> Grid<T>
where
    T: Eq + Clone,
{
    pub fn replace<U: Borrow<T>>(&mut self, find: U, replace: T) {
        let find = find.borrow();
        self.rows.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|item| {
                if (&*item).eq(find) {
                    *item = replace.clone();
                }
            })
        })
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (row_index, row) in self.rows.iter().enumerate() {
            for (item_index, item) in row.iter().enumerate() {
                item.fmt(f)?;
                if item_index != row.len() - 1 {
                    std::fmt::Display::fmt(&" ", f)?;
                }
            }
            if row_index != self.rows.len() - 1 {
                std::fmt::Display::fmt(&'\n', f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::utils::point::Point;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> super::Grid<i32> {
        init();
        super::Grid {
            rows: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]],
        }
    }

    #[test]
    fn display() {
        let grid = example();

        let expected = "1 2 3 4\n5 6 7 8";

        assert_eq!(expected, grid.to_string());
    }

    #[test]
    fn get() {
        let grid = example();

        assert_eq!(1, grid.get(Point::new(0u8, 0)).cloned().unwrap());
        assert_eq!(6, grid.get(Point::new(1u8, 1)).cloned().unwrap());
        assert_eq!(None, grid.get(Point::new(2u8, 2)));
    }

    #[test]
    fn find() {
        let grid = example();

        assert_eq!(vec![Point::new(0, 0)], grid.find(&1));
        assert_eq!(vec![Point::new(1, 1)], grid.find(&6));
        assert_eq!(Vec::<Point<usize>>::new(), grid.find(&9));
    }

    #[test]
    fn is_in_grid() {
        let grid = example();

        assert!(grid.is_in_grid(&Point::new(0, 0)));
        assert!(grid.is_in_grid(&Point::new(1, 1)));

        assert!(!grid.is_in_grid(&Point::new(-1, -1)));
        assert!(!grid.is_in_grid(&Point::new(1, 2)));
        assert!(!grid.is_in_grid(&Point::new(4, 1)));
    }

    #[test]
    fn replace() {
        let mut grid = example();
        let point = Point::new(1u8, 0);
        assert_eq!(2, *grid.get(&point).unwrap());
        grid.replace(2, 20);
        assert_eq!(20, *grid.get(&point).unwrap());
    }

    #[test]
    fn adjacent() {
        let grid = example();
        let point = Point::new(1, 0);
        let adjacent = grid.adjacent(&point);
        assert_eq!(None, adjacent.above);
        assert_eq!(grid.get(Point::new(0, 0)), adjacent.left);
        assert_eq!(grid.get(Point::new(2, 0)), adjacent.right);
        assert_eq!(grid.get(Point::new(1, 1)), adjacent.below);

        let adjacent_diagonal = grid.adjacent_diagonal(&point);

        assert_eq!(None, adjacent_diagonal.above_left);
        assert_eq!(None, adjacent_diagonal.above);
        assert_eq!(None, adjacent_diagonal.above_right);
        assert_eq!(grid.get(Point::new(0, 0)), adjacent_diagonal.left);
        assert_eq!(grid.get(Point::new(2, 0)), adjacent_diagonal.right);
        assert_eq!(grid.get(Point::new(0, 1)), adjacent_diagonal.below_left);
        assert_eq!(grid.get(Point::new(1, 1)), adjacent_diagonal.below);
        assert_eq!(grid.get(Point::new(2, 1)), adjacent_diagonal.below_right);
    }
}
