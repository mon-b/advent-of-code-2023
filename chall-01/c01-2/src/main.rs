use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file: File = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut word_as_digit: HashMap<&str, &str> = HashMap::new();

    word_as_digit.insert("one", "1");
    word_as_digit.insert("two", "2");
    word_as_digit.insert("three", "3");
    word_as_digit.insert("four", "4");
    word_as_digit.insert("five", "5");
    word_as_digit.insert("six", "6");
    word_as_digit.insert("seven", "7");
    word_as_digit.insert("eight", "8");
    word_as_digit.insert("nine", "9");

    let mut sum: i32 = 0;

    let mut line = String::new();
    let mut num_string: String = String::new(); 
    let mut curr_num: String = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let mut updated_line = line.clone(); // Clone the original line to work with

        for (word, digit) in &word_as_digit {
            updated_line = updated_line.replace(word, digit);

        }
        let mut num_string: String = String::new(); 
    let mut curr_num: String = String::new();

        for char in updated_line.chars() {
            if char.is_numeric() {
                num_string.push(char);
            }
        }

        let num_arr: Vec<char> = num_string.chars().collect();
        curr_num.push(num_arr[0]);
        curr_num.push(num_arr[num_arr.len() - 1]);
        let curr_num_int: i32 = curr_num.parse().unwrap();

        sum += curr_num_int;

        line.clear();
        updated_line.clear();
        num_string.clear();
        curr_num.clear();
    }

    println!("{}", sum);
    Ok(())
}
