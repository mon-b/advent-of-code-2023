use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let file: File = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut word_as_digit: HashMap<&str, &str> = HashMap::new();

    // having hashmap like this makes the algorithm work buuut i did not come up with it!
    // maybe will redo at some point to have my own solution
    // if you leave "one" and "two" twone -like cases do not replace correctly
    word_as_digit.insert("one", "o1e");
    word_as_digit.insert("two", "t2o");
    word_as_digit.insert("three", "th3ee");
    word_as_digit.insert("four", "f4ur");
    word_as_digit.insert("five", "fi5e");
    word_as_digit.insert("six", "s6x");
    word_as_digit.insert("seven", "sev7n");
    word_as_digit.insert("eight", "ei8ht");
    word_as_digit.insert("nine", "ni9e");

    let mut sum: i32 = 0;

    let mut line = String::new();
    let mut _num_string: String = String::new(); 
    let mut _curr_num: String = String::new();

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