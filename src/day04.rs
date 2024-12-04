use crate::utils::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

fn parse(input: &str) -> Grid<char> {
    Grid::new(input.lines()
        .map(|line| line.chars().into_iter().collect::<Vec<char>>())
        .collect::<Vec<_>>())
}

pub fn run_a(input: &str) -> i64 {
    let grid = parse(input);

    let xs = grid.find('X');

    let mut result = 0;

    for x in xs {
        for direction in Direction::values() {
            let dir_offset = crate::utils::point::Point::from(*direction);
            let second_pos = dir_offset + x;
            let third_pos = second_pos + dir_offset;
            let fourth_pos = third_pos + dir_offset;

            let m = grid.get(Point::new(second_pos.x as usize, second_pos.y as usize)).cloned();
            let a = grid.get(Point::new(third_pos.x as usize, third_pos.y as usize)).cloned();
            let s = grid.get(Point::new(fourth_pos.x as usize, fourth_pos.y as usize)).cloned();

            if matches!((m, a, s), (Some('M'), Some('A'), Some('S'))) {
                result += 1;
            }
        }
    }

    result
}

pub fn run_b(input: &str) -> i64 {
    let grid = parse(input);
    let centers = grid.find('A');

    let mut result = 0;

    for center in centers {
        if [(Direction::Northwest, Direction::Southeast), (Direction::Northeast, Direction::Southwest)].iter().all(|(one, two)| {
            let one_offset = Point::from(*one);
            let two_offset = Point::from(*two);

            let one_pos = one_offset + center;
            let two_pos = two_offset + center;

            let char_one = grid.get(Point::new(one_pos.x as usize, one_pos.y as usize)).cloned();
            let char_two = grid.get(Point::new(two_pos.x as usize, two_pos.y as usize)).cloned();

            match (char_one, char_two) {
                (Some('M'), Some('S')) | (Some('S'), Some('M')) => true,
                _ => false
            }

        }) {
            result += 1;
        }
    }

    result

}

#[cfg(test)]
mod test {

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn small_example() -> &'static str {
        r"
..X...
.SAMX.
.A..A.
XMAS.S
.X....
"
    }

    fn example() -> &'static str {
        r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
    }

    fn example_b() -> &'static str {
        r"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
"
    }

    #[test]
    fn part_a_small_example() {
        init();
        assert_eq!(4, super::run_a(small_example()));
    }

    #[test]
    fn part_a_example() {
        assert_eq!(18, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        assert_eq!(9, super::run_b(example_b()));
    }
}