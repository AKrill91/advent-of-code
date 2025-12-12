
fn parse(input: &str) -> i64 {
    0
}

pub async fn run_a(input: &str) -> i64 {
    0
}

pub async fn run_b(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn example() -> &'static str {
        r""
    }

    #[test]
    fn parse() {
        init();
        let parsed = super::parse(example());
    }

    #[tokio::test]
    async fn part_a_example() {
        init();
        assert_eq!(1, run_a(example()).await);
    }

    #[tokio::test]
    async fn part_b_example() {
        init();
        assert_eq!(1, run_b(example()).await);
    }
}