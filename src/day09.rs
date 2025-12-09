use crate::utils::point::Point;

fn parse(input: &str) -> Vec<Point<i64>> {
    input.trim()
        .lines()
        .map(|l| {
            let mut parts = l.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            Point::new(x, y)
        })
        .collect()
}

pub fn run_a(input: &str) -> i64 {
    let points = parse(input);

    let mut max_area = i64::MIN;

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let width = (p1.x - p2.x).abs() + 1;
            let height = (p1.y - p2.y).abs() + 1;
            let area = width * height;

            max_area = max_area.max(area);
        }
    }

    max_area
}

pub fn run_b(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
        assert_eq!(parsed.len(), 8);
        assert_eq!(parsed[0], Point::new(7, 1));
        assert_eq!(parsed[1], Point::new(11, 1));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(50, run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(24, run_b(example()));
    }
}