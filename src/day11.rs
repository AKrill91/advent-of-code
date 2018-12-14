use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct PowerResult {
    x: i32,
    y: i32,
    size: i32,
    power: i32,
}

pub fn run_a(serial_number: i32, width: i32, height: i32) -> String {
    let power_levels = get_power_levels(serial_number, width, height);

    let max_result = find_max_point(width, height, 3, &power_levels);

    let result = format!("{},{}", max_result.x, max_result.y);

    println!("Largest 3x3 starts at {} with power {}", result, max_result.power);

    result
}

pub fn run_b(serial_number: i32, width: i32, height: i32) -> String {
    let power_levels = get_power_levels(serial_number, width, height);

    let mut max_result = PowerResult {x: 0, y: 0, size: 0, power: 0};

    for size in 1..301 {
        println!("Checking size {}", size);
        let power_result = find_max_point(
            width,
            height,
            size,
            &power_levels
        );

        if power_result.power > max_result.power {
            max_result = power_result;
        }
    }

    let result = format!("{},{},{}", max_result.x, max_result.y, max_result.size);

    println!(
        "Largest square is a {}x{} starting at {},{} with power {}",
        max_result.size,
        max_result.size,
        max_result.x,
        max_result.y,
        max_result.power
    );

    result

}

fn find_max_point(width: i32, height: i32, size: i32, power_levels: &HashMap<Point, i32>) -> PowerResult{

    let mut max_power = 0;
    let mut max_point = Point { x: 0, y: 0 };

    let y_max = height - size + 1;
    let x_max = width - size + 1;

    for y in 0..y_max  {
        for x in 0..x_max {
            let mut sum_power = 0;

            for y_offset in 0..size {
                for x_offset in 0..size {
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

    PowerResult {
        x: max_point.x,
        y: max_point.y,
        size,
        power: max_power
    }
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

    #[test]
    #[ignore]
    fn sample_input_b_1() {
        assert_eq!(String::from("90,269,16"), run_b(18, 300, 300));
    }

    #[test]
    #[ignore]
    fn sample_input_b_2() {
        assert_eq!(String::from("232,251,12"), run_b(42, 300, 300));
    }
}