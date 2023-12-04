use std::fs::File;
use std::io::{BufRead, BufReader};

struct Symbol {
    row: i32,
    col: i32
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    let mut map: Vec<Vec<&str>> = Vec::new();
    
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
    
        line.clear();
    }

    Ok(())
}