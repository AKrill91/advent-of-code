use std::fmt::{Display, Formatter};

pub fn run_a(input: &Vec<String>) -> i64 {
    let forest = Forest::from(input);

    let mut num_collisions = 0;
    let slope_x = 3;
    let slope_y = 1;
    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < forest.height {
        if forest.tree_at(pos_x, pos_y) {
            num_collisions += 1;
        }

        pos_x += slope_x;
        pos_y += slope_y;

    }

    num_collisions
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let forest = Forest::from(input);
    let mut product = 1;

    for (slope_x, slope_y) in slopes {
        let mut num_collisions = 0;
        let mut pos_x = 0;
        let mut pos_y = 0;

        while pos_y < forest.height {
            if forest.tree_at(pos_x, pos_y) {
                num_collisions += 1;
            }

            pos_x += slope_x;
            pos_y += slope_y;

        }

        product *= num_collisions;
    }

    product
}

struct Forest {
    pattern: Vec<Vec<bool>>,
    width: usize,
    height: usize
}

impl Display for Forest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::with_capacity((self.width + 1) * self.height);

        for row in &self.pattern {
            for col in row {
                let c = if *col {
                    '#'
                } else {
                    '.'
                };

                str.push(c);
            }
            str.push('\n');
        }
        f.write_str(&str)
    }
}

impl From<&Vec<String>> for Forest {
    fn from(input: &Vec<String>) -> Self {
        let mut grid = vec![];

        for line in input {
            let mut row = vec![];

            for c in line.chars() {
                row.push(c == '#');
            }

            grid.push(row);
        }

        let height = grid.len();
        let width = grid.get(0).map_or(0, |r| r.len());

        Forest {
            pattern: grid,
            width,
            height
        }
    }
}

impl Forest {
    fn tree_at(&self, x: usize, y: usize) -> bool {
        let real_x = x % self.width;
        self.pattern[y][real_x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ]
    }

    #[test]
    pub fn forest_parse_small() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("..#"),
            String::from("#.#")
        ];

        let forest = Forest::from(&input);

        assert_eq!(3, forest.width);
        assert_eq!(2, forest.height);
        assert_eq!(true, forest.tree_at(2, 0));
    }

    #[test]
    pub fn forest_parse_sample() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        let forest = Forest::from(&input);

        assert_eq!(11, forest.width);
        assert_eq!(11, forest.height);
        assert_eq!(true, forest.tree_at(1, 2));
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(7, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(336, run_b(&input));
    }
}