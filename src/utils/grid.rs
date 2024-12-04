use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use crate::utils::point::Point;

pub struct Grid<T> {
    rows: Vec<Vec<T>>,
}

impl <T> Grid<T> {

    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self {
            rows
        }
    }

    pub fn get(&self, position: Point<usize>) -> Option<&T> {
        self.rows.get(position.y)
            .and_then(|row| row.get(position.x))
    }
}

impl <T> Grid<T>
where T: Eq {
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

impl <T> Display for Grid<T>
 where T: Display {
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

    fn example() -> super::Grid<i32> {
        super::Grid {
            rows: vec![vec![1, 2, 3, 4], vec![5,6,7,8]]
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

        assert_eq!(1, grid.get(Point::new(0, 0)).cloned().unwrap());
        assert_eq!(6, grid.get(Point::new(1, 1)).cloned().unwrap());
        assert_eq!(None, grid.get(Point::new(2, 2)));
    }

    #[test]
    fn find() {
        let grid = example();

        assert_eq!(vec![Point::new(0, 0)], grid.find(&1));
        assert_eq!(vec![Point::new(1, 1)], grid.find(&6));
        assert_eq!(Vec::<Point<usize>>::new(), grid.find(&9));
    }
}