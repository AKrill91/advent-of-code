use std::collections::HashSet;

pub fn run_a(input: &Vec<String>) -> i64 {
    let groups = FlightGroup::multiple_from(&input);

    let sum: usize = groups.iter()
        .map(|g| g.combined_answers())
        .map(|set| set.len())
        .sum();

    sum as i64
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let groups = FlightGroup::multiple_from(&input);

    let sum: usize = groups.iter()
        .map(|g| g.answer_intersection())
        .map(|set| set.len())
        .sum();

    sum as i64
}

struct FlightPerson {
    answers: HashSet<char>
}

impl From<&str> for FlightPerson {
    fn from(input: &str) -> Self {
        FlightPerson {
            answers: input.chars()
                .collect()
        }
    }
}

struct FlightGroup {
    persons: Vec<FlightPerson>
}

impl From<Vec<FlightPerson>> for FlightGroup {
    fn from(persons: Vec<FlightPerson>) -> Self {
        FlightGroup {
            persons
        }
    }
}

impl FlightGroup {
    pub fn answer_intersection(&self) -> HashSet<char> {
        let mut intersection: HashSet<char> = HashSet::new();
        let mut first = true;

        for person in &self.persons {
            if first {
                intersection = person.answers.clone();
                first = false;
            } else {
                intersection = intersection.intersection(&person.answers)
                    .cloned()
                    .collect();
            }

        }

        intersection
    }

    pub fn combined_answers(&self) -> HashSet<char> {
        self.persons
            .iter()
            .flat_map(|p| &p.answers)
            .cloned()
            .collect()
    }

    pub fn multiple_from(input: &Vec<String>) -> Vec<Self> {
        let mut lines: Vec<&str> = vec![];

        let mut groups: Vec<Self> = vec![];

        for line in input {
            if line == "" {
                let persons: Vec<FlightPerson> = lines.iter()
                    .map(|l| FlightPerson::from(*l))
                    .collect();

                groups.push(FlightGroup::from(persons));
                lines.clear();
            } else {
                lines.push(line.as_str());
            }
        }

        if !lines.is_empty() {
            let persons: Vec<FlightPerson> = lines.iter()
                .map(|l| FlightPerson::from(*l))
                .collect();

            groups.push(FlightGroup::from(persons));
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        vec![
            String::from("abc"),
            String::from(""),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from(""),
            String::from("ab"),
            String::from("ac"),
            String::from(""),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from(""),
            String::from("b"),
        ]
    }

    #[test]
    pub fn person_from_str() {
        let person = FlightPerson::from("abc");

        assert_eq!(3, person.answers.len());
        assert!(person.answers.contains(&'a'));
        assert!(person.answers.contains(&'b'));
        assert!(person.answers.contains(&'c'));
    }

    #[test]
    pub fn flight_group_combined() {
        let group = FlightGroup::from(
            vec![
                FlightPerson::from("a"),
                FlightPerson::from("b"),
                FlightPerson::from("c")
            ]
        );

        let answers = group.combined_answers();

        assert_eq!(3, answers.len());

        assert!(answers.contains(&'a'));
        assert!(answers.contains(&'b'));
        assert!(answers.contains(&'c'));
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(11, run_a(&input))
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        assert_eq!(6, run_b(&input))
    }
}