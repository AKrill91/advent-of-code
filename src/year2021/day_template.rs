
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


    #[test]
    fn test_sample_a() {
        init();
    }

    #[test]
    fn test_sample_b() {
        init();
    }
}