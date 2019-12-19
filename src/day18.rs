use std::collections::{HashMap, HashSet};

use petgraph::Graph;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn zero() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }
}

struct Key {
    id: char,
    position: Point,
    collected: bool,
}

struct Node {
    position: Point
}

struct Maze {
    entrance: Point,
    walls: HashSet<Point>,
    keys: HashMap<char, Key>,
    locks: HashMap<char, Point>,
}

impl Maze {
    fn parse(input: &Vec<String>) -> Maze {
        let mut entrance = Point::zero();
        let mut walls = HashSet::new();
        let mut keys = HashMap::new();
        let mut locks = HashMap::new();

        let mut y = 0;

        for line in input {
            let mut x = 0;
            for ch in line.chars() {
                let point = Point { x, y };
                match ch {
                    '#' => { walls.insert(point); }
                    '.' => {}
                    '@' => { entrance = point; }
                    c => {
                        assert!(c.is_ascii_alphabetic());

                        if c.is_ascii_lowercase() {
                            keys.insert(
                                c,
                                Key {
                                    id: c,
                                    position: point,
                                    collected: false,
                                },
                            );
                        } else if c.is_ascii_uppercase() {
                            locks.insert(c, point);
                        } else {
                            panic!();
                        }
                    }
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }

        Maze {
            entrance,
            walls,
            keys,
            locks,
        }
    }

    fn to_graph(&self) -> Graph<Node, i64> {
        let mut out = Graph::new_undirected();

        let mut nodes = HashMap::new();

        nodes.insert(
            self.entrance,
            out.add_node(
                Node {
                    position: self.entrance
                }
            ),
        );

        for (_, key) in self.keys {
            nodes.insert(
                key.position,
                out.add_node(
                    Node {
                        position: key.position
                    }
                ),
            );
        }

        out
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let mut maze = Maze::parse(input);

    info!("Found {} walls, {} keys, and {} locks", maze.walls.len(), maze.keys.len(), maze.locks.len());

    0
}

pub fn run_b(input: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            String::from("#########"),
            String::from("#b.A.@.a#"),
            String::from("#########"),
        ];

        assert_eq!(8, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            String::from("########################"),
            String::from("#f.D.E.e.C.b.A.@.a.B.c.#"),
            String::from("######################.#"),
            String::from("#d.....................#"),
            String::from("########################"),
        ];

        assert_eq!(86, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            String::from("########################"),
            String::from("#...............b.C.D.f#"),
            String::from("#.######################"),
            String::from("#.....@.a.B.c.d.A.e.F.g#"),
            String::from("########################"),
        ];

        assert_eq!(132, run_a(&input));
    }

    #[test]
    pub fn sample_input_3_a() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            String::from("#################"),
            String::from("#i.G..c...e..H.p#"),
            String::from("########.########"),
            String::from("#j.A..b...f..D.o#"),
            String::from("########@########"),
            String::from("#k.E..a...g..B.n#"),
            String::from("########.########"),
            String::from("#l.F..d...h..C.m#"),
            String::from("#################"),
        ];

        assert_eq!(136, run_a(&input));
    }

    #[test]
    pub fn sample_input_4_a() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            String::from("########################"),
            String::from("#@..............ac.GI.b#"),
            String::from("###d#e#f################"),
            String::from("###A#B#C################"),
            String::from("###g#h#i################"),
            String::from("########################"),
        ];

        assert_eq!(81, run_a(&input));
    }
}