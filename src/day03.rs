use std::collections::HashMap;


struct Claim {
    id: String,
    start: (i32, i32),
    size: (i32, i32)
}

pub fn run_a(input: Vec<String>) -> i32 {
    let mut fabric_counts = HashMap::new();

    for line in input {
        let claim = parse_claim(line);

        debug!("{} @ ({},{}) - {}x{}", claim.id, claim.start.0, claim.start.1, claim.size.0, claim.start.1);

        for x_offset in 0..claim.size.0 {
            for y_offset in 0..claim.size.1 {
                let x = claim.start.0 + x_offset;
                let y = claim.start.1 + y_offset;

                let point = (x, y);

                let count = fabric_counts.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut num_collisions = 0;

    for (_, count) in fabric_counts {
        if count > 1 {
            num_collisions += 1;
        }
    }

    info!("Found {} points with more than one claim", num_collisions);

    num_collisions
}

pub fn run_b() {}

fn parse_claim(line: String) -> Claim {
    let mut parts = line.split(" ");
    let claim_id = parts.next().unwrap();
    parts.next();

    let start_coords = parts.next().unwrap();
    let mut coord_parts = start_coords.split(",");
    let coord_x = coord_parts.next().unwrap();
    let mut coord_y = coord_parts.next().unwrap();
    coord_y = &coord_y[..coord_y.len() - 1];

    let size = parts.next().unwrap();
    let mut size_parts = size.split("x");
    let size_x = size_parts.next().unwrap();
    let size_y = size_parts.next().unwrap();

    let point: (i32, i32) = (
        coord_x.parse::<i32>().expect("Error parsing x coord as int"),
        coord_y.parse::<i32>().expect("Error parsing y coord as int")
    );

    let size_tuple: (i32, i32) = (
        size_x.parse::<i32>().expect("Error parsing x size as int"),
        size_y.parse::<i32>().expect("Error parsing y size as int")
    );

    Claim {
        id: claim_id.to_string(),
        start: point,
        size: size_tuple
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let sample: Vec<String> = vec![
            String::from("#1 @ 1,3: 4x4"),
            String::from("#2 @ 3,1: 4x4"),
            String::from("#3 @ 5,5: 2x2")
        ];

        assert_eq!(4, run_a(sample));
    }
}