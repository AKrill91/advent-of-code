use std::collections::HashMap;

use advent_helper;

pub fn run_a() -> i32 {
    let ids = advent_helper::read_file_lines("resources/day02.txt").expect("Error reading file");

    let mut num_twos = 0;
    let mut num_threes = 0;

    for id in ids {
        let mut letter_counts = HashMap::new();

        let letters = id.as_bytes();

        for letter in letters {
            let count = letter_counts.entry(letter).or_insert(0);
            *count += 1;
        }

        let mut exact_two = false;
        let mut exact_three = false;

        for (_, count) in letter_counts {
            if exact_two && exact_three {
                break;
            }

            if count == 2 {
                exact_two = true;
            } else if count == 3 {
                exact_three = true;
            }
        }

        if exact_two {
            num_twos += 1;
        }

        if exact_three {
            num_threes += 1;
        }
    }

    info!("Found {} ids with exactly 2 matching letters", num_twos);
    info!("Found {} ids with exactly 3 matching letters", num_threes);

    let output = num_twos * num_threes;
    info!("Checksum is {}", num_twos * num_threes);
    output
}

pub fn run_b() -> String {
    let ids = advent_helper::read_file_lines("resources/day02.txt").expect("Error reading file");
    let mut output = String::from("");

    for (index, id) in ids.iter().enumerate() {
        debug!("Id at index {} is {}", index, id);

        for (compare_index, compare_id) in ids[index..].iter().enumerate() {
            debug!("Comparing against index {}", compare_index);

            let bytes = id.as_bytes();
            let compare_bytes = compare_id.as_bytes();
            let mut num_diffs = 0;

            let mut similar_letters : Vec<u8> = Vec::new();

            for pair in bytes.iter().zip(compare_bytes.iter()) {
                let left = pair.0;
                let right = pair.1;

                if left != right {
                    num_diffs += 1;
                } else {
                    similar_letters.push(*left);
                }
            }

            if num_diffs == 1 {
                output =  String::from_utf8(similar_letters).expect("Error building output");
                info!("{} and {} only differ by 1, similar letters are {}", id, compare_id, output);

                return output;
            }
        }
    }

    output
}