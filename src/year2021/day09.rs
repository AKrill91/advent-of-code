use crate::common::{Grid, Point2};

pub fn run_a(_: i32, input: &[String]) -> String {
    let grid = parse_grid(input);

    let minimums = grid.local_minimums(false);

    info!("Found {} local minimums: {:?}", minimums.len(), minimums);

    let min_sum: u8 = minimums.iter()
        .map(|point| grid.get(point).unwrap() + 1)
        .sum();

    format!("{}", min_sum)
}

pub fn run_b(_: i32, input: &[String]) -> String {
    format!("")
}

fn parse_grid<T: AsRef<str>>(lines: &[T]) -> Grid<i32, u8> {
    Grid::new(
        lines.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.as_ref()
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| (Point2{x: x as i32, y: y as i32},c.to_digit(10).unwrap() as u8))
            }
            )
            .collect()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(&get_sample());

        assert_eq!(5, grid.height());
        assert_eq!(10, grid.width());
        assert_eq!(0u8, *grid.get(&(9, 0).into()).unwrap());
        assert_eq!(5u8, *grid.get(&(2, 2).into()).unwrap());
    }

    fn get_sample() -> Vec<String> {
        vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "15";

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