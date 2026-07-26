use uuid::Uuid;

use crate::models::Card;
use crate::storage::{create_deck, load_deck, save_deck};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub fn import(path: String) {
    let file =
        File::open(path).expect(" Failed to read file (make sure you use a full, qualified path)");
    let reader = BufReader::new(file);
    let mut line_num = 0;

    for line in reader.lines() {
        line_num += 1;
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                println!("󰒭 Skipping line {line_num}: failed to read line");
                continue;
            }
        };

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 3 {
            println!("󰒭 Skipping line {line_num}: more or less than 3 columns");
            continue;
        }

        let deck = parts[0].trim();
        let front = parts[1].trim();
        let back = parts[2].trim();

        create_deck(deck.to_string());
        let (dx, _) = load_deck(deck.to_string() + ".json");
        let mut dx = match dx {
            Some(d) => d,
            None => {
                println!("Failed to load deck");
                exit(1);
            }
        };
        let card = Card {
            id: Uuid::new_v4(),
            front: front.to_string(),
            back: back.to_string(),
            state: 1,
            last_reviewed: None,
        };

        dx.cards.push(card);

        match save_deck(&dx) {
            true => (),
            false => {
                println!("Failed to save deck");
                exit(1);
            }
        }
    }

    println!("Import complete!")
}
