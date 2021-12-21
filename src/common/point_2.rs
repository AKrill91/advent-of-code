use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Debug for Point2<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Point2{x:")?;
        self.x.fmt(f)?;
        f.write_str(",y:")?;
        self.y.fmt(f)?;
        f.write_str("}")
    }
}

impl<T> From<(T, T)> for Point2<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<Point2<T>> for (T, T) {
    fn from(point: Point2<T>) -> Self {
        (point.x, point.y)
    }
}

impl<T> Copy for Point2<T> where T: Copy + Clone {}

impl<T> Clone for Point2<T> where T: Clone {
    fn clone(&self) -> Self {
        Point2 {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Eq for Point2<T> where T: Eq + PartialEq {}

impl<T> PartialEq for Point2<T>
    where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl<T> Hash for Point2<T>
    where T: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl <T> Point2<T> where T: num_traits::PrimInt {
    pub fn neighbors(&self, include_diagonals: bool) -> Vec<Self> {
        let one = T::one();

        let mut out = vec![
            Point2{ x: self.x, y: self.y + one}, //Up
            Point2{ x: self.x - one, y: self.y}, //Left
            Point2{ x: self.x + one, y: self.y}, //Right
            Point2{ x: self.x, y: self.y - one}, //Down
        ];

        if include_diagonals {
            out.push(Point2{ x: self.x - one, y: self.y + one}); //Up-Left
            out.push(Point2{ x: self.x + one, y: self.y + one}); //Up-Right
            out.push(Point2{ x: self.x - one, y: self.y - one}); //Down-Left
            out.push(Point2{ x: self.x + one, y: self.y - one}); //Down-Right
        }

        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbors_no_diagonals() {
        let p = Point2{x: 0, y: 0};

        let neighbors = p.neighbors(false);

        assert_eq!(4, neighbors.len());
        assert!(neighbors.contains(&Point2{x: 0, y: 1}));
        assert!(neighbors.contains(&Point2{x: -1, y: 0}));
        assert!(neighbors.contains(&Point2{x: 1, y: 0}));
        assert!(neighbors.contains(&Point2{x: 0, y: -1}));
    }

    #[test]
    fn neighbors_diagonals() {
        let p = Point2{x: 0, y: 0};

        let neighbors = p.neighbors(true);

        assert_eq!(8, neighbors.len());
        assert!(neighbors.contains(&Point2{x: -1, y: 1}));
        assert!(neighbors.contains(&Point2{x: 1, y: 1}));
        assert!(neighbors.contains(&Point2{x: -1, y: -1}));
        assert!(neighbors.contains(&Point2{x: -1, y: 1}));
    }
}