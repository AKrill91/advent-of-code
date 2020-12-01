use std::collections::HashMap;

use intcode_computer::IntCodeComputer;

struct Game {
    tiles: HashMap<Position, TileId>,
    ball_pos: Position,
    paddle_pos: Position,
    score: i64
}

impl Game {
    pub fn create(outputs: &Vec<i64>) -> Game {
        let mut game = Game {
            tiles: HashMap::new(),
            ball_pos: Position{x:0, y:0},
            paddle_pos: Position{x:0, y:0},
            score: 0
        };

        game.handle_updates(outputs);

        game
    }

    pub fn handle_updates(&mut self, outputs: &Vec<i64>) {
        let num_triplets = outputs.len() / 3;

        for i in 0..num_triplets {
            let x = outputs[i * 3];
            let y = outputs[i * 3 + 1];
            let id_i64 = outputs[i * 3 + 2];

            if x == -1 && y == 0 {
                self.score = id_i64;
                continue;
            }

            let id = TileId::from_i64(id_i64);
            let position = Position { x, y };

            if id == TileId::Ball {
                self.ball_pos = position;
            } else if id == TileId::Paddle {
                self.paddle_pos = position;
            }

            self.tiles.insert(position, id);
        }
    }

    pub fn count_tiles_by_id(&self, look_for: TileId) -> i32 {
        self.tiles.values()
            .filter(|tile| **tile == look_for)
            .count() as i32
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum TileId {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Unknown,
}

impl TileId {
    pub fn from_i64(id: i64) -> TileId {
        match id {
            0 => TileId::Empty,
            1 => TileId::Wall,
            2 => TileId::Block,
            3 => TileId::Paddle,
            4 => TileId::Ball,
            99 => TileId::Unknown,
            _ => panic!()
        }
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let computer = IntCodeComputer::new(vec![]);
    let outputs = computer.run(&input, vec![]);
    let game = Game::create(&outputs);
    game.count_tiles_by_id(TileId::Block)
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let computer = IntCodeComputer::new(vec![]);
    let mut adjustments = HashMap::new();
    adjustments.insert(0, 2);

    let mut program = computer.start_with_adjustments(&input, adjustments);
    program.run();
    let mut outputs = program.get_outputs();

    let mut game = Game::create(&outputs);

    program.clear_outputs();

    let mut num_blocks = game.count_tiles_by_id(TileId::Block);

    while num_blocks > 0 {
        let paddle_pos = game.paddle_pos.clone();
        let ball_pos = game.ball_pos.clone();

        let input = if ball_pos.x < paddle_pos.x {
            -1
        } else if ball_pos.x > paddle_pos.x {
            1
        } else {
            0
        };

        program.input(input);

        outputs = program.get_outputs();

        game.handle_updates(&outputs);

        num_blocks = game.count_tiles_by_id(TileId::Block);
    }

    game.score
}
