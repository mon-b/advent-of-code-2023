use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut _points = 0;

    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }


        let sep = line.trim().split(":");
        let card_nums = sep.collect::<Vec<&str>>()[1];
        let sep_win = card_nums.trim().split("|");

        let mut winning_numbers: Vec<i32> = sep_win.clone().collect::<Vec<&str>>()[1]
            .split(" ")
            .map(|s| s.trim().parse::<i32>().unwrap_or_default())
            .collect();

        winning_numbers.retain(|&x| x != 0);

        let mut my_numbers: Vec<i32> = sep_win.clone().collect::<Vec<&str>>()[0]
            .split(" ")
            .map(|s| s.trim().parse::<i32>().unwrap_or_default())
            .collect();
        
        my_numbers.retain(|&x| x != 0);
        
        let total_matches = 
            my_numbers.iter()
            .filter(|&x| winning_numbers.contains(x))
            .count();

        if total_matches > 0 {
            // not having this was causing overflow because when 0 matches exponent would be -1
            println!("not 0");
            _points += i32::pow(2, total_matches as u32 - 1);
        }

        line.clear();
    }
    println!("{}", _points);
    Ok(())
}