pub fn run_a(_: i32, input: &Vec<String>) -> String {
    let mut counter = 0;
    let mut previous: Option<usize> = None;

    input.iter()
        .map(|line| line.parse::<usize>().unwrap())
        .for_each(|i| {
            if let Some(j) = previous {
                if j < i {
                    counter += 1;
                }
            }

            previous = Some(i);
        });

    counter.to_string()
}

pub fn run_b(_: i32, input: &Vec<String>) -> String {
    let mut counter = 0;
    let mut iter = input.iter()
        .map(|line| line.parse::<usize>().unwrap());

    let mut minus_two = iter.next().unwrap();
    let mut minus_one = iter.next().unwrap();
    let mut previous_sum = None;

    for current in iter {
        let sum = minus_two + minus_one + current;

        if let Some(previous) = previous_sum {
            if previous < sum {
                counter += 1;
            }
        }

        minus_two = minus_one;
        minus_one = current;
        previous_sum = Some(sum);
    }

    counter.to_string()
}