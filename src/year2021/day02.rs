pub fn run_a(_: i32, input: &Vec<String>) -> String {
    let mut horizontal = 0;
    let mut depth = 0;

    input.iter()
        .map(|line| line.split(' '))
        .for_each(|mut pair| {
            let direction = pair.next().unwrap();
            let amount = pair.next().unwrap().parse::<i32>().unwrap();

            match direction {
                "forward" => {
                    horizontal += amount;
                },
                "up" => {
                    depth -= amount;
                },
                "down" => {
                    depth += amount;
                },
                _ => {
                    log::error!("Unknown direction '{}'", direction);
                }
            }
        });

    format!("{}", horizontal * depth)
}

pub fn run_b(_: i32, input: &Vec<String>) -> String {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    input.iter()
        .map(|line| line.split(' '))
        .for_each(|mut pair| {
            let direction = pair.next().unwrap();
            let amount = pair.next().unwrap().parse::<i32>().unwrap();

            match direction {
                "forward" => {
                    horizontal += amount;
                    depth += aim * amount;
                },
                "up" => {
                    aim -= amount;
                },
                "down" => {
                    aim += amount;
                },
                _ => {
                    log::error!("Unknown direction '{}'", direction);
                }
            }
        });

    format!("{}", horizontal * depth)
}