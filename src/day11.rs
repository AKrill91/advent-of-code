use std::collections::{HashMap, HashSet};

use intcode_computer::IntCodeComputer;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Panel {
    position: Position,
    is_white: bool,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0)
        }
    }
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let panels = run(&input, false);

    panels.len() as i32
}

pub fn run_b(input: &Vec<String>) -> String {
    let panels = run(&input, true);

    print(&panels)
}

fn run(input: &Vec<String>, start_on_white: bool) -> HashMap<Position, Panel> {
    let mut panels: HashMap<Position, Panel> = HashMap::new();
    let computer = IntCodeComputer::new(vec![]);
    let mut program = computer.start(&input);

    let mut move_number = 0;
    let mut direction = Direction::North;
    let mut position = Position { x: 0, y: 0 };

    loop {
        if !panels.contains_key(&position) {
            panels.insert(position, Panel { position, is_white: if move_number == 0 { start_on_white } else { false } });
        }

        let current_panel = panels.get_mut(&position).unwrap();

        debug!("On move {} -- {:?}", move_number, current_panel);

        program.input(
            if current_panel.is_white {
                1
            } else {
                0
            }
        );

        if program.halted() {
            info!("Program halted");
            break;
        }

        let outputs = program.latest_outputs(2);

        debug!("Got {:?} outputs", outputs);

        let color = outputs[0];
        let dir = outputs[1];

        current_panel.is_white = color == 1;
        if dir == 0 {
            direction = direction.turn_left();
        } else {
            direction = direction.turn_right();
        }

        let offset = direction.offset();

        position = Position { x: position.x + offset.0, y: position.y + offset.1};

        move_number += 1;
    }

    panels
}

fn print(panels: &HashMap<Position, Panel>) -> String {
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;

    let mut chars = vec![];

    for position in panels.keys() {
        let x = position.x;
        let y = position.y;

        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    info!("Hull is {} by {}", max_x - min_x, max_y - min_y);

    for y in (min_y..max_y+1).rev() {
        for x in min_x..max_x+1 {
            let pos = Position {x, y};

            let panel = panels.get(&pos);

            let c = match panel {
                Some(p) => if p.is_white { '@' } else { ' '},
                None => ' '
            };

            chars.push(c);
        }
        chars.push('\n');
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {}
}