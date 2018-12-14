use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run_a(serial_number: i32, width: i32, height: i32) -> String {
    let power_levels = get_power_levels(serial_number, width, height);

    let mut max_power = 0;
    let mut max_point = Point { x: 0, y: 0 };

    for y in 0..height - 2 {
        for x in 0..width - 2 {
            let mut sum_power = 0;

            for y_offset in 0..3 {
                for x_offset in 0..3 {
                    let point = Point {
                        x: x + x_offset,
                        y: y + y_offset,
                    };

                    let power_level = *power_levels.get(&point).unwrap();
                    sum_power += power_level;
                }
            }

            if sum_power > max_power {
                max_power = sum_power;
                max_point = Point { x, y };
            }
        }
    }

    let result = format!("{},{}", max_point.x, max_point.y);

    println!("Largest 3x3 starts at {}", result);

    result
}

fn get_power_levels(serial_number: i32, width: i32, height: i32) -> HashMap<Point, i32> {
    let mut output = HashMap::new();

    for x in 0..width + 1 {
        for y in 0..height + 1 {
            output.insert(Point { x, y }, power_level(x, y, serial_number));
        }
    }

    output
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;

    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level_sample() {
        assert_eq!(4, power_level(3, 5, 8));
    }

    #[test]
    fn sample_input_a() {
        let input = 42;

        assert_eq!(String::from("21,61"), run_a(input, 300, 300));
    }
}