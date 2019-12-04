use std::collections::{HashSet, HashMap};

pub fn run_a(input: &Vec<String>) -> i64 {
    let mut wires = vec![];

    for line in input {
        wires.push(parse_wire(line));
    }
    let intersection = intersection(&wires[0], &wires[1]);

    let mut min_distance = std::i64::MAX;

    for (point, _) in intersection {
        let dist = point.0.abs() + point.1.abs();

        if dist < min_distance && dist > 0 {
            min_distance = dist;
        }
    }

    min_distance
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let mut wires = vec![];

    for line in input {
        wires.push(parse_wire(line));
    }

    let intersection = intersection(&wires[0], &wires[1]);

    let mut min_distance = std::i64::MAX;

    for (_, d) in intersection {
        if d < min_distance && d > 0 {
            min_distance = d;
        }
    }

    min_distance
}

fn intersection(first: &HashMap<(i64, i64), i64>, second: &HashMap<(i64, i64), i64>) -> HashMap<(i64, i64), i64> {
    let mut output : HashMap<(i64, i64), i64> = HashMap::new();

    let first_uniques = to_hashset(&first);
    let second_uniques = to_hashset(&second);

    let intersection = first_uniques.intersection(&second_uniques);

    for point in intersection {
        let first_dist = *first.get(point).unwrap();
        let second_dist = *second.get(point).unwrap();

        output.insert(*point, first_dist + second_dist);
    }

    output
}

fn to_hashset(input: &HashMap<(i64, i64), i64>) -> HashSet<(i64, i64)> {
    let mut output : HashSet<(i64, i64)> = HashSet::new();

    for (point, _) in input {
        output.insert(*point);
    }

    output
}

fn parse_wire(input: &str) -> HashMap<(i64, i64), i64> {
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut points = HashMap::new();
    points.insert((pos_x, pos_y), 0);

    let instructions = input.split(",");
    let mut step = 1;

    for instruction in instructions {
        let (offset, amount) = parse_instruction(instruction);

        debug!("{}: {:?} - {}", instruction, offset, amount);

        for _ in 0..amount {
            pos_x += offset.0;
            pos_y += offset.1;
            points.insert((pos_x, pos_y), step);
            step += 1;
        }
    }

    points
}

fn parse_instruction(instruction: &str) -> ((i64, i64), i64) {
    let direction = &instruction[0..1];
    let offset = match direction {
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        "U" => (0, 1),
        _ => {
            warn!("Unknown direction {}", direction);
            (0, 0)
        }
    };

    let amount = instruction[1..].parse::<i64>().unwrap();

    (offset, amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sample_input_0_a() {
        let input = vec![
            String::from("R8,U5,L5,D3"),
            String::from("U7,R6,D4,L4")
        ];

        assert_eq!(6, run_a(&input));
    }

    #[test]
    pub fn sample_input_1_a() {
        let input = vec![
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83")
        ];

        assert_eq!(159, run_a(&input));
    }

    #[test]
    pub fn sample_input_2_a() {
        let input = vec![
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ];

        assert_eq!(135, run_a(&input));
    }

    #[test]
    pub fn sample_input_0_b() {
        let input = vec![
            String::from("R8,U5,L5,D3"),
            String::from("U7,R6,D4,L4")
        ];

        assert_eq!(30, run_b(&input));
    }

    #[test]
    pub fn sample_input_1_b() {
        let input = vec![
            String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            String::from("U62,R66,U55,R34,D71,R55,D58,R83")
        ];

        assert_eq!(610, run_b(&input));
    }

    #[test]
    pub fn sample_input_2_b() {
        let input = vec![
            String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ];

        assert_eq!(410, run_b(&input));
    }
}