use std::ops::{Add, AddAssign, Sub, SubAssign};
use crate::utils::Direction;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

impl <T> Sub for Point<T> where T: Sub<Output = T> + Copy {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl <T> AddAssign for Point<T> where T: Add<Output = T> + Copy {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl <T> SubAssign for Point<T> where T: Sub<Output = T> + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
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