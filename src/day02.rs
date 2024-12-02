pub fn run_a(input: &str) -> i64 {
    let input = input.trim();

    input.lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|levels| is_safe(&levels))
        .count() as i64
}

fn is_safe(levels: &[i64]) -> bool {
    let mut increasing = None;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i-1];

        if diff > 3 || diff < -3 || diff == 0 {
            return false
        }

        match increasing {
            None => {
                increasing = Some(diff > 0);
            },
            Some(true) => {
                if diff < 0 {
                    return false;
                }
            },
            Some(false) => {
                if diff > 0 {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::day02::is_safe;

    fn input() -> &'static str {
        r"
        7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    }

    #[test]
    fn is_safe_all_decreasing() {
        assert!(is_safe(&[7,6,4,2,1]))
    }

    #[test]
    fn isnt_safe_large_infrease() {
        assert!(!is_safe(&[1,2,7,8,9]))
    }

    #[test]
    fn isnt_safe_large_decrease() {
        assert!(!is_safe(&[9,7,6,2,1]))
    }

    #[test]
    fn isnt_safe_mixed_increase_decrease() {
        assert!(!is_safe(&[1,3,2,4,5]))
    }

    #[test]
    fn isnt_safe_no_change() {
        assert!(!is_safe(&[8,6,4,4,1]))
    }

    #[test]
    fn is_safe_all_increasing() {
        assert!(is_safe(&[1,3,6,7,9]))
    }

    #[test]
    fn part_a_example() {
        assert_eq!(2, super::run_a(input()))
    }
}