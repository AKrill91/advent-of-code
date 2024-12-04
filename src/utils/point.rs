use std::ops::Add;
use crate::utils::Direction;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl <T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T> Add for Point<T> where T: Add<Output = T> + Copy {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<Direction> for &Point<i32> {
    type Output = Point<i32>;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs_point = Point::from(rhs);
        Point {
            x: self.x + rhs_point.x,
            y: self.y + rhs_point.y,
        }
    }
}

impl Add<Point<usize>> for Point<i32> {
    type Output = Self;

    fn add(self, rhs: Point<usize>) -> Self::Output {
        Point {
            x: self.x + rhs.x as i32,
            y: self.y + rhs.y as i32
        }
    }
}