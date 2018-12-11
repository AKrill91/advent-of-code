use std::collections::HashMap;

#[derive(Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Point {
    pub fn distance_to(&self, other: &Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }

    pub fn is_on_boundary(&self, width: i32, height: i32) -> bool {
        return self.x == 0 || self.y == 0 || self.x == width || self.y == height;
    }
}

struct Origin {
    id: char,
    point: Point,
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let origins = parse_origins(input);

    let mut max_x = -1;
    let mut max_y = -1;

    for origin in &origins {
        if origin.point.x > max_x {
            max_x = origin.point.x;
        }

        if origin.point.y > max_y {
            max_y = origin.point.y;
        }
    }

    info!("Found {} origins, grid is {} x {}", origins.len(), max_x, max_y);

    let mut grid = HashMap::new();

    for y in 0..max_y + 1 {
        let mut line = String::with_capacity((max_x + 1) as usize);

        for x in 0..max_x + 1 {
            let point = Point { x, y };

            let mut nearest_distance = std::i32::MAX;
            let mut nearest_point = None;

            for origin in &origins {
                let distance = point.distance_to(&origin.point);

                if distance == nearest_distance {
                    nearest_point = None;
                } else if distance < nearest_distance {
                    nearest_point = Some(origin.id);
                    nearest_distance = distance;
                }
            }

            if nearest_point.is_some() {
                grid.insert(point, nearest_point.unwrap());
                line.push(nearest_point.unwrap());
            } else {
                line.push('.');
            }
        }

//        println!("{}", line);
    }

    let mut area_totals: HashMap<char, i32> = HashMap::new();

    for (point, id) in grid.iter() {
        let count = area_totals.entry(*id).or_insert(0);
        *count += 1;

        if point.is_on_boundary(max_x, max_y) {
            *count -= 100000;
        }
    }

    let mut max_area_id = '?';
    let mut max_area = -1;

    for (id, area) in area_totals.iter() {
        info!("Origin {} has area {}", id, area);
        if *area > max_area {
            max_area = *area;
            max_area_id = *id;
        }
    }

    info!("Origin {} has the largest area with {}", max_area_id, max_area);

    max_area
}

pub fn run_b(input: &Vec<String>, max_distance: i32) -> i32 {
    let origins = parse_origins(input);

    let mut max_x = -1;
    let mut max_y = -1;

    for origin in &origins {
        if origin.point.x > max_x {
            max_x = origin.point.x;
        }

        if origin.point.y > max_y {
            max_y = origin.point.y;
        }
    }

    info!("Found {} origins, grid is {} x {}", origins.len(), max_x, max_y);

    let mut num_points_in_distance = 0;

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let point = Point { x, y };

            let mut origin_distance = 0;

            for origin in &origins {
                let distance = point.distance_to(&origin.point);

                origin_distance += distance;
            }

            if origin_distance < max_distance {
                num_points_in_distance += 1;
            }
        }
    }

    info!("{} points are within {} combined distance to origins", num_points_in_distance, max_distance);

    num_points_in_distance
}

fn parse_origins(input: &Vec<String>) -> Vec<Origin> {
    let mut origins = Vec::new();

    let mut id_counter = 0;

    for line in input {
        let mut parts = line.split(", ");
        let x_str = parts.next().expect("Error getting x coord");
        let y_str = parts.next().expect("Error getting y coord");

        let x = x_str.parse::<i32>().expect("Error parsing x coord");
        let y = y_str.parse::<i32>().expect("Error parsing y coord");

        origins.push(Origin { id: counter_to_char(id_counter), point: Point { x, y } });
        id_counter += 1;
    }

    origins
}

fn counter_to_char(counter: u8) -> char {
    let mut ascii = counter + 65;
    if ascii > 90 {
        ascii += 7;
    }

    ascii as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance_from_origin() {
        let first = Point { x: 0, y: 0 };
        let second = Point { x: 2, y: 2 };

        assert_eq!(4, first.distance_to(&second));
    }

    #[test]
    fn point_distance_negatives() {
        let first = Point { x: -1, y: -1 };
        let second = Point { x: -3, y: -3 };

        assert_eq!(4, first.distance_to(&second));
    }

    #[test]
    fn point_distance_negatives_swapped() {
        let first = Point { x: -1, y: -1 };
        let second = Point { x: -3, y: -3 };

        assert_eq!(4, second.distance_to(&first));
    }

    #[test]
    fn point_boundary_bottom() {
        let p = Point { x: 2, y: 0 };
        assert_eq!(true, p.is_on_boundary(5, 5));
    }

    #[test]
    fn point_boundary_top() {
        let p = Point { x: 2, y: 5 };
        assert_eq!(true, p.is_on_boundary(5, 5));
    }

    #[test]
    fn point_boundary_left() {
        let p = Point { x: 0, y: 2 };
        assert_eq!(true, p.is_on_boundary(5, 5));
    }

    #[test]
    fn point_boundary_right() {
        let p = Point { x: 5, y: 2 };
        assert_eq!(true, p.is_on_boundary(5, 5));
    }

    #[test]
    fn sample_input_a() {
        let sample = vec![
            String::from("1, 1"),
            String::from("1, 6"),
            String::from("8, 3"),
            String::from("3, 4"),
            String::from("5, 5"),
            String::from("8, 9")
        ];

        assert_eq!(17, run_a(&sample));
    }

    #[test]
    fn sample_input_b() {
        let sample = vec![
            String::from("1, 1"),
            String::from("1, 6"),
            String::from("8, 3"),
            String::from("3, 4"),
            String::from("5, 5"),
            String::from("8, 9")
        ];

        assert_eq!(16, run_b(&sample, 32));
    }
}