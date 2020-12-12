use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    run(input, FerryRoundAdjacent {})
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    run(input, FerryRoundVisible {})
}

fn run<'a, I, J, L>(input: I, logic: L) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized,
        L: FerryRoundLogic
{
    let mut prev_ferry = Ferry::empty();
    let mut next_ferry = Ferry::from(input);

    while prev_ferry != next_ferry {
        prev_ferry = next_ferry;
        next_ferry = prev_ferry.do_round(&logic);
    }

    next_ferry.number_occupied() as i64
}

#[derive(Eq, PartialEq)]
struct Ferry {
    positions: Vec<FerryPosition>,
    width: usize,
    height: usize,
}

impl<'a, I, J> From<I> for Ferry
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    fn from(input: I) -> Self {
        let mut positions = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in input {
            height += 1;
            let str = line.as_ref();
            width = max(width, str.len());

            for c in str.chars() {
                positions.push(FerryPosition::from(c));
            }
        }

        Ferry {
            positions,
            width,
            height,
        }
    }
}

impl Ferry {
    pub fn empty() -> Ferry {
        Ferry {
            positions: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn position_at(&self, x: usize, y: usize) -> FerryPosition {
        self.positions[y * self.width + x]
    }

    pub fn number_occupied(&self) -> usize {
        self.positions.iter()
            .filter(|p| **p == FerryPosition::OccupiedSeat)
            .count()
    }

    pub fn do_round<T: FerryRoundLogic>(&self, logic: &T) -> Self {
        let mut new_positions = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let new_pos = logic.decide_new_position(self, x, y);

                new_positions.push(new_pos);
            }
        }

        Ferry {
            positions: new_positions,
            width: self.width,
            height: self.height,
        }
    }

    pub fn get_adjacent_positions(&self, x: usize, y: usize) -> Vec<FerryPosition> {
        let adj_y_start = if y == 0 { y } else { y - 1 };
        let adj_x_start = if x == 0 { x } else { x - 1 };
        let adj_y_end = min(y + 1, self.height - 1);
        let adj_x_end = min(x + 1, self.width - 1);
        (adj_y_start..=adj_y_end).into_iter()
            .flat_map(
                move |adj_y| (adj_x_start..=adj_x_end).into_iter()
                    .filter(move |adj_x| x != *adj_x || y != adj_y)
                    .map(move |adj_x| self.position_at(adj_x, adj_y))
            )
            .collect()
    }

    pub fn get_visible_positions(&self, x: usize, y: usize) -> Vec<FerryPosition> {
        let mut positions = vec![];
        let max_side: i32 = max(self.width, self.height) as i32;

        for slope_y in -1..=1 {
            for slope_x in -1..=1 {
                if slope_x == 0 && slope_y == 0 {
                    continue;
                }

                for i in 1..=max_side {
                    let examine_y: i32 = y as i32 + (slope_y * i);
                    let examine_x: i32 = x as i32 + (slope_x * i);

                    // info!("Examining {}, {} --- {}, {}, {}", examine_x, examine_y, slope_x, slope_y, i);
                    if !self.in_bounds(examine_x, examine_y) {
                        break;
                    }

                    let pos = self.position_at(examine_x as usize, examine_y as usize);
                    // info!("  in bounds, found {:?}", pos);

                    if pos.is_seat() {
                        positions.push(pos);
                        break;
                    }
                }
            }
        }

        positions
    }

    // pub fn clamp(&self, x: usize, y: usize) -> bool {}

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        return x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32;
    }
}

trait FerryRoundLogic {
    fn decide_new_position(&self, ferry: &Ferry, x: usize, y: usize) -> FerryPosition;
}

pub struct FerryRoundAdjacent {}

impl FerryRoundLogic for FerryRoundAdjacent {
    fn decide_new_position(&self, ferry: &Ferry, x: usize, y: usize) -> FerryPosition {
        let current = ferry.position_at(x, y);
        if current.is_floor() {
            return current;
        }

        let adjacent = ferry.get_adjacent_positions(x, y);

        let num_occupied = adjacent.iter()
            .filter(|p| p.is_occupied_seat())
            .count();

        if num_occupied == 0 {
            FerryPosition::OccupiedSeat
        } else if num_occupied >= 4 {
            FerryPosition::EmptySeat
        } else {
            current
        }
    }
}

pub struct FerryRoundVisible {}

