use crate::utils::grid::Grid;
use crate::utils::point::Point;

type Roll = bool;
type Row = Vec<Roll>;
fn parse(input: &str) -> Grid<bool> {
    Grid::new(input.trim().lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> Row {
    line.chars().map(|c| c == '@').collect()
}

pub fn run_a(input: &str) -> i64 {
    let grid = parse(input);

    let mut accessible_count = 0;

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let point = Point::new(x as i32, y as i32);
            if grid.get(&point).cloned().unwrap_or_default() {
                let adjacent = grid.adjacent_diagonal(&point);
                let roll_count = adjacent.points().iter().filter_map(|x| *x)
                    .filter(|x| **x)
                    .count();

                if roll_count < 4 {
                    accessible_count += 1;
                }
            }
        }
    }

    accessible_count
}

pub fn run_b(input: &str) -> i64 {
    let mut removed_count = 0;
    let mut grid = parse(input);

    let mut changed = true;

    let mut pass = 0;
    while changed {
        log::debug!("Pass: {}", pass);
        let mut copy = grid.clone();
        changed = false;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let point = Point::new(x as i32, y as i32);
                if grid.get(&point).cloned().unwrap_or_default() {
                    let adjacent = grid.adjacent_diagonal(&point);
                    let roll_count = adjacent.points().iter().filter_map(|x| *x)
                        .filter(|x| **x)
                        .count();

                    if roll_count < 4 {
                        copy.set(Point::new(x, y), false);
                        removed_count += 1;
                        changed = true;
                    }
                }
            }
        }
        grid = copy;
        pass += 1;
    }

    removed_count
}

#[cfg(test)]
mod test {
    use crate::utils::point::Point;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
        assert_eq!(10, parsed.width());
        assert_eq!(10, parsed.height());
        assert_eq!(true, parsed.get(Point::new(4, 2)).cloned().unwrap());
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(13, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(43, super::run_b(example()));
    }
}
