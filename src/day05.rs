use std::ops::Not;
use crate::utils::Direction;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

type Page = u32;

struct OrderingRule {
    left: Page,
    right: Page
}

impl OrderingRule {
    fn is_relevant(&self, update: &PageUpdate) -> bool {
        update.pages.contains(&self.left) && update.pages.contains(&self.right)
    }

    fn is_followed(&self, update: &PageUpdate) -> bool {
        let mut left_index = usize::MAX;
        let mut right_index = usize::MIN;

        for i in 0..update.pages.len() {
            let page = update.pages[i];

            if page == self.left {
                left_index = i;
            } else if page == self.right {
                right_index = i;
            }
        }

        left_index < right_index
    }
}

struct PageUpdate {
    pages: Vec<Page>
}

impl PageUpdate {
    fn middle_page(&self) -> &Page {
        assert_eq!(self.pages.len() % 2, 1);
        let half = self.pages.len() / 2;

        &self.pages[half]
    }

    fn fix(&mut self, rules: Vec<&OrderingRule>) {
        let count = self.pages.len();
        let mut tmp: Vec<Page> = vec![0; count];
        std::mem::swap(&mut tmp, &mut self.pages);

        for i in 0..count {
            let (index, smallest) = tmp.iter()
                .enumerate()
                .find(|(_, p)| rules.iter().filter(|r| !self.pages.contains(&r.left) && !self.pages.contains(&r.right))
                .all(|r| r.right.ne(p)))
                .unwrap();

            self.pages[i] = *smallest;
            tmp.remove(index);
        }
    }
}

fn parse(input: &str) -> (Vec<OrderingRule>, Vec<PageUpdate>) {
    let mut rules = vec![];
    let mut updates = vec![];

    input.lines().for_each(|line| {
        if line.contains('|') {
            let mut parts = line.split('|');
            let left = parts.next().unwrap().parse::<Page>().unwrap();
            let right = parts.next().unwrap().parse::<Page>().unwrap();

            rules.push(OrderingRule { left, right });
        } else if line.contains(',') {
            let mut pages = vec![];

            for part in line.split(',') {
                pages.push(part.parse::<Page>().unwrap());
            }

            updates.push(PageUpdate{ pages });
        }
    });

    (rules, updates)
}

pub fn run_a(input: &str) -> i64 {
    let (rules, updates) = parse(input);

    updates.iter().filter(|update| {
        rules.iter().filter(|r| r.is_relevant(update)).all(|r| r.is_followed(update))
    })
        .map(PageUpdate::middle_page)
        .cloned()
        .sum::<u32>() as i64
}

pub fn run_b(input: &str) -> i64 {
    let (rules, mut updates) = parse(input);

    updates.iter_mut()
        .filter(|update| {
            rules.iter().filter(|r| r.is_relevant(update))
                .all(|r| r.is_followed(update))
                .not()
        })
        .map(|update| {
            let relevant_rules = rules.iter().filter(|r| r.is_relevant(update))
                .collect::<Vec<_>>();

            update.fix(relevant_rules);
            &*update
        })
        .map(PageUpdate::middle_page)
        .cloned()
        .sum::<u32>() as i64

}

#[cfg(test)]
mod test {
    use crate::day05::PageUpdate;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
    }


    #[test]
    fn part_a_example() {
        init();
        assert_eq!(143, super::run_a(example()));
    }

    #[test]
    fn part_b_example() {
        assert_eq!(123, super::run_b(example()));
    }

    #[test]
    fn parse() {
        let (rules, updates) = super::parse(example());

        assert_eq!(21, rules.len());
        assert_eq!(6, updates.len());
    }

    #[test]
    fn middle_page() {
        assert_eq!(2, *PageUpdate { pages: vec![1, 2, 3]}.middle_page());
        assert_eq!(3, *PageUpdate { pages: vec![1, 2, 3, 4, 5]}.middle_page());
    }
}