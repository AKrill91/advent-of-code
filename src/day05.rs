use std::collections::HashSet;

pub fn run_a(input: &Vec<String>) -> i64 {
    input.iter()
        .map(|line| Seat::from(line.as_str()))
        .map(|s| s.get_id())
        .max()
        .unwrap() as i64
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let seat_ids: HashSet<_> = input.iter()
        .map(|line| Seat::from(line.as_str()))
        .map(|s| s.get_id())
        .collect();

    for id in seat_ids.iter() {

        let next = id + 2;
        let maybe = id + 1;
        if seat_ids.contains(&next) && ! seat_ids.contains(&maybe){
            return maybe;
        }
    }

    0
}

struct Seat {
    row: usize,
    column: usize,
}

impl From<&str> for Seat {
    fn from(spec: &str) -> Self {
        let chars: Vec<char> = spec.chars().collect();

        let mut row: i32 = 0;
        let mut column: i32 = 0;

        for i in 0..7 {
            let c = chars[i as usize];

            if c == 'B' {
                row += 2i32.pow(6 - i);
            }
        }

        for i in 7..10 {
            let c = chars[i as usize];

            if c == 'R' {
                column += 2i32.pow(9 - i);
            }
        }

        Seat {
            row: row as usize,
            column: column as usize
        }
    }
}

impl Seat {
    pub fn get_id(&self) -> i64 {
        (self.row * 8 + self.column) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn seat_from_sample_0() {
        let spec = "BFFFBBFRRR";
        let seat = Seat::from(spec);

        assert_eq!(70, seat.row);
        assert_eq!(7, seat.column);
        assert_eq!(567, seat.get_id());
    }

    #[test]
    pub fn seat_from_sample_1() {
        let spec = "FFFBBBFRRR";
        let seat = Seat::from(spec);

        assert_eq!(14, seat.row);
        assert_eq!(7, seat.column);
        assert_eq!(119, seat.get_id());
    }

    #[test]
    pub fn seat_from_sample_2() {
        let spec = "BBFFBBFRLL";
        let seat = Seat::from(spec);

        assert_eq!(102, seat.row);
        assert_eq!(4, seat.column);
        assert_eq!(820, seat.get_id());
    }

    #[test]
    pub fn sample_input_0_a() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = vec![
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ];

        assert_eq!(820, run_a(&input));
    }
}