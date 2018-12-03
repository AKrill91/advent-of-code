extern crate time;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;


fn main() {
    let lines = read_file_lines("resources/day01.txt").expect("Error reading file");

    let mut adjustments = Vec::new();

    for line in lines {
        let val : i32 = line.parse::<i32>().expect("Error parsing line as integer");
        adjustments.push(val);
    }

    let mut frequency = 0;

    for adjustment in &adjustments {
        frequency += adjustment;
    }

    println!("{}: Sum of adjustments is {}", time::now().rfc822(), frequency);

    frequency = 0;

    let mut frequencies = Vec::new();
    let mut output : Option<i32> = None;
    let mut loop_counter = 0;

    while output.is_none() {
        println!("{}: Starting Loop #{}", time::now().rfc822(), loop_counter);
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

    println!("{}: First duplicated frequency: {}", time::now().rfc822(), output.unwrap())
}

fn read_file_lines(filename: &str) -> Result<Vec<String>, Error> {
    let mut lines = Vec::new();

    let f =  File::open(filename);

    match f {
        Ok(file) => {
            let reader = BufReader::new(&file);

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        lines.push(l);
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            Ok(lines)
        },
        Err(e) => Err(e)
    }
}