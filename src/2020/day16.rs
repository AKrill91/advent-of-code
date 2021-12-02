use std::collections::{HashMap, HashSet};
use std::ops::{Not, RangeInclusive};

pub fn run_a<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let notes = Notes::from(input);

    notes.error_scanning_rate()
}

pub fn run_b<'a, I, J>(input: I) -> i64
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    let mut notes = Notes::from(input);
    notes.remove_invalid();

    let ticket = notes.analyze();

    ticket.fields.iter()
        .filter(|(name, value)| name.starts_with("departure"))
        .map(|(name, &value)| value)
        .product()
}

struct Notes {
    fields: Vec<Field>,
    our_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl<'a, I, J> From<I> for Notes
    where
        I: IntoIterator<Item=&'a J>,
        J: AsRef<str> + 'a + ?Sized
{
    fn from(input: I) -> Self {
        let mut fields = vec![];

        let mut nearby_tickets = vec![];

        let mut iter = input.into_iter();

        loop {
            let next: &str = iter.next().unwrap().as_ref();
            if next.len() == 0 {
                break;
            }

            fields.push(Field::from(next));
        }

        iter.next(); //your ticket:
        let our_ticket = Ticket::from(iter.next().unwrap().as_ref());

        iter.next(); //empty line
        iter.next(); //nearby tickets:

        for line in iter {
            nearby_tickets.push(Ticket::from(line.as_ref()));
        }

        Notes {
            fields,
            our_ticket,
            nearby_tickets,
        }
    }
}

impl Notes {
    pub fn error_scanning_rate(&self) -> i64 {
        self.nearby_tickets
            .iter()
            .map(|ticket| ticket.scanning_error(&self.fields))
            .sum()
    }

    pub fn remove_invalid(&mut self) {
        let fields = &self.fields;
        self.nearby_tickets.retain(|ticket| ticket.maybe_valid(fields));
    }

    pub fn analyze(&self) -> IdentifiedTicket {
        let num_fields = self.fields.len();
        let mut available_indices: HashSet<usize> = (0..num_fields).collect();

        let mut fields = HashMap::new();

        while !available_indices.is_empty() {
            let options: HashMap<&str, Vec<usize>> =self.fields.iter()
                .filter(|field| !fields.contains_key(field.name.as_str()))
                .map(
                    |field| {
                        let mut maybes = vec![];

                        for i in 0..num_fields {
                            if !available_indices.contains(&i) {
                                continue;
                            }
                            if !field.is_valid(self.our_ticket.fields[i]) {
                                continue;
                            }

                            if self.nearby_tickets.iter()
                                .all(|ticket| field.is_valid(ticket.fields[i])) {
                                maybes.push(i);
                            }
                        }

                        assert_ne!(maybes.len(), 0);

                        (field.name.as_str(), maybes)
                    }
                )
                .collect();

            let single_matches = options.iter()
                .filter(|(_, matches)| matches.len() == 1);

            for (name, matches) in single_matches {
                let index = matches[0];
                fields.insert(name.to_string(), self.our_ticket.fields[index]);
                available_indices.remove(&index);
            }
        }

        IdentifiedTicket {
            fields
        }
    }
}

struct Field {
    name: String,
    range_a: RangeInclusive<i64>,
    range_b: RangeInclusive<i64>,
}

impl From<&str> for Field {
    fn from(input: &str) -> Self {
        let mut parts = input.split(":");
        let name = parts.next().unwrap().trim().to_string();
        let mut ranges = parts.next().unwrap().trim().split(" or ");
        let mut range_a_parts = ranges.next().unwrap().trim().split("-");
        let range_a = RangeInclusive::new(
            range_a_parts.next().unwrap().parse::<i64>().unwrap(),
            range_a_parts.next().unwrap().parse::<i64>().unwrap(),
        );
        let mut range_b_parts = ranges.next().unwrap().trim().split("-");
        let range_b = RangeInclusive::new(
            range_b_parts.next().unwrap().parse::<i64>().unwrap(),
            range_b_parts.next().unwrap().parse::<i64>().unwrap(),
        );

        Field {
            name,
            range_a,
            range_b,
        }
    }
}

impl Field {
    pub fn is_valid(&self, value: i64) -> bool {
        let valid = self.range_a.contains(&value) || self.range_b.contains(&value);
        valid
    }
}

struct Ticket {
    fields: Vec<i64>
}

impl From<&str> for Ticket {
    fn from(input: &str) -> Self {
        let mut fields = vec![];

        for part in input.split(",") {
            fields.push(part.parse::<i64>().unwrap());
        }

        Ticket {
            fields
        }
    }
}

impl Ticket {
    pub fn scanning_error(&self, fields: &Vec<Field>) -> i64 {
        self.fields
            .iter()
            .filter(
                |&&value| fields.iter()
                    .any(|field| field.is_valid(value))
                    .not()
            )
            .sum()
    }

