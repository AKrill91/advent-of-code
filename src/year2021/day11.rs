use crate::common::{Grid, Point2};

pub fn run_a(_: i32, input: &[String]) -> String {
    run(input, 100)
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("")
}

fn run<T: AsRef<str>>(input: &[T], step_count: usize) -> String {
    let mut grid = parse_octopuses(input);

    let num_flashes: i32 = (0..step_count).into_iter()
        .map(|_| step(&mut grid))
        .sum();

    format!("{}", num_flashes)
}

fn parse_octopuses<T: AsRef<str>>(input: &[T]) -> Grid<i32, i32> {
    Grid::parse_simple_input(input)
}

fn step(grid: &mut Grid<i32,i32>) -> i32 {
    let mut flash_count = 0;

    grid.points_mut()
        .iter_mut()
        .for_each(|(_, val)| *val += 1);

    let mut neighbors =

    for (_pos, val) in grid.points_mut()
        .iter_mut() {
        if *val >= 9 {
            flash_count += 1;
            *val = 0;
        }
    }

    flash_count
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn single_octopus() {
        assert_eq!("1", run(&["9"], 1));
    }

    #[test]
    fn two_octopi() {
        assert_eq!("2", run(&["87"], 1));
    }

    fn get_sample() -> Vec<String> {
        vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "1656";

        assert_eq!(expected, run_a(0, &input));
    }

    #[test]
    fn test_sample_b() {
        init();

        let input = get_sample();

        let expected = "";

        assert_eq!(expected, run_b(0, &input));
    }
}