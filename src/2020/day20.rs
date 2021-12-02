use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let tiles = parse(input);
    let corners = find_corners(&tiles);

    info!("{:?}", corners.iter().map(|t| t.id).collect::<Vec<_>>());

    corners.iter()
        .map(|tile| tile.id as i64)
        .product()
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    0
}

fn parse<'a, I, J>(input: I) -> HashMap<i32, Tile>
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut tiles = HashMap::new();
    let mut iter = input.into_iter().peekable();

    while iter.peek().is_some() {
        let id_line: &str = iter.next().unwrap().as_ref();

        let id = id_line.split(" ").nth(1).unwrap();
        let id = id[..id.len() - 1].parse().unwrap();

        let mut data = vec![];
        let mut width = None;
        let  mut height = 0;

        while iter.peek().is_some() {
            let data_line: &str = iter.next().unwrap().as_ref();

            if data_line.is_empty() {
                break;
            } else {
                for c in data_line.chars() {
                    data.push(c == '#');
                }

                if width.is_none() {
                    width = Some(data.len());
                }
            }
            height += 1;
        }

        tiles.insert(id, Tile {
            id,
            width: width.unwrap(),
            height,
            data
        });
    }

    tiles
}

fn find_corners(tiles: &HashMap<i32, Tile>) -> Vec<&Tile> {
    let mut edge_counts: HashMap<i32, usize> = HashMap::new();

    for tile in tiles.values() {
        for edge in tile.get_edges() {
            let count = edge_counts.entry(edge).or_insert(0);
            *count += 1;
        }
    }

    info!("{:?}", edge_counts);

    tiles.values()
        .into_iter()
        .filter(|tile| {
            let edges = tile.get_edges();

            info!("{}, {:?}", tile.id, edges);

            edges.iter()
                .filter(|e| *edge_counts.get(*e).unwrap() == 1)
                .count() == 4
        })
        .collect()
}

struct Tile {
    id: i32,
    width: usize,
    height: usize,
    data: Vec<bool>
}

impl Tile {
    pub fn get_edges(&self) -> HashSet<i32> {
        TileEdge::list().iter()
            .map(|edge| self.get_edge(*edge))
            .collect()
    }

    pub fn get_edge(&self, edge: TileEdge) -> i32 {
        let mut sum = 0;
        let (start, step) = edge.get_loop_params(self.width, self.height);

        for i in 0..self.width {
            let d = self.data[(start as i32 + (step * i as i32)) as usize];

            if d {
                sum += 2i32.pow(i as u32);
            }
        }

        sum
    }
}

#[derive(Copy, Clone)]
enum TileEdge {
    Top,
    TopMirror,
    Right,
    RightMirror,
    Bottom,
    BottomMirror,
    Left,
    LeftMirror
}

impl TileEdge {
    pub fn list() -> Vec<TileEdge> {
        vec![
            TileEdge::Top,
            TileEdge::TopMirror,
            TileEdge::Right,
            TileEdge::RightMirror,
            TileEdge::Bottom,
            TileEdge::BottomMirror,
            TileEdge::Left,
            TileEdge::LeftMirror
        ]
    }

    pub fn get_loop_params(&self, width: usize, height: usize) -> (usize, i32) {
        let top_left = 0;
        let top_right = width - 1;
        let bottom_right = width * height - 1;
        let bottom_left = width * (height - 1);
        match self {
            TileEdge::Top => (top_left, 1),
            TileEdge::TopMirror => (top_right, -1),
            TileEdge::Right => (top_right, width as i32),
            TileEdge::RightMirror => (bottom_right, width as i32 * -1),
            TileEdge::Bottom => (bottom_left, 1),
            TileEdge::BottomMirror => (bottom_right, -1),
            TileEdge::Left => (top_left, width as i32),
            TileEdge::LeftMirror => (bottom_left, width as i32 * -1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn tile_get_edge_top() {
        let tile = Tile {
            id: 1,
            width: 3,
            height: 3,
            data: vec![true, false, false, false, false, false, false, false, false]
        };

        assert_eq!(1, tile.get_edge(TileEdge::Top));
        assert_eq!(4, tile.get_edge(TileEdge::TopMirror));
    }

    #[test]
    pub fn tile_get_edge_right() {
        let tile = Tile {
            id: 1,
            width: 3,
            height: 3,
            data: vec![false, false, true, false, false, false, false, false, false]
        };

        assert_eq!(1, tile.get_edge(TileEdge::Right));
        assert_eq!(4, tile.get_edge(TileEdge::RightMirror));
    }

    #[test]
    pub fn tile_get_edge_bottom() {
        let tile = Tile {
            id: 1,
            width: 3,
            height: 3,
            data: vec![false, false, false, false, false, false, true, false, false]
        };

        assert_eq!(1, tile.get_edge(TileEdge::Bottom));
        assert_eq!(4, tile.get_edge(TileEdge::BottomMirror));
    }

    #[test]
    pub fn tile_get_edge_left() {
        let tile = Tile {
            id: 1,
            width: 3,
            height: 3,
            data: vec![true, false, false, false, false, false, false, false, false]
        };

        assert_eq!(1, tile.get_edge(TileEdge::Left));
        assert_eq!(4, tile.get_edge(TileEdge::LeftMirror));
    }

    #[test]
    pub fn tile_get_edges() {
        let tile = Tile {
            id: 1,
            width: 3,
            height: 3,
            data: vec![true, false, true, false, false, true, false, false, false]
        };

        let edges = tile.get_edges();

        assert!(edges.contains(&0));
        assert!(edges.contains(&1));
        assert!(edges.contains(&3));
        assert!(edges.contains(&4));
        assert!(edges.contains(&5));
        assert!(edges.contains(&6));
    }

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
            "",
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
            "",
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
            "",
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
            "",
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
            "",
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
            "",
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
            "",
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
            "",
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
            "",
        ]
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(20_899_048_083_289, run_a(&sample));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(-1, run_b(&sample));
    }
}