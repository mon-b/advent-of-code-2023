use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file: File = File::open("src/input.txt")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line = String::new();
    let mut times: Vec<i64> = Vec::new();
    let mut record: Vec<i64> = Vec::new();
    let mut num_string: String = String::new();
    let mut _count: i64 = 0;

    loop {
        let bytes_read: usize = reader.read_line(&mut line)?;
        if bytes_read == 0 
        {
            break;
        }
        for char in line.chars() {
            if char.is_numeric() {
                num_string.push(char);
            }
        }
        let num = num_string.parse::<i64>().unwrap();
        if _count == 0 
        {
            times.push(num)
        }
        else 
        {
            record.push(num)
        }
        

        _count += 1;
        num_string.clear();
        line.clear();
    }

    for i in 0..1
    {
        let time: i64 = times[i];
        let rec: i64 = record[i];
        let mut count = 0;

        for j in 0..time+1 
        {   
            let dist: i64 = j * (time - j);
        
            if dist > rec 
            {
                count += 1
            }
        }
        println!("Total ways: {}", count);

    }

    Ok(())
}