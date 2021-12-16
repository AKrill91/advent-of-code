use std::collections::HashSet;
use std::fmt::format;
use std::hash::Hash;
use std::slice::Iter;

struct Board {
    rows: Vec<Vec<i32>>
}

impl Board {
    fn new(rows: Vec<Vec<i32>>) -> Self {
        Board {
            rows
        }
    }

    fn unmarked_sum(&self, marked: &HashSet<i32>) -> i32 {
        self.rows.iter()
            .map(
                |row| {
                    let row_sum: i32 = row.iter()
                        .filter(|r| !marked.contains(*r))
                        .sum();

                    row_sum
                }
            )
            .sum()
    }

    fn wins(&self, marked: &HashSet<i32>) -> bool {
        let marks: Vec<Vec<bool>> = self.rows.iter()
            .map(|row| row.iter().map(|i| marked.contains(i))
                .collect()
            )
            .collect();

        check_marks(marks)
    }
}

pub fn run_a(_: i32, input: &Vec<String>) -> String {
    let mut iter = input.iter();

    let numbers = parse_numbers(iter.next().unwrap().as_str());
    let mut boards = vec![];

    while iter.next().is_some() {
        boards.push(parse_board(&mut iter));
    }

    let mut marked = HashSet::new();

    for number in numbers {
        marked.insert(number);

        let winners: Vec<&Board> = boards.iter()
                .filter(|board| board.wins(&marked))
                .collect();

        if !winners.is_empty() {
            let first = winners.first()
                .unwrap();

            return format!("{}", number * first.unmarked_sum(&marked));
        }
    }

    format!("I shouldn't get here")
}

pub fn run_b(_: i32, input: &Vec<String>) -> String {
    let mut iter = input.iter();

    let numbers = parse_numbers(iter.next().unwrap().as_str());
    let mut boards = vec![];

    while iter.next().is_some() {
        boards.push(parse_board(&mut iter));
    }

    let mut marked = HashSet::new();


    for number in numbers {
        marked.insert(number);
        let first = boards.first().unwrap();

        if boards.len() == 1 && first.wins(&marked) {
            info!("Last board found on number {}", number);

            return format!("{}", number * first.unmarked_sum(&marked));
        } else {
            boards.retain(|board| !board.wins(&marked));
        }
    }

    format!("I shouldn't get here")
}

fn parse_numbers(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn parse_board(iter: &mut Iter<String>) -> Board {
    let rows = vec![
        parse_line(iter.next().unwrap()),
        parse_line(iter.next().unwrap()),
        parse_line(iter.next().unwrap()),
        parse_line(iter.next().unwrap()),
        parse_line(iter.next().unwrap()),
    ];

    Board::new(rows)
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn check_marks(marks: Vec<Vec<bool>>) -> bool {
    let size = marks[0].len();
    let rows = marks.iter()
        .any(|row| row.len() == row.iter().filter(|b| **b).count());

    let columns = rows || (0..size).into_iter()
        .any(|i| marks.iter().all(|row| row[i]));

    columns
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_sample() -> Vec<String> {
        vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            "8  2 23  4 24",
            "21  9 14 16  7",
            "6 10  3 18  5",
            "1 12 20 15 19",
            "",
            "3 15  0  2 22",
            "9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            "2  0 12  3  7",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_parse_numbers() {
        let expected = vec![1,2,3];

        let actual = parse_numbers("1,2,3");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_line() {
        let expected = vec![22,13,17,11,0];
        let actual = parse_line("22 13 17 11  0");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_check_marks_row() {
        let marks = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![true, true, true, true, true],
        ];
        assert_eq!(true, check_marks(marks));
    }

    #[test]
    fn test_check_marks_column() {
        let marks = vec![
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
            vec![true, false, false, false, false],
        ];
        assert_eq!(true, check_marks(marks));
    }

    #[test]
    fn test_check_marks_diagonal() {
        let marks = vec![
            vec![true, false, false, false, false],
            vec![false, true, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, true, false],
            vec![false, false, false, false, true],
        ];
        assert_eq!(false, check_marks(marks));
    }

    #[test]
    fn test_sample_a() {
        init();
        let input = get_sample();

        let result = run_a(4, &input);

        assert_eq!("4512", result.as_str());
    }

    #[test]
    fn test_sample_b() {
        init();
        let input = get_sample();

        let result = run_b(4, &input);

        assert_eq!("1924", result.as_str());
    }
}