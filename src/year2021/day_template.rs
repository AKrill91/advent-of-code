
pub fn run_a(_: i32, input: &Vec<String>) -> String {
    format!("")
}

pub fn run_b(_: i32, input: &Vec<String>) -> String {
    format!("")
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_sample() -> Vec<String> {
        vec![]
    }

    #[test]
    fn test_sample_a() {
        init();

        let input = get_sample();

        let expected = "";

        assert_eq!(expected, run_a(0, &input));
    }

    #[test]
    fn test_sample_b() {
        init();

        let input = get_sample();

        let expected = "";

        assert_eq!(expected, run_b(0, &input));
    }
}