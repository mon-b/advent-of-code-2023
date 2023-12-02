use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    
    let file: File = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut sum = 0;
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        
        if bytes_read == 0 
        {
            break;
        }


        let separated: std::str::Split<'_, &str> = line.split(":");
        let game_info: Vec<&str> = separated.collect::<Vec<&str>>();

        let _game_id: &str = game_info[0]; // Game xx need to get id and change to int
        let id_sep: std::str::Split<'_, &str> = _game_id.split(" "); 
        let game_id_str: String = id_sep.collect::<Vec<&str>>()[1].to_string();
        let _game_id: i32 = game_id_str.parse().unwrap(); // final game id

        let _game: &str = game_info[1]; // actual game stuff saved here
        let game_sep: std::str::Split<'_, &str> = _game.split(";");

        let mut _red: Vec<i32> = Vec::new();
        let mut _green: Vec<i32> = Vec::new();
        let mut _blue: Vec<i32> = Vec::new();

        for game in game_sep {
            let color_info: std::str::Split<'_, &str> = game.split(", ");

            for color in color_info{
                let num_color = color.trim().split(" ");
                let num_color_info = num_color.collect::<Vec<&str>>();

                let num_str = num_color_info[0];
                let num: String = num_str.to_string();
                let num_int: i32 = num.parse().unwrap();

                if num_color_info[1] == "red" 
                {
                    _red.push(num_int)
                }
                else if num_color_info[1] == "green" 
                {
                    _green.push(num_int)
                }
                else if num_color_info[1] == "blue" 
                {
                    _blue.push(num_int)
                }
            }
        }

        
        let max_red = _red.iter().cloned().max().unwrap_or_default();
        let max_green = _green.iter().cloned().max().unwrap_or_default();
        let max_blue = _blue.iter().cloned().max().unwrap_or_default();
        let multi: i32 = max_blue * max_green * max_red;
        sum += multi;

        line.clear();
    }
    println!("{}", sum);

    Ok(())
}
