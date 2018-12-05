use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

pub fn read_file_lines(filename: &str) -> Result<Vec<String>, Error> {
    let mut lines = Vec::new();

    let f = File::open(filename);

    match f {
        Ok(file) => {
            let reader = BufReader::new(&file);

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        lines.push(l);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            Ok(lines)
        }
        Err(e) => Err(e)
    }
}

pub fn read_file_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("Error opening file");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer);

    buffer
}