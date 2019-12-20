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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MazeElement {
    Entrance,
    Empty,
    Key { id: char },
    Lock { id: char },
    Wall,
}

impl MazeElement {
    pub fn to_char(&self) -> char {
        match self {
            MazeElement::Entrance => '@',
            MazeElement::Empty => ' ',
            MazeElement::Key { id: c } => *c,
            MazeElement::Lock { id: c } => *c,
            MazeElement::Wall => '#',
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            MazeElement::Entrance => true,
            MazeElement::Empty => false,
            MazeElement::Key { .. } => true,
            MazeElement::Lock { .. } => false,
            MazeElement::Wall => false,
        }
    }

    pub fn is_wall(&self) -> bool {
        match self {
            MazeElement::Wall => true,
            _ => false
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MazeElement::Empty => true,
            _ => false
        }
    }
}

#[derive(Debug)]
struct Node {
    position: Point,
    element: MazeElement,
}

struct Maze {
    entrance: Point,
    width: i64,
    height: i64,
    elements: HashMap<Point, MazeElement>,
    nodes: HashMap<Point, Node>
}

impl Maze {
    fn parse(input: &Vec<String>) -> Maze {
        let mut entrance = Point::zero();
        let mut elements = HashMap::new();
        let mut width = 0;
        let mut y = 0;

        for line in input {
            let mut x = 0;
            for ch in line.chars() {
                let point = Point { x, y };
                let element = match ch {
                    '#' => MazeElement::Wall,
                    '.' => MazeElement::Empty,
                    '@' => MazeElement::Entrance,
                    c if c.is_ascii_lowercase() => MazeElement::Key { id: c },
                    c if c.is_ascii_uppercase() => MazeElement::Lock { id: c },
                    _ => panic!()
                };
                elements.insert(point, element);
                x += 1;
            }
            if y == 0 {
                width = x;
            }
            y += 1;
            x = 0;
        }

        let width = width;
        let height = y;
//        let mut graph = Graph::new_undirected();
        let mut nodes = HashMap::new();

        info!("Width: {}, Height: {}", width, height);
        info!("{:?}", elements);

        for y in 0..height {
            for x in 0..width {
                let point = Point { x, y };
                let element = elements.get(&point).unwrap();

                if element.is_node() {
                    nodes.insert(
                        point,
                        Node {
                            position: point,
                            element: *element,
                        },
                    );

                    if element == &MazeElement::Entrance {
                        entrance = point;
                    }
                } else if element.is_empty() {
                    let north = Point { x: point.x, y: point.y - 1 };
                    let east = Point { x: point.x + 1, y: point.y };
                    let south = Point { x: point.x, y: point.y + 1 };
                    let west = Point { x: point.x - 1, y: point.y };

                    let north_south = elements.get(&north).map_or(false, |e| e.is_empty()) ||
                        elements.get(&south).map_or(false, |e| e.is_empty());

                    let east_west = elements.get(&east).map_or(false, |e| e.is_empty()) ||
                        elements.get(&west).map_or(false, |e| e.is_empty());

                    let intersection = north_south && east_west;

                    if intersection {
                        nodes.insert(
                            point,
                            Node {
                                position: point,
                                element: MazeElement::Empty
                            }
                        );
                    }
                }
            }
        }

        info!("{:?}", nodes);

        Maze {
            entrance,
            width,
            height,
            elements,
            nodes
        }
    }

    fn render(&self) -> String {
        let mut chars = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point { x, y };

                let element = self.elements.get(&point).unwrap();

                if element.is_empty() && self.nodes.contains_key(&point) {
                    chars.push('.');
                } else {
                    chars.push(element.to_char());
                }
            }
            chars.push('\n');
        }

        chars.into_iter().collect()
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let mut maze = Maze::parse(input);

    info!("\n{}", maze.render());

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