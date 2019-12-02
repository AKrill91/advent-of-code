use advent_helper;

pub fn run_a() {
    let adjustments = get_adjustments();

    let mut frequency = 0;

    for adjustment in &adjustments {
        frequency += adjustment;
    }

    info!("Sum of adjustments is {}", frequency);
}

pub fn run_b() {
    let adjustments = get_adjustments();

    let mut frequency = 0;

    let mut frequencies = Vec::new();
    let mut output: Option<i32> = None;
    let mut loop_counter = 0;

    while output.is_none() {
        info!("Starting Loop #{}", loop_counter);
        for adjustment in &adjustments {
            frequency += adjustment;

            if frequencies.contains(&frequency) {
                output = Some(frequency);
                break;
            } else {
                frequencies.push(frequency);
            }
        }
        loop_counter += 1;
    }

    info!("First duplicated frequency: {}", output.unwrap())
}

fn get_adjustments() -> Vec<i32> {
    let lines = advent_helper::read_file_lines("resources/day01.txt");

    let mut adjustments = Vec::new();

    for line in lines {
        let val: i32 = line.parse::<i32>().expect("Error parsing line as integer");
        adjustments.push(val);
    }

    adjustments
}