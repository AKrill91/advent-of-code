use std::collections::HashMap;

use regex::Regex;

struct Instruction {
    steps: HashMap<u8, Step>
}

impl Instruction {
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }

    pub fn upsert_step(&mut self, step_id: u8) {
        self.steps
            .entry(step_id)
            .or_insert(Step { id: step_id, parent_ids: Vec::new(), complete: false });
    }

    pub fn add_prerequisite(&mut self, step_id: u8, prereq_id: u8) {
        let steps = &mut self.steps;
        let step_opt = steps.get_mut(&step_id);
        let step = step_opt.unwrap();

        let parents = &mut step.parent_ids;
        parents.push(prereq_id);
    }

    pub fn is_step_complete(&self, step_id: u8) -> bool {
        self.steps.get(&step_id).unwrap().complete
    }

    pub fn complete_step(&mut self, step_id: u8) {
        if !self.is_step_completable(step_id) {
            panic!("Attempted to complete step that cannot be completed");
        }

        let steps = &mut self.steps;
        let step_opt = steps.get_mut(&step_id);
        let step = step_opt.unwrap();
        step.complete = true;
    }

    pub fn is_step_completable(&self, step_id: u8) -> bool {
        let step = self.steps.get(&step_id).unwrap();

        let mut completable = true;

        for parent_id in &step.parent_ids {
            completable = completable && self.is_step_complete(*parent_id);

            if !completable {
                break;
            }
        }

        completable
    }
}

struct Step {
    id: u8,
    parent_ids: Vec<u8>,
    complete: bool,
}

pub fn run_a(input: &Vec<String>) -> String {
    let mut instructions = parse_instructions(input);
    let num_steps = instructions.num_steps();
    let mut output = String::new();

    for (step_id, step) in instructions.steps.iter() {
        println!("Step {} depends on {}", *step_id as char, String::from_utf8_lossy(&step.parent_ids))
    }

    while output.len() != num_steps {
        let mut completable = Vec::new();

        for step_id in instructions.steps.keys() {
            if !instructions.is_step_complete(*step_id) && instructions.is_step_completable(*step_id) {
                completable.push(*step_id);
            }
        }

        completable.sort();

        let next_id = completable[0];
        let next_char = next_id as char;

        println!("{} is next step to complete", next_char);
        instructions.complete_step(next_id);

        output.push(next_char);
    }

    println!("Step order is {}", output);

    output
}

pub fn run_b(input: &Vec<String>, num_workers: i32, base_duration: i32) -> i32 {
    let mut instructions = parse_instructions(input);
    let num_steps = instructions.num_steps();

    println!("Found {} steps", num_steps);

    let mut output = String::new();
    let mut elapsed_seconds = 0;

    let mut step_starts: HashMap<u8, i32> = HashMap::new();
    let mut assignments: HashMap<i32, u8> = HashMap::new();

    while output.len() != num_steps {
        println!("Elapsed seconds is {}", elapsed_seconds);

        let mut available = Vec::new();

        {
            let mut finished_workers = Vec::new();
            let mut finished_steps = Vec::new();

            for (worker_id_ref, step_id_ref) in assignments.iter() {
                let worker_id = *worker_id_ref;
                let step_id = *step_id_ref;
                let step_start = *step_starts.get(&step_id).unwrap();
                let step_len = base_duration + step_id as i32 - 64;

                if elapsed_seconds >= step_start + step_len {
                    println!("Worker {} finished step {}", worker_id, step_id as char);
                    finished_steps.push(step_id);
                    instructions.complete_step(step_id);
                    finished_workers.push(worker_id);
                }
            }

            for worker_id in finished_workers {
                assignments.remove(&worker_id);
            }

            finished_steps.sort();

            for step in finished_steps {
                output.push(step as char);
            }
        }

        for step_id in instructions.steps.keys() {
            let mut is_available = !step_starts.contains_key(step_id);
            is_available = is_available && !instructions.is_step_complete(*step_id);
            is_available = is_available && instructions.is_step_completable(*step_id);

            if is_available {
                println!("Step {} is available", step_id);
                available.push(*step_id);
            }
        }

        println!("Found {} available steps", available.len());

        available.sort();
        let mut next_index = 0;

        for worker_id in 0..num_workers {
            if !assignments.contains_key(&worker_id) {
                if next_index + 1 <= available.len() {
                    let next_step = available[next_index];

                    println!("Worker {} starting step {}", worker_id, next_step as char);
                    assignments.insert(worker_id, next_step);
                    step_starts.insert(next_step, elapsed_seconds);
                    next_index += 1;
                }
            }
        }

        elapsed_seconds += 1;
    }

    println!("Order of steps is {}, will take {} workers {} seconds", output, num_workers, elapsed_seconds - 1);

    elapsed_seconds - 1
}

fn parse_instructions(input: &Vec<String>) -> Instruction {
    let mut output = Instruction { steps: HashMap::new() };

    let step_pattern = Regex::new("Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    for line in input {
        if step_pattern.is_match(line) {
            let captures = step_pattern.captures(line).unwrap();

            let req = captures.get(1).unwrap().as_str().as_bytes()[0];
            let id = captures.get(2).unwrap().as_str().as_bytes()[0];

            output.upsert_step(req);
            output.upsert_step(id);
            output.add_prerequisite(id, req);
        }
    }

    println!("Found {} steps", output.num_steps());

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_input_a() {
        let input = vec![
            String::from("Step C must be finished before step A can begin."),
            String::from("Step C must be finished before step F can begin."),
            String::from("Step A must be finished before step B can begin."),
            String::from("Step A must be finished before step D can begin."),
            String::from("Step B must be finished before step E can begin."),
            String::from("Step D must be finished before step E can begin."),
            String::from("Step F must be finished before step E can begin.")
        ];

        assert_eq!(String::from("CABDFE"), run_a(&input));
    }

    #[test]
    fn sample_input_b() {
        let input = vec![
            String::from("Step C must be finished before step A can begin."),
            String::from("Step C must be finished before step F can begin."),
            String::from("Step A must be finished before step B can begin."),
            String::from("Step A must be finished before step D can begin."),
            String::from("Step B must be finished before step E can begin."),
            String::from("Step D must be finished before step E can begin."),
            String::from("Step F must be finished before step E can begin.")
        ];

        assert_eq!(15, run_b(&input, 2, 0));
    }
}