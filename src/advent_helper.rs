use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

pub fn read_file_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();

    let f = File::open(filename).expect("Error opening file");

    let reader = BufReader::new(&f);

    for line in reader.lines() {
        lines.push(line.expect("Error reading line"));
    }

    lines
}

pub fn read_file_bytes(filename: &str) -> Vec<u8> {
    debug!("Reading bytes from {}", filename);
    let mut f = File::open(filename).expect("Error opening file");
    let mut buffer = Vec::new();

    let bytes_read = f.read_to_end(&mut buffer).expect("Error reading file");
    debug!("Read {} bytes", bytes_read);

    buffer
}