    pub fn maybe_valid(&self, fields: &Vec<Field>) -> bool {
        self.scanning_error(fields) == 0
    }
}

struct IdentifiedTicket {
    fields: HashMap<String, i64>
}

impl IdentifiedTicket {
    pub fn get(&self, field: &str) -> i64 {
        *self.fields.get(field).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_sample() -> Vec<&'static str> {
        vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
    }

    #[test]
    pub fn field_from_str() {
        let input = "class: 1-3 or 5-7";

        let field = Field::from(input);
        assert_eq!("class", field.name);
        assert_eq!(1, *field.range_a.start());
        assert_eq!(3, *field.range_a.end());
        assert_eq!(5, *field.range_b.start());
        assert_eq!(7, *field.range_b.end());
    }

    #[test]
    pub fn ticket_from_str() {
        let input = "7,1,14";

        let ticket = Ticket::from(input);

        assert_eq!(3, ticket.fields.len());
        assert_eq!(14, ticket.fields[2]);
    }

    #[test]
    pub fn notes_from_sample() {
        let input = get_sample();

        let notes = Notes::from(input);

        assert_eq!(50, *notes.fields[2].range_b.end());
        assert_eq!(1, notes.our_ticket.fields[1]);
        assert_eq!(20, notes.nearby_tickets[2].fields[2]);
    }

    #[test]
    pub fn ticket_maybe_valid_true() {
        let input = get_sample();

        let notes = Notes::from(input);

        assert_eq!(true, notes.nearby_tickets[0].maybe_valid(&notes.fields));
    }

    #[test]
    pub fn ticket_maybe_valid_false() {
        let input = get_sample();

        let notes = Notes::from(input);

        assert_eq!(false, notes.nearby_tickets[2].maybe_valid(&notes.fields));
    }

    #[test]
    pub fn ticket_scanning_error_zero() {
        let input = get_sample();

        let notes = Notes::from(input);

        assert_eq!(0, notes.nearby_tickets[0].scanning_error(&notes.fields));
    }

    #[test]
    pub fn ticket_scanning_error_nonzero() {
        let input = get_sample();

        let notes = Notes::from(input);

        assert_eq!(55, notes.nearby_tickets[2].scanning_error(&notes.fields));
    }

    #[test]
    pub fn notes_analyze() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = vec![
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
            "",
            "your ticket:",
            "11,12,13",
            "",
            "nearby tickets:",
            "3,9,18",
            "15,1,5",
            "5,14,9",
        ];

        let mut notes = Notes::from(input);
        notes.remove_invalid();
        let ticket = notes.analyze();
        assert_eq!(12, ticket.get("class"));
        assert_eq!(11, ticket.get("row"));
        assert_eq!(13, ticket.get("seat"));
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(71, run_a(&sample));
    }

    #[test]
    pub fn sample_input_0_b() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sample = get_sample();

        assert_eq!(1, run_b(&sample));
    }
}