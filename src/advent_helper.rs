use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn read_file_lines(filename: &str) -> Vec<String> {
    debug!("Reading file {}", filename);
    let mut lines = Vec::new();

    let f = File::open(filename).expect("Error opening file");

    let reader = BufReader::new(&f);

    for line in reader.lines() {
        lines.push(line.expect("Error reading line"));
    }

    lines
}

pub fn _read_file_bytes(filename: &str) -> Vec<u8> {
    debug!("Reading bytes from {}", filename);
    let mut f = File::open(filename).expect("Error opening file");
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).expect("Error reading file");
    debug!("Read {} bytes", bytes_read);

    buffer
}

#[allow(dead_code)]
pub fn gcd(a: i32, b: i32) -> i32 {
    let mut remainder;
    let mut left = a;
    let mut right = b;

    while left != 0 {
        remainder = right % left;
        right = left;
        left = remainder;
    }

    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_gcd_0() {
        assert_eq!(3, gcd(3, 6));
    }

    #[test]
    pub fn test_gcd_1() {
        assert_eq!(1, gcd(3, 7));
    }

    #[test]
    pub fn test_gcd_2() {
        assert_eq!(5, gcd(5, 25));
    }
}