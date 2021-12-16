use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Self {
        Point {
            x: p.0,
            y: p.1,
        }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",");

        Ok(
            Point {
                x: iter.next().unwrap().parse::<i32>().unwrap(),
                y: iter.next().unwrap().parse::<i32>().unwrap(),
            }
        )
    }
}

impl Point {
    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");
        let start = iter.next().unwrap();
        iter.next();
        let end = iter.next().unwrap();

        Ok(
            Line {
                start: Point::from_str(start).unwrap(),
                end: Point::from_str(end).unwrap(),
            }
        )
    }
}

impl Line {
    pub fn pairs(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Line {
            start: Point {
                x: x1,
                y: y1,
            },
            end: Point {
                x: x2,
                y: y2,
            },
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn points(&self) -> Vec<Point> {
        let mut out = vec![];

        if self.is_horizontal() {
            let y = self.start.y;
            let x_start = self.start.x.min(self.end.x);
            let x_end = self.start.x.max(self.end.x);

            for x in x_start..=x_end {
                out.push(Point::from((x, y)));
            }
        } else if self.is_vertical() {
            let x = self.start.x;
            let y_start = self.start.y.min(self.end.y);
            let y_end = self.start.y.max(self.end.y);

            for y in y_start..=y_end {
                out.push(Point::from((x, y)));
            }
        }

        out
    }
}

pub fn run_a(_: i32, input: &Vec<String>) -> String {
    let lines: Vec<Line> = input.iter()
        .map(|line| Line::from_str(&line).unwrap())
        .collect();

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    lines.iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
        .flat_map(|line| line.points())
        .for_each(|point| {
            let tuple = point.to_tuple();
            let entry = grid.entry(tuple).or_insert(0);

            *entry += 1;
        });

    let count = grid.iter()
        .filter(|(point, count)| **count >= 2)
        .count();

    format!("{}", count)
}

pub fn run_b(_: i32, input: &Vec<String>) -> String {
    format!("")
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_sample() -> Vec<String> {
        vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn line_fromstr() {
        init();
        let expected = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 1, y: 1 },
        };

        assert_eq!(expected, Line::from_str("0,0 -> 1,1").unwrap());
    }

    #[test]
    fn line_is_horizontal() {
        init();

        assert!(Line::pairs(0, 0, 1, 0).is_horizontal());
    }

    #[test]
    fn line_is_vertical() {
        init();

        assert!(Line::pairs(0, 0, 0, 1).is_vertical());
    }

    #[test]
    fn line_points_horizontal() {
        init();

        let line = Line::pairs(0, 0, 2, 0);

        let expected = vec![
            Point::from((0, 0)),
            Point::from((1, 0)),
            Point::from((2, 0)),
        ];

        assert_eq!(expected, line.points());
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "5";

        assert_eq!(expected, run_a(5, &input));
    }

    #[test]
    fn test_sample_b() {
        init();
    }
}