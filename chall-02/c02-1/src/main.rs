use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file: File = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        let separated: std::str::Split<'_, &str> = line.split(":");
        let game_info: Vec<&str> = separated.collect::<Vec<&str>>();

        let _game_id = game_info[0]; // Game xx need to get id and change to int
        // to get game id as int i can probably separate with space again and take the second one?

        let _game = game_info[1]; // actual game stuff saved here
        // should separate with ;
        // maybe after ; separate with space and make tuples then make another tuple to get all values for the same color??
        println!("{}", _game);


        line.clear();
    }

    let mut _red: [i32; 3] = [0, 0, 0];
    let mut _blue: [i32; 3] = [0, 0, 0];
    let mut _green: [i32; 3] = [0, 0, 0];

    Ok(())
}
