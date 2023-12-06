use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file: File = File::open("src/input.txt")?;
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut line = String::new();
    let mut times: Vec<i32> = Vec::new();
    let mut record: Vec<i32> = Vec::new();
    let mut _count: i32 = 0;

    loop {
        let bytes_read: usize = reader.read_line(&mut line)?;
        if bytes_read == 0 
        {
            break;
        }
        let sep: std::str::Split<'_, &str> = line.split(":").collect::<Vec<&str>>()[1].trim().split(" ");
        for num in sep {
            let num_parsed = num.parse::<i32>().unwrap_or_default();
            if _count == 0 {
                times.push(num_parsed)
            }
            else {
                record.push(num_parsed)
            }
        }

        _count += 1;
        line.clear();
    }
    times.retain(|&x| x != 0);
    record.retain(|&x| x != 0);

    let mut ok_times: i32 = 1;

    for i in 0..4
    {
        let time: i32 = times[i];
        let rec: i32 = record[i];
        let mut count = 0;

        for j in 0..time+1 
        {   
            let dist: i32 = j * (time - j);
        
            if dist > rec 
            {
                count += 1
            }
        }
        println!("{}", count);
        ok_times *= count

    }
    println!("{}", ok_times);



    

    Ok(())
}