impl FerryRoundLogic for FerryRoundVisible {
    fn decide_new_position(&self, ferry: &Ferry, x: usize, y: usize) -> FerryPosition {
        let current = ferry.position_at(x, y);
        if current.is_floor() {
            return current;
        }

        let visible = ferry.get_visible_positions(x, y);

        let num_occupied = visible.iter()
            .filter(|p| p.is_occupied_seat())
            .count();

        if num_occupied == 0 {
            FerryPosition::OccupiedSeat
        } else if num_occupied >= 5 {
            FerryPosition::EmptySeat
        } else {
            current
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FerryPosition {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl Into<char> for FerryPosition {
    fn into(self) -> char {
        match self {
            FerryPosition::Floor => '.',
            FerryPosition::EmptySeat => 'L',
            FerryPosition::OccupiedSeat => '#',
        }
    }
}

impl From<char> for FerryPosition {
    fn from(c: char) -> Self {
        match c {
            'L' => FerryPosition::EmptySeat,
            '.' => FerryPosition::Floor,
            '#' => FerryPosition::OccupiedSeat,
            _ => panic!()
        }
    }
}

impl FerryPosition {
    pub fn is_floor(&self) -> bool {
        *self == FerryPosition::Floor
    }

    pub fn is_empty_seat(&self) -> bool {
        *self == FerryPosition::EmptySeat
    }

    pub fn is_occupied_seat(&self) -> bool {
        *self == FerryPosition::OccupiedSeat
    }

    pub fn is_seat(&self) -> bool {
        *self != FerryPosition::Floor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn ferry_from_lines() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "L.",
            ".L"
        ];

        let ferry = Ferry::from(&input);

        assert_eq!(2, ferry.width);
        assert_eq!(2, ferry.height);
        assert_eq!(4, ferry.positions.len());
        assert_eq!(FerryPosition::EmptySeat, ferry.position_at(1, 1));
    }

    #[test]
    pub fn adjacent_positions_center() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "L.L",
            ".#.",
            "L.L",
        ];

        let ferry = Ferry::from(&input);

        let adj = ferry.get_adjacent_positions(1, 1);
        assert_eq!(8, adj.len());
        assert_eq!(4, adj.iter()
            .filter(|p| p.is_empty_seat())
            .count()
        );
        assert_eq!(4, adj.iter()
            .filter(|p| p.is_floor())
            .count()
        );
    }

    #[test]
    pub fn adjacent_positions_corner() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "L.L",
            ".#.",
            "L.L",
        ];

        let ferry = Ferry::from(&input);

        let adj = ferry.get_adjacent_positions(0, 0);
        assert_eq!(3, adj.len());
        assert_eq!(1, adj.iter()
            .filter(|p| p.is_occupied_seat())
            .count()
        );
        assert_eq!(2, adj.iter()
            .filter(|p| p.is_floor())
            .count()
        );
    }

    #[test]
    pub fn adjacent_positions_edge() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "L.L",
            ".#.",
            "L.L",
        ];

        let ferry = Ferry::from(&input);

        let adj = ferry.get_adjacent_positions(1, 2);
        assert_eq!(5, adj.len());
        assert_eq!(1, adj.iter()
            .filter(|p| p.is_occupied_seat())
            .count()
        );
        assert_eq!(2, adj.iter()
            .filter(|p| p.is_floor())
            .count()
        );
        assert_eq!(2, adj.iter()
            .filter(|p| p.is_empty_seat())
            .count()
        );
    }

    #[test]
    pub fn visible_positions_all() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ];

        let ferry = Ferry::from(&input);
        let visible = ferry.get_visible_positions(3, 4);

        assert_eq!(8, visible.len());
    }

    #[test]
    pub fn visible_positions_none() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            ".##.##.",
            "#.#.#.#",
            "##...##",
            "...L...",
            "##...##",
            "#.#.#.#",
            ".##.##.",
        ];

        let ferry = Ferry::from(&input);
        let visible = ferry.get_visible_positions(3, 3);

        assert_eq!(0, visible.len());
    }

    #[test]
    pub fn do_round_adj_all_unoccupied() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "L.L",
            ".L.",
            "L.L",
        ];

        let ferry = Ferry::from(&input);
        let logic = FerryRoundAdjacent {};
        let next_round = ferry.do_round(&logic);

        assert_eq!(5, next_round.number_occupied());
    }

    #[test]
    pub fn do_round_adj_all_occupied() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "#.#",
            ".#.",
            "#.#",
        ];

        //Should turn into
        // #.#
        // .L.
        // #.#

        let ferry = Ferry::from(&input);
        let logic = FerryRoundAdjacent {};
        let next_round = ferry.do_round(&logic);

        assert_eq!(4, next_round.number_occupied());
    }

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(37, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(26, run_b(&input));
    }
}