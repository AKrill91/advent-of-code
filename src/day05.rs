use std::cmp::Ordering;

fn merge_ranges(ranges: Vec<IdRange>) -> Vec<IdRange> {
    let mut out = ranges;

    loop {
        let mut changed = false;
        let mut updated = Vec::new();

        for range in out.iter() {
            if updated.is_empty() {
                updated.push(*range);
                continue;
            }

            let mut merged = false;

            for compare in updated.iter_mut() {
                if let Some(result) = compare.merge(range) {
                    *compare = result;
                    changed = true;
                    merged = true;
                    break;
                }
            }

            if !merged {
                updated.push(*range);
            }
        }

        if !changed {
            break;
        } else {
            out = updated;
        }
    }

    out
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IdRange {
    start: i64,
    end: i64,
}

impl From<&str> for IdRange {
    fn from(input: &str) -> Self {
        let mut parts = input.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();
        IdRange { start, end }
    }
}

impl PartialEq<(i64, i64)> for IdRange {
    fn eq(&self, other: &(i64, i64)) -> bool {
        self.start == other.0 && self.end == other.1
    }
}

impl IdRange {
    fn new(start: i64, end: i64) -> Self {
        IdRange { start, end }
    }

    fn is_in_range(&self, id: i64) -> bool {
        id >= self.start && id <= self.end
    }

    fn length(&self) -> i64 {
        self.end - self.start + 1
    }

    fn merge(&self, other: &IdRange) -> Option<IdRange> {
        let merged = match other.start.cmp(&self.end) {
            Ordering::Less => {
                // If they start before we end
                match other.end.cmp(&self.start) {
                    Ordering::Equal | Ordering::Less => {
                        // If they end before we start, we can't do anything unless they're off by 1
                        if other.end + 1 >= self.start {
                            Some(IdRange {
                                start: i64::min(self.start, other.start),
                                end: i64::max(self.end, other.end),
                            })
                        } else {
                            None
                        }
                    }
                    Ordering::Greater => {
                        //If they also end after we start we fully overlap
                        Some(IdRange {
                            start: i64::min(self.start, other.start),
                            end: i64::max(self.end, other.end),
                        })
                    }
                }
            },
            Ordering::Equal | Ordering::Greater => {
                //If they start when we end, or immediately after, we can merge
                if other.start <= self.end + 1 {
                    Some(IdRange {
                        start: i64::min(self.start, other.start),
                        end: i64::max(self.end, other.end),
                    })
                } else {
                    None
                }
            }
        };

        if merged.is_some() && log::log_enabled!(log::Level::Debug) {
            log::debug!(" Merged ranges {:?} and {:?} into {:?}", self, other, merged);
        }

        merged
    }

}

struct Inventory {
    fresh_id_ranges: Vec<IdRange>,
    available_ids: Vec<i64>,
}

impl From<&str> for Inventory {
    fn from(input: &str) -> Self {
        let mut sections = input.trim().split("\n\n");
        let fresh_id_ranges = sections
            .next()
            .unwrap()
            .lines()
            .map(IdRange::from)
            .collect();

        let available_ids = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        Inventory {
            fresh_id_ranges: merge_ranges(fresh_id_ranges),
            available_ids,
        }
    }
}

impl Inventory {
    fn is_id_fresh(&self, id: i64) -> bool {
        self.fresh_id_ranges.iter().any(|range| range.is_in_range(id))
    }

    fn fresh_count(&self) -> i64 {
        self.available_ids.iter().filter(|&&id| self.is_id_fresh(id)).count() as i64
    }

    fn num_fresh_ids(&self) -> i64 {
        self.fresh_id_ranges
            .iter()
            .map(IdRange::length)
            .sum()
    }
}

fn parse(input: &str) -> Inventory {
    Inventory::from(input)
}
pub fn run_a(input: &str) -> i64 {
    parse(input)
        .fresh_count()
}

pub fn run_b(input: &str) -> i64 {
    let inventory = parse(input);

    inventory
        .num_fresh_ids()
}

#[cfg(test)]
mod test {
    use crate::day05::IdRange;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());

        assert_eq!(parsed.fresh_id_ranges.len(), 2);
        assert_eq!(parsed.fresh_id_ranges[1], (10, 20));
    }

    #[test]
    fn part_a_example() {
        init();
        assert_eq!(3, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        init();
        assert_eq!(14, super::run_b(example()));
    }


    mod id_range {
        use crate::day05::IdRange;

        #[test]
        fn sequential() {
            let first = IdRange::new(1, 5);
            let second = IdRange::new(6, 10);

            let merged = first.merge(&second).unwrap();

            assert_eq!(merged, (1, 10))
        }

        #[test]
        fn overlap_start() {
            let first = IdRange::new(5, 10);
            let second = IdRange::new(1, 6);
            let merged = first.merge(&second).unwrap();

            assert_eq!(merged, (1, 10))
        }

        #[test]
        fn overlap_end() {
            let first = IdRange::new(1, 6);
            let second = IdRange::new(5, 10);
            let merged = first.merge(&second).unwrap();

            assert_eq!(merged, (1, 10))
        }

        #[test]
        fn encompassed() {
            let first = IdRange::new(1, 10);
            let second = IdRange::new(2, 6);
            let merged = first.merge(&second).unwrap();

            assert_eq!(merged, (1, 10))
        }

        #[test]
        fn separate() {
            let first = IdRange::new(1, 5);
            let second = IdRange::new(7, 10);
            let merged = first.merge(&second);

            assert!(merged.is_none());
        }
    }

    #[test]
    fn flatten_ranges_contiguous() {
        init();
        let ranges = vec![IdRange::new(1, 5), IdRange::new(6, 10), IdRange::new(12, 15)];
        let merged = super::merge_ranges(ranges);
        log::debug!("{:?}", merged);

        assert_eq!(2, merged.len());
        assert_eq!(merged[0], (1, 10));
    }
}