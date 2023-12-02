use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut num_string: String = String::new();
    let mut curr_num: String = String::new();
    let mut sum = 0;

    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        for char in line.chars() {
            if char.is_numeric() {
                num_string.push(char)
            }
        }

        let num_arr: Vec<char> = num_string.chars().collect();
        curr_num.push(num_arr[0]);
        curr_num.push(num_arr[num_arr.len() - 1]);
        let curr_num_int: i32 = curr_num.parse().unwrap();

        sum += curr_num_int;
        println!("{}", curr_num_int);

        line.clear();
        num_string.clear();
        curr_num.clear()
    }
    println!("{}", sum);
    Ok(())
}