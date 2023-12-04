use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    card_id: i32,
    copies: i32,
    winning_entries: i32
}

impl Card {
    fn new(card_id: i32, winning_entries: i32) -> Self {
        // all cards start with 1 copy
        Card {
            card_id,
            copies: 1,
            winning_entries,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut cards: Vec<Card> = <Vec<Card>>::new();
    let mut _id_count = 1;

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

        let mut card_instance = Card::new(_id_count, total_matches as i32);
        cards.push(card_instance);

        line.clear();
        _id_count += 1
    }
    for i in 0..cards.len() {
        let multiplier = cards[i].copies;
        for x in 1..(cards[i].winning_entries+1) {
            cards[x as usize + i].copies += 1 * multiplier;
        }
    }
    let mut card_count = 0;
    for card in &cards {
        card_count += card.copies
    }
    println!("Total of cards: {}", card_count);
    Ok(())
}