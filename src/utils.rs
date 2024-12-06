use crate::utils::point::Point;

pub mod grid;
pub mod point;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest
}

const CARDINALS: &[Direction; 4] = &[Direction::North, Direction::East, Direction::South, Direction::West];
const INTERCARDINALS: &[Direction; 4] = &[Direction::Northeast, Direction::Southeast, Direction::Southwest, Direction::Northwest];
const ALL_DIRECTIONS: &[Direction; 8] = &[
    Direction::North,
    Direction::Northeast,
    Direction::East,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
];

const VERTICALS: &[Direction; 2] = &[Direction::North, Direction::South];
const HORIZONTALS: &[Direction; 2] = &[Direction::East, Direction::West];
const NE_SW: &[Direction; 2] = &[Direction::Northeast, Direction::Southwest];
const SE_NW: &[Direction; 2] = &[Direction::Southeast, Direction::Northwest];

impl Direction {
    pub fn cardinals() -> &'static [Direction] {
        CARDINALS
    }

    pub fn intercardinals() -> &'static [Direction] {
        INTERCARDINALS
    }

    pub fn values() -> &'static [Direction] {
        ALL_DIRECTIONS
    }

    pub fn perpendiculars(self) -> &'static [Direction] {
        match self {
            Direction::North | Direction::South => HORIZONTALS,
            Direction::East | Direction::West => VERTICALS,
            Direction::Northeast | Direction::Southwest => SE_NW,
            Direction::Southeast | Direction::Northwest => NE_SW
        }
    }

    pub fn clockwise_90(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::Northeast => Direction::Southeast,
            Direction::East => Direction::South,
            Direction::Southeast => Direction::Southwest,
            Direction::South => Direction::West,
            Direction::Southwest => Direction::Northwest,
            Direction::West => Direction::North,
            Direction::Northwest => Direction::Northeast
        }
    }
}

impl From<Direction> for Point<i32> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Point::new(0, -1),
            Direction::Northeast => Point::new(1, -1),
            Direction::East => Point::new(1, 0),
            Direction::Southeast => Point::new(1, 1),
            Direction::South => Point::new(0, 1),
            Direction::Southwest => Point::new(-1, 1),
            Direction::West => Point::new(-1, 0),
            Direction::Northwest => Point::new(-1, -1),
        }
    }